use std::path::PathBuf;

use mizer::{build_runtime, Flags};
use test_case::test_case;

#[test_case("artnet"; "artnet")]
#[test_case("fixture"; "fixture")]
#[test_case("laser"; "laser")]
#[test_case("pixels"; "pixels")]
#[test_case("sacn"; "sacn")]
#[test_case("sequence"; "sequence")]
#[test_case("video"; "video")]
#[test_case("inputs"; "inputs")]
#[test_case("osc"; "osc")]
#[tokio::test]
async fn test_build_project_pipeline(project: &str) {
    let handle = tokio::runtime::Handle::current();
    let flags = Flags {
        join: false,
        generate_graph: false,
        file: Some(PathBuf::from(format!("examples/{}.yml", project))),
        disable_media_api: true,
        disable_grpc_api: true,
        headless: true,
    };

    build_runtime(handle, flags).unwrap();
}
