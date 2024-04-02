use std::fs::{write};
use std::str::from_utf8;
use std::path::PathBuf;
use crate::battaglia_navale::Error::{BoatCount, OutOfBounds, Overlap};

const BSIZE: usize = 20;
#[derive(Debug)]
pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}
pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}
pub enum Boat {
    Vertical(usize),
    Horizontal(usize)
}
impl Board {
    /** crea una board vuota con una disponibilità di navi */
    pub fn new(boats: &[u8]) -> Board {
        let filepath = PathBuf::from("board.txt");
        let boats_tmp = from_utf8(boats).unwrap().to_string();

        let mut write_boats = boats_tmp.replace(",", " ");
        write_boats.push_str("\n");

        let binding = boats_tmp.replace(",", "");
        let convert_bytes = binding.as_bytes();
        let mut boats_new:  [u8; 4] = [b' '; 4];
        boats_new.copy_from_slice(convert_bytes);

        let space_line : [u8; BSIZE] = [b' '; BSIZE];
        let board: [[u8; BSIZE]; BSIZE] = [space_line; BSIZE];

        let mut x = String::from_utf8(board[0].clone().to_vec()).unwrap();
        x.push_str("\n");
        write_boats.push_str(x.repeat(BSIZE).as_str());
        write(&filepath, write_boats.clone()).expect("");
        Board {boats: boats_new, data: board}
    }

    /* crea una board a partire da una stringa che rappresenta tutto
    il contenuto del file board.txt  */
    pub fn from(s: String)-> Board {
        let all = s.split("\n");
        let mut board: [[u8; BSIZE]; BSIZE] = [[b' '; BSIZE]; BSIZE];
        let mut boats : [u8; 4] = [b' '; 4];
        for (i, s) in all.enumerate() {
            let str = s.to_string();
            match i {
                0 => {
                    let boats_as_str = str.replace(" ", "");
                    boats.copy_from_slice(boats_as_str.trim().as_bytes());
                }
                1..=20 => {
                    let line = str.replace("\n", "");
                    board[i-1].copy_from_slice(line.as_bytes());
                }
                _ => ()
            }
        }
        Board{boats, data: board}
    }
    /* aggiunge la nave alla board, restituendo la nuova board se
    possibile */

    /* bonus: provare a *non copiare* data quando si crea e restituisce
    una nuova board con la barca, come si può fare?  */
    pub fn add_boat(self, boat: Boat, pos: (usize, usize)) -> Result<Board, Error> {
        let b = b'B';
        let mut boats = self.boats.clone();
        let mut data = self.data.clone();

        match boat {
            Boat::Vertical(size) => {
                let last_pos = pos.0 + size.clone();
                if last_pos > BSIZE {
                    Err(OutOfBounds)
                }
                else if data[pos.0..last_pos].iter().any(|row| row[pos.1] == b) {
                    Err(Overlap)
                }
                else if  boats[size-1] == 0 {
                    Err(BoatCount)
                }
                else {
                    for i in pos.0..last_pos {
                        data[i][pos.1] = b.clone();
                    }
                    boats[size-1] -= 1;
                    Ok(Board {boats, data})
                }
            },
            Boat::Horizontal(size) => {
                // fix pos.0
                let last_pos = pos.1 + size.clone();
                if last_pos > size {
                    Err(OutOfBounds)
                }
                else if data[pos.0][pos.1..last_pos].contains(&b) {
                    Err(Overlap)
                }
                else if  boats[size-1] == 0 {
                    Err(BoatCount)
                }
                else {
                    for i in pos.1..last_pos {
                        data[pos.0][i] = b.clone();
                    }
                    boats[size-1] -= 1;
                    Ok(Board {boats, data})
                }
            }
        }
    }

    pub fn save_board(&self, path: PathBuf) {
        let mut str_boats = self.boats
            .iter()
            .map(|&x| x as char)
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        str_boats.push_str("\n");

        for line in self.data {
            str_boats.push_str(
                  line
                  .iter()
                  .map(|&x| x as char)
                  .map(|x| x.to_string())
                  .collect::<String>()
                  .as_str()
            );
            str_boats.push_str("\n");
        }

        write(path, str_boats).unwrap();
    }

    pub fn to_string(&self) -> String {
        let mut str_boats = self.boats
            .iter()
            .map(|&x| x as char)
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        str_boats.push_str("\n");

        for line in self.data {
            str_boats.push_str(
                line
                    .iter()
                    .map(|&x| x as char)
                    .map(|x| x.to_string())
                    .collect::<String>()
                    .as_str()
            );
        }

        str_boats
    }
}
