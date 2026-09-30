use std::{
    fs::File,
    io::Write,
    thread::sleep,
    time::{Duration, Instant},
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
        mut stdout,
        channel,
        chip,
        protocol,
        timeout_sec,
    } = Cli::parse();
    if out.is_none() {
        stdout = false;
    }
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
    let up_channel = {
        let ch = rtt
            .up_channels()
            .iter_mut()
            .find(|ch| ch.name() == Some(channel.as_str()));
        if let Some(ch) = ch {
            Some(ch)
        } else {
            let num: usize = channel.parse().unwrap();
            rtt.up_channel(num)
        }
    };

    // Read from a channel
    if let Some(input) = up_channel {
        println!("got channel: {input:?}");
        let mut f;
        let mut s;
        let f = if let Some(out) = out {
            f = File::create(&out).wrap_err_with(|| format!("open: {out:?}"))?;
            &mut f as &mut dyn Write
        } else {
            s = std::io::stdout();
            &mut s as &mut dyn Write
        };
        let mut buf = [0u8; 128];
        let mut left = len.unwrap_or(usize::MAX);
        println!("will read at most {}bytes", left);
        let mut total = 0;
        let end = timeout_sec
            .map(|timeout_sec| Instant::now() + Duration::from_secs(timeout_sec));
        while left > 0 && end.map(|end| Instant::now() < end).unwrap_or(true) {
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
            if stdout {
                std::io::stdout().write_all(read).unwrap();
            }
        }
        println!("read {total} bytes");
    } else {
        bail!("no such channel: {channel}");
    }
    Ok(())
}
