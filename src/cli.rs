use std::path::PathBuf;

use clap::Parser;
use probe_rs::probe::WireProtocol;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub len: Option<usize>,
    #[clap(short, long)]
    pub timeout_sec: Option<u64>,
    /// Also print to stdout
    #[clap(short, long)]
    pub stdout: bool,
    pub out: Option<PathBuf>,
    #[clap(short='n', long, default_value="0")]
    pub channel: String,
    #[clap(short, long, env = "PROBE_RS_CHIP")]
    pub chip: String,
    #[clap(long, default_value="swd")]
    pub protocol: WireProtocol
}
