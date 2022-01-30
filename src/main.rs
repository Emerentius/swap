extern crate libc;
extern crate structopt;

use std::ffi::{CString, NulError};
use std::io;
use std::os::unix::prelude::OsStringExt;
use std::path::PathBuf;

use structopt::StructOpt;

fn pathbuf_to_cstring(path: PathBuf) -> Result<CString, NulError> {
    CString::new(path.into_os_string().into_vec())
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

    let src = pathbuf_to_cstring(args.src)?;
    let dst = pathbuf_to_cstring(args.dst)?;

    let result = unsafe {
        libc::syscall(
            // See `man renameat2`
            // or https://man7.org/linux/man-pages/man2/renameat.2.html
            libc::SYS_renameat2,
            // From manual:
            // If oldpath is relative and olddirfd is the special value
            // AT_FDCWD, then oldpath is interpreted relative to the current
            // working directory of the calling process.
            // newdirfd has the same behavior for newpath.
            libc::AT_FDCWD,        // olddirfd
            src.as_ptr(),          // oldpath
            libc::AT_FDCWD,        // newdirfd
            dst.as_ptr(),          // newpath
            libc::RENAME_EXCHANGE, // flags
        )
    };

    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
