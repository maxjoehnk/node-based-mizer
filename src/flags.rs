use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub struct Flags {
    #[structopt(long)]
    pub generate_graph: bool,
    #[structopt(name = "FILE", parse(from_os_str))]
    pub file: Option<PathBuf>,
    #[cfg(feature = "export_metrics")]
    #[structopt(long)]
    pub metrics: bool,
    #[cfg(feature = "export_metrics")]
    #[structopt(long, default_value = "8888")]
    pub metrics_port: u16,
    /// Join an existing session or start a new session
    #[structopt(long)]
    pub join: bool,
    #[structopt(long)]
    pub disable_media_api: bool,
    #[structopt(long)]
    pub disable_grpc_api: bool,
    #[cfg(feature = "ui")]
    #[structopt(long)]
    pub headless: bool,
}
