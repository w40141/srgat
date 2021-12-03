use std::path::PathBuf;
use structopt::StructOpt;

/// Manage tags written in comments in files
#[derive(StructOpt, Debug)]
#[structopt(name = "srgat")]
struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
    /// Pring tags in the files
    #[structopt(short, long, parse(from_os_str))]
    files: Vec<PathBuf>,
    /// Print tags in the directory
    #[structopt(short, long)]
    recursively: String,
    /// Ignore the files
    #[structopt(short, long)]
    ignore: String,
    /// Print the all saved tags in default file or target file
    #[structopt(short, long)]
    show: String,
    /// Save the tags in the files to default file or target file
    #[structopt(short, long)]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
