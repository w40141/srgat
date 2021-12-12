use std::path::PathBuf;
use structopt::StructOpt;

/// Manage tags written in comments in files
#[derive(StructOpt, Debug)]
#[structopt(name = "srgat", about = "tag search for source code")]
struct Opt {
    /// Pring tags in the files
    #[structopt(short, long, parse(from_os_str))]
    files: Option<Vec<PathBuf>>,
    /// Print tags in the directory
    #[structopt(short = "r")]
    directory: Option<String>,
    /// Ignore the files
    #[structopt(short, long)]
    ignore: Option<String>,
    // /// Print the all saved tags in default file or target file
    // #[structopt(short = "t", default_value = "json")]
    // ftype: String,
    // /// Save the tags in the files to default file or target file
    // #[structopt(short, long)]
    // dump: Option<PathBuf>,
}

fn run_args(opt: Opt) {
    // TODO: すっきりさせたい。
    match opt.files {
        Some(v) => run_files(v),
        None => match opt.directory {
            Some(v) => run_dirctory(v, opt.ignore),
            None => run_show(),
        },
    }
}

fn run_files(v: Vec<PathBuf>) {
    println!("{:#?}", v);
}

fn run_dirctory(dir: String, ignore: Option<String>) {
    match ignore {
        Some(v) => println!("{:#?}, {}", dir, v),
        None => println!("{:#?}", dir),
    }
}

fn run_show() {
    println!("show")
}

fn main() {
    let opt = Opt::from_args();
    run_args(opt)
}
