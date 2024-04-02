mod error;
mod read_files;
mod node;

use std::path::PathBuf;
use std::str::from_utf8;
use clap::{command, Parser};
use read_files::read_and_repeat;
use node::Node;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    filepath: Option<PathBuf>
}


fn main() {
    /*
    let arg = Args::parse();
    match arg.filepath {
        Some(x) if !x.is_file() || !x.exists() => println!("Path or file do not exist!"),
        Some(x) => {
            read_and_repeat(&x);
        },
        None => ()
    }
    */

    let mut node = Node::new(String::from("nodo")).size(10).count(5);
    println!("{}", node.to_string());
    node.grow();
    node.inc();
    println!("{}", node.to_string());

    let prova = "ciao ciao".as_bytes();
    let mut ciao : [u8; 8] = [b' '; 8];
    ciao.copy_from_slice(&prova[0..8]);
    println!("{:?}", ciao);
}
