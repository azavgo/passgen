//ASCI symbols selected for the password generation: 33 = "!", 35 = "#" - 38 = "&", 40 = "(" - 93 = "]", 95 = "_", 97 = "a" - 125 = "}"
//rng.gen_range(0..89) is equivalent to [0..89), 89 is not included, so it is the same as [0..88]

use rand::Rng;
use std::env;
use std::num::ParseIntError;
use std::string::{FromUtf8Error, String};

#[derive(Debug)]
pub enum PassgenError {
    FromUtf8Error(FromUtf8Error),
    ParseIntError(ParseIntError),
}

impl From<FromUtf8Error> for PassgenError {
    fn from(error: FromUtf8Error) -> Self {
        PassgenError::FromUtf8Error(error)
    }
}

impl From<ParseIntError> for PassgenError {
    fn from(error: ParseIntError) -> Self {
        PassgenError::ParseIntError(error)
    }
}

fn passgen(n: u8) -> Result<String, PassgenError> {
    let a: [u8; 89] = [
        33, 35, 36, 37, 38, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
        58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
        81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 95, 97, 98, 99, 100, 101, 102, 103,
        104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121,
        122, 123, 124, 125,
    ];

    let mut rng = rand::thread_rng();
    let mut pass_vec: Vec<u8> = vec![0; 8];

    for _i in 0..n {
        pass_vec.push(a[rng.gen_range(0..89)]);
    }

    let password = String::from_utf8(pass_vec)?;

    Ok(password)
}

pub fn passgen_ui() -> Result<(), PassgenError> {
    let args: Vec<String> = env::args().collect();

    match args.len() > 1 {
        true => {
            if args[1] == "?" || args[1] == "--help" {
                println!("usage: passgen [? | --help] [n=<number between 0 and 255, length of the password>]");
            } else {
                let n = args[1].parse::<u8>()?;
                println!("Password {} characters length: {}", n, passgen(n)?);
            }
        }
        _ => {
            println!("Password 8 characters length (default): {}", passgen(8)?);
        }
    }
    Ok(())
}

fn main() -> Result<(), PassgenError> {
    passgen_ui()?;

    Ok(())
}
