use regex::Regex;
use std::{fs::read_to_string, path::PathBuf};
use structopt::StructOpt;

/// Manage tags written in comments in files
#[derive(StructOpt, Debug)]
#[structopt(name = "srgat", about = "tag search for source code")]
pub struct Cli {
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

fn find_tags(num: usize, line: &str) {
    let tags = [
        "TODO: ",
        "INFO: ",
        "FIX: ",
        "WARNING: ",
        "NOTE: ",
        "HACK: ",
        "PERF: ",
    ];
    for tag in tags {
        if find_tag(line, tag) {
            print_line(num, line)
        }
    }
}

fn find_tag(line: &str, tag: &str) -> bool {
    let re = Regex::new(tag).unwrap();
    let contains_tag = re.find(line);
    match contains_tag {
        Some(_) => true,
        None => false,
    }
}

#[test]
fn test_find_tag() {
    let result = find_tag("TODO: This is a todo.", "TODO: ");
    assert_eq!(result, true);
    let result = find_tag("NOTE: This is a NOTE.", "NOTE: ");
    assert_eq!(result, true);
}

fn run_files(v: Vec<PathBuf>) {
    read_files(v)
}

fn read_files(vp: Vec<PathBuf>) {
    for v in vp {
        read_file(v);
        println!("");
    }
    // NOTE: 改行
}

fn read_file(p: PathBuf) {
    println!("{:?}", p);
    let content = read_to_string(&p).expect("could not read file");
    read_lines(content)
}

fn read_lines(content: String) {
    for (i, line) in content.lines().enumerate() {
        let num = i + 1;
        find_tags(num, line)
    }
}

fn print_line(num: usize, line: &str) {
    println!("{}: {}", num, line);
}

fn run_dirctory(dir: String, ignore: Option<String>) {
    match ignore {
        Some(v) => println!("{:#?}, {}", dir, v),
        None => println!("{:#?}", dir),
    }
}

// TODO: showをerror表示から変える
fn run_show() {
    println!("error")
}

// TODO: すっきりさせる
fn run(cli: Cli) {
    match cli.files {
        Some(v) => run_files(v),
        None => match cli.directory {
            Some(v) => run_dirctory(v, cli.ignore),
            None => run_show(),
        },
    }
}

fn main() {
    let cli = Cli::from_args();
    run(cli)
}
