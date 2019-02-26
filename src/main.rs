extern crate libc;
extern crate structopt;

use std::ffi::CString;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use structopt::StructOpt;

fn pathbuf_to_cstring(x: &Path) -> io::Result<CString> {
    let canonicalized = x.canonicalize()?;
    let canon_str = canonicalized.as_os_str().as_bytes();

    Ok(CString::new(canon_str)?)
}

#[derive(StructOpt, Debug)]
#[structopt(name = "swap")]
struct Flags {
    /// Source
    #[structopt(name = "SRC", parse(from_os_str))]
    src: PathBuf,

    /// Destination
    #[structopt(name = "DST", parse(from_os_str))]
    dst: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Flags::from_args();

    let src = pathbuf_to_cstring(args.src.as_ref())?;
    let dst = pathbuf_to_cstring(args.dst.as_ref())?;

    let result = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            0,
            src.as_ptr(),
            0,
            dst.as_ptr(),
            libc::RENAME_EXCHANGE
        )
    };

    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
