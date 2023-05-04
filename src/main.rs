use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::num::ParseIntError;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let mut pargs = pico_args::Arguments::from_env();
    if pargs.contains(["-h", "--help"]) {
        println!(
            "Usage: {} <filename> <start offset> <end offset>",
            env!("CARGO_BIN_NAME")
        );
        return Ok(());
    }

    if pargs.contains(["-v", "--version"]) {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let filename: PathBuf = pargs.free_from_str()?;
    let start: u64 = pargs.free_from_fn(hex_or_dec)?;
    let endx: u64 = pargs.free_from_fn(hex_or_dec)?;

    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = BufWriter::new(stdout);

    copy_slice(&mut reader, &mut stdout, start, endx)?;

    return Ok(());
}

fn hex_or_dec(s: &str) -> Result<u64, ParseIntError> {
    if s.starts_with("0x") || s.starts_with("0X") {
        u64::from_str_radix(&s[2..], 16)
    } else {
        u64::from_str_radix(s, 10)
    }
}

fn copy_slice<R: Read + Seek, W: Write>(
    src: &mut R,
    dst: &mut W,
    start: u64,
    endx: u64,
) -> io::Result<()> {
    src.seek(SeekFrom::Start(start))?;
    let mut buf: [u8; 4096] = [0; 4096];
    let mut remaining: usize = endx.saturating_sub(start) as usize;
    loop {
        let n = src.read(&mut buf)?;
        if n == 0 {
            break;
        }
        let upto = usize::min(remaining, n);
        dst.write_all(&mut buf[..upto])?;
        remaining -= upto;
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::copy_slice;
    use std::io::Cursor;

    #[test]
    fn test_copy_slice() {
        let mut r = Cursor::new(b"hello".to_vec());
        let mut w = Vec::new();

        copy_slice(&mut r, &mut w, 0, 5).unwrap();
        assert_eq!(w, b"hello");

        w.clear();
        copy_slice(&mut r, &mut w, 0, 4).unwrap();
        assert_eq!(w, b"hell");

        w.clear();
        copy_slice(&mut r, &mut w, 1, 4).unwrap();
        assert_eq!(w, b"ell");

        w.clear();
        copy_slice(&mut r, &mut w, 1, 2).unwrap();
        assert_eq!(w, b"e");

        w.clear();
        copy_slice(&mut r, &mut w, 1, 1).unwrap();
        assert_eq!(w, b"");

        w.clear();
        copy_slice(&mut r, &mut w, 1, 0).unwrap();
        assert_eq!(w, b"");

        w.clear();
        copy_slice(&mut r, &mut w, 0, 6).unwrap();
        assert_eq!(w, b"hello");

        w.clear();
        copy_slice(&mut r, &mut w, 8, 10).unwrap();
        assert_eq!(w, b"");
    }
}
