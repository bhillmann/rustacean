use structopt::StructOpt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    // let s = std::fs::read_to_string(&args.path);
    let f = File::open(&args.path);

    let mut reader = BufReader::new(f?);

    for line in reader.lines() {
        // The question mark operator is a new thing to me, it seems to unpack a results object
        // https://stackoverflow.com/questions/42917566/what-is-this-question-mark-operator-about
        println!("Line is {}", line?);
    }

    Ok(())
}
