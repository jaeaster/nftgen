// Credit to BurntSushi and RipGrep. Original source can be found at
// https://github.com/BurntSushi/ripgrep/blob/master/crates/core/config.rs
// Explanation of this method of combining of config file w/ cli args can be found at
// https://www.reddit.com/r/rust/comments/d038gj/comment/ez90rk7/?utm_source=share&utm_medium=web2x&context=3

// This module provides routines for reading config files. The
// primary output of these routines is a sequence of arguments, where each
// argument corresponds precisely to one shell argument.

use color_eyre::eyre;
use std::env;
use std::error;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use bstr::{io::BufReadExt, ByteSlice};
use log;

// type Result<T> = std::result::Result<T, std::io::Error>;
type Result<T> = ::std::result::Result<T, Box<dyn error::Error>>;

type Args = Vec<OsString>;
type Errors = Vec<Box<dyn Error>>;

/// Return a sequence of arguments derived from ripgrep rc configuration files.
pub fn args() -> eyre::Result<Vec<OsString>> {
    let config_path = match env::var_os("NFTGEN_CONFIG_PATH") {
        None => return Ok(vec![]),
        Some(config_path) => {
            if config_path.is_empty() {
                return Ok(vec![]);
            }
            PathBuf::from(config_path)
        }
    };
    let (args, errs) = match parse(&config_path) {
        Ok((args, errs)) => (args, errs),
        Err(err) => {
            eyre::bail!(
                "failed to read the file specified in NFTGEN_CONFIG_PATH: {}",
                err
            );
        }
    };

    if let Some(err) = errs.into_iter().next() {
        eyre::bail!("{}:{}", config_path.display(), err);
    }

    log::debug!(
        "{}: arguments loaded from config file: {:?}",
        config_path.display(),
        args
    );
    Ok(args)
}

/// Parse a single ripgrep rc file from the given path.
///
/// On success, this returns a set of shell arguments, in order, that should
/// be pre-pended to the arguments given to ripgrep at the command line.
///
/// If the file could not be read, then an error is returned. If there was
/// a problem parsing one or more lines in the file, then errors are returned
/// for each line in addition to successfully parsed arguments.
fn parse<P: AsRef<Path>>(path: P) -> Result<(Args, Errors)> {
    let path = path.as_ref();
    match File::open(path) {
        Ok(file) => parse_reader(file),
        Err(err) => Err(From::from(format!("{}: {}", path.display(), err))),
    }
}

/// Parse a single ripgrep rc file from the given reader.
///
/// Callers should not provided a buffered reader, as this routine will use its
/// own buffer internally.
///
/// On success, this returns a set of shell arguments, in order, that should
/// be pre-pended to the arguments given to ripgrep at the command line.
///
/// If the reader could not be read, then an error is returned. If there was a
/// problem parsing one or more lines, then errors are returned for each line
/// in addition to successfully parsed arguments.
fn parse_reader<R: io::Read>(rdr: R) -> Result<(Args, Errors)> {
    let bufrdr = io::BufReader::new(rdr);
    let (mut args, mut errs) = (vec![], vec![]);
    let mut line_number = 0;
    bufrdr.for_byte_line_with_terminator(|line| {
        line_number += 1;

        let line = line.trim();
        if line.is_empty() || line[0] == b'#' {
            return Ok(true);
        }
        match line.to_os_str() {
            Ok(osstr) => {
                args.push(osstr.to_os_string());
            }
            Err(err) => {
                errs.push(format!("{}: {}", line_number, err).into());
            }
        }
        Ok(true)
    })?;
    Ok((args, errs))
}

#[cfg(test)]
mod tests {
    use super::parse_reader;
    use std::ffi::OsString;

    #[test]
    fn basic() {
        let (args, errs) = parse_reader(
            &b"\
# Test
--context=0
   --smart-case
-u


   # --bar
--foo
"[..],
        )
        .unwrap();
        assert!(errs.is_empty());
        let args: Vec<String> = args.into_iter().map(|s| s.into_string().unwrap()).collect();
        assert_eq!(args, vec!["--context=0", "--smart-case", "-u", "--foo",]);
    }

    // We test that we can handle invalid UTF-8 on Unix-like systems.
    #[test]
    #[cfg(unix)]
    fn error() {
        use std::os::unix::ffi::OsStringExt;

        let (args, errs) = parse_reader(
            &b"\
quux
foo\xFFbar
baz
"[..],
        )
        .unwrap();
        assert!(errs.is_empty());
        assert_eq!(
            args,
            vec![
                OsString::from("quux"),
                OsString::from_vec(b"foo\xFFbar".to_vec()),
                OsString::from("baz"),
            ]
        );
    }

    // ... but test that invalid UTF-8 fails on Windows.
    #[test]
    #[cfg(not(unix))]
    fn error() {
        let (args, errs) = parse_reader(
            &b"\
quux
foo\xFFbar
baz
"[..],
        )
        .unwrap();
        assert_eq!(errs.len(), 1);
        assert_eq!(args, vec![OsString::from("quux"), OsString::from("baz"),]);
    }
}
