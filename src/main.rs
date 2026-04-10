use std::{
    fs::File, io::Write, thread::sleep, time::{Duration, Instant}
};

use clap::Parser;
use eyre::{Context, bail};
use probe_rs::{Permissions, probe::list::Lister, rtt::Rtt};

use crate::cli::Cli;

mod cli;

fn main() -> eyre::Result<()> {
    let Cli {
        len,
        out,
        channel,
        chip,
        protocol,
        timeout_sec,
    } = Cli::parse();
    // First obtain a probe-rs session (see probe-rs documentation for details)
    let lister = Lister::new();

    let probes = lister.list_all();

    let probe = probes[0].open()?;
    let mut session = probe.attach(chip, Permissions::default())?;
    // println!("session: {session:?}");
    // Select a core.
    let mut core = session.core(0)?;

    // Attach to RTT
    let mut rtt = Rtt::attach(&mut core)?;

    // Read from a channel
    if let Some(input) = rtt.up_channel(channel) {
        println!("got channel: {input:?}");
        let mut f = File::create(&out).wrap_err_with(|| format!("open: {out:?}"))?;
        let mut buf = [0u8; 128];
        let mut left = len.unwrap_or(usize::MAX);
        let mut total = 0;
        let end = Instant::now() + Duration::from_secs(timeout_sec);
        while left > 0 && Instant::now() < end {
            let to_read = left.min(buf.len());
            let count = input.read(&mut core, &mut buf[..to_read])?;
            if count > 0 {
                // println!("read {count} bytes");
            } else {
                sleep(Duration::from_millis(3));
            }
            let read = &buf[..count];
            left -= count;
            total += count;
            f.write_all(read)?;
        }
        println!("read {total} bytes");
    } else {
        bail!("no such channel: {channel}");
    }
    Ok(())
}
