use std::path::PathBuf;

use clap::Parser;
use probe_rs::probe::WireProtocol;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub len: Option<usize>,
    #[clap(short, long, default_value = "30")]
    pub timeout_sec: u64,
    pub out: PathBuf,
    #[clap(short='n', long, default_value="0")]
    pub channel: usize,
    #[clap(short, long, env = "PROBE_RS_CHIP")]
    pub chip: String,
    #[clap(long, default_value="swd")]
    pub protocol: WireProtocol
}
