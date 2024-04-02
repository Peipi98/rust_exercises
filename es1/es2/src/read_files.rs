use std::fs::{read, read_to_string, write};
use std::path::PathBuf;

pub fn read_and_repeat(filepath: &PathBuf) {
    let res = read_to_string(filepath);
    let res_read = read(filepath).unwrap();
    println!("res: {:?}", res);
    println!("res_read: {:?}", res_read);

    if res.is_ok() {
        let text = res.unwrap().repeat(10);
        write(filepath, text).expect("");
    }
    else {
        panic!("error {:?}", res.err().unwrap())
    }
}