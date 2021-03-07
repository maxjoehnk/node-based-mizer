use std::time::Duration;

use mizer_project_files::Project;

pub use crate::flags::Flags;
use anyhow::Context;
use mizer_devices::DeviceModule;
use mizer_fixtures::library::{FixtureLibrary, FixtureLibraryProvider};
use mizer_fixtures::manager::FixtureManager;
use mizer_fixtures::FixtureModule;
use mizer_media::{MediaDiscovery, MediaServer};
use mizer_module::{Module, Runtime};
use mizer_open_fixture_library_provider::OpenFixtureLibraryProvider;
use mizer_protocol_dmx::*;
use mizer_runtime::DefaultRuntime;
use mizer_media::api::MediaServerApi;

mod flags;

const FRAME_DELAY_60FPS: Duration = Duration::from_millis(16);

pub async fn build_runtime(flags: Flags) -> anyhow::Result<Mizer> {
    let handle = tokio::runtime::Handle::try_current()?;
    let mut runtime = DefaultRuntime::new();

    register_device_module(&mut runtime, &handle)?;
    register_dmx_module(&mut runtime)?;
    let fixture_manager = register_fixtures_module(&mut runtime)?;

    let mut media_paths = Vec::new();
    load_project_files(&flags, &mut runtime, &mut media_paths)?;

    if flags.generate_graph {
        runtime.generate_pipeline_graph()?;
    }

    let media_server = MediaServer::new().await?;
    let media_server_api = media_server.open_api(&handle)?;
    import_media_files(&media_paths, &media_server_api)?;

    let grpc = setup_grpc_api(&flags, handle, &mut runtime, fixture_manager, &media_server_api)?;
    setup_media_api(flags, media_server_api)?;

    Ok(Mizer { runtime, grpc })
}

pub struct Mizer {
    runtime: DefaultRuntime,
    #[allow(dead_code)]
    grpc: Option<mizer_grpc_api::Server>,
}

impl Mizer {
    pub async fn run(&mut self) {
        loop {
            let before = std::time::Instant::now();
            self.runtime.process();
            let after = std::time::Instant::now();
            let frame_time = after.duration_since(before);
            metrics::histogram!("mizer.frame_time", frame_time);
            if frame_time <= FRAME_DELAY_60FPS {
                tokio::time::delay_for(FRAME_DELAY_60FPS - frame_time).await;
            }
        }
    }
}

fn register_device_module(runtime: &mut DefaultRuntime, handle: &tokio::runtime::Handle) -> anyhow::Result<()> {
    let (device_module, device_manager) = DeviceModule::new();
    handle.spawn(device_manager.clone().start_discovery());
    device_module.register(runtime)?;

    Ok(())
}

fn register_dmx_module(runtime: &mut DefaultRuntime) -> anyhow::Result<()> {
    DmxModule.register(runtime)
}

fn register_fixtures_module(runtime: &mut DefaultRuntime) -> anyhow::Result<FixtureManager> {
    let ofl_provider = load_ofl_provider()?;
    let providers: Vec<Box<dyn FixtureLibraryProvider>> = vec![Box::new(ofl_provider)];

    let (fixture_module, fixture_manager) = FixtureModule::new(providers);
    fixture_module.register(runtime)?;

    Ok(fixture_manager)
}

fn load_ofl_provider() -> anyhow::Result<OpenFixtureLibraryProvider> {
    log::info!("Loading open fixture library...");
    let mut ofl_provider = OpenFixtureLibraryProvider::new();
    ofl_provider
        .load("components/fixtures/open-fixture-library/.fixtures.json")
        .context("loading open fixture library")?;
    log::info!("Loading open fixture library...Done");

    Ok(ofl_provider)
}

fn load_project_files(flags: &Flags, runtime: &mut DefaultRuntime, media_paths: &mut Vec<String>) -> anyhow::Result<()> {
    log::info!("Loading projects...");
    for file in &flags.files {
        let project = Project::load_file(&file)?;
        media_paths.extend(project.media_paths.clone());
        {
            let injector = runtime.injector();
            let manager = injector.get().unwrap();
            let library = injector.get().unwrap();
            load_fixtures(manager, library, &project);
        }
        runtime.load_project(project).context("loading project")?;
    }
    log::info!("Loading projects...Done");

    Ok(())
}

fn load_fixtures(fixture_manager: &FixtureManager, library: &FixtureLibrary, project: &Project) {
    for fixture in &project.fixtures {
        let def = library.get_definition(&fixture.fixture);
        if let Some(def) = def {
            fixture_manager.add_fixture(
                fixture.id.clone(),
                def,
                fixture.mode.clone(),
                fixture.output.clone(),
                fixture.channel,
                fixture.universe,
            );
        } else {
            log::warn!("No fixture definition for fixture id {}", fixture.fixture);
        }
    }
}

fn import_media_files(media_paths: &[String], media_server_api: &MediaServerApi) -> anyhow::Result<()> {
    let handle = tokio::runtime::Handle::try_current()?;
    for path in media_paths {
        let media_discovery = MediaDiscovery::new(media_server_api.clone(), path);
        handle.spawn(async move { media_discovery.discover().await });
    }
    Ok(())
}

fn setup_grpc_api(flags: &Flags, handle: tokio::runtime::Handle, runtime: &mut DefaultRuntime, fixture_manager: FixtureManager, media_server_api: &MediaServerApi) -> anyhow::Result<Option<mizer_grpc_api::Server>> {
    let grpc = if !flags.disable_grpc_api {
        Some(mizer_grpc_api::start(
            handle.clone(),
            runtime.api(),
            fixture_manager,
            media_server_api.clone(),
        )?)
    } else {
        None
    };
    Ok(grpc)
}

fn setup_media_api(flags: Flags, media_server_api: MediaServerApi) -> anyhow::Result<()> {
    if !flags.disable_media_api {
        mizer_media::http_api::start(media_server_api)?;
    }
    Ok(())
}
