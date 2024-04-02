use std::time::SystemTime;

enum Error {
    Simple(SystemTime),
    Complex(SystemTime, String)
}

fn print_error(e: Error) {
    match e {
        Error::Simple(x) => {
            match x.elapsed() {
                Ok(elapsed) => println!("{}", elapsed.as_secs()),
                Err(_)=> ()
            }
        },
        Error::Complex(x, y) => {
            match x.elapsed() {
                Ok(elapsed) => println!("{} - {}", elapsed.as_secs(), y),
                Err(_)=> ()
            }
        }
    }
}