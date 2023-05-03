use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let mut pargs = pico_args::Arguments::from_env();
    if pargs.contains(["-h", "--help"]) {
        println!(
            "Usage: {} --from=<offset> --to=<offset> <filename>",
            env!("CARGO_BIN_NAME")
        );
        return Ok(());
    }

    if pargs.contains(["-v", "--version"]) {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let start: u64 = pargs.value_from_str(["-f", "--from"])?;
    let endx: u64 = pargs.value_from_str(["-t", "--to"])?;
    let filename: PathBuf = pargs.free_from_str()?;

    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    reader.seek(SeekFrom::Start(start))?;
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = BufWriter::new(stdout);
    let mut buf: [u8; 4096] = [0; 4096];
    let mut remaining: usize = endx.saturating_sub(start) as usize;
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        } else if n > remaining {
            stdout.write_all(&mut buf[..remaining])?;
        } else {
            stdout.write_all(&mut buf[..n])?;
            remaining -= n;
        }
    }

    return Ok(());
}
