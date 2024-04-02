mod battaglia_navale;

use std::fmt::{Debug};
use std::path::PathBuf;
use clap::{Parser};
use battaglia_navale::{Board, Error, Boat};
use std::fs::{read_to_string};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, value_delimiter = ' ', num_args = 2)]
    new: Option<Vec<String>>,

    #[arg(short, long, value_delimiter = ' ', num_args = 3)]
    add: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();
    match args.new {
        Some(x) => {
            let mut str = x.clone();
            let boats = str.pop().unwrap();
            let board = Board::new(boats.as_bytes());
            println!("{:?}", board);
        },
        None => ()
    }

    match args.add {
        Some(x) => {
            let mut str = x.clone();
            let arg_tmp = str.pop().unwrap();
            let mut arg_pos = arg_tmp.split(",");
            let pos = (arg_pos.next().unwrap().trim().parse::<usize>().unwrap(), arg_pos.next().unwrap().trim().parse::<usize>().unwrap());

            let arg_tmp = str.pop().unwrap();
            let mut arg_len = arg_tmp.chars();
            let len = arg_len.next().unwrap().to_digit(10).unwrap() as usize;
            let boat = match arg_len.next().unwrap() as u8 {
                b'V' => Boat::Vertical(len),
                b'H' => Boat::Horizontal(len),
                _ => Boat::Vertical(0 as usize),
            };

            let path = PathBuf::from(str.pop().unwrap());

            let board_str = read_to_string(&path).unwrap();
            let board = Board::from(board_str);
            match board.add_boat(boat, pos) {
                Ok(x) => {
                    x.save_board(path);
                    let board_string = x.to_string();
                    println!("New Board: {}", board_string);
                },
                Err(Error::Overlap) => {},
                Err(Error::BoatCount) => {},
                Err(Error::OutOfBounds) => {}
            }
        },
        None => ()
    }
}
