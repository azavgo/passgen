//ASCI symbols selected for the password generation: 33 = "!", 35 = "#" - 38 = "&", 40 = "(" - 93 = "]", 97 = "a" - 125 = "}"
//The array is 0..87. rng.gen_range(0..88) generates a number [0..88), 88 is not included, so it is [0..87]

use rand::Rng;
use std::string::{String, FromUtf8Error};

#[derive(Debug)]
pub enum PassgenError {
    FromUtf8Error(FromUtf8Error),
}

impl From<FromUtf8Error> for PassgenError {
    fn from(error: FromUtf8Error) -> Self {
        PassgenError::FromUtf8Error(error)
    }
}

pub fn passgen(n: u8) -> Result<String, PassgenError>{
    let a:[u8; 88] = [33, 35, 36, 37, 38, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125];
    
    let mut rng = rand::thread_rng();
    let mut pass_vec: Vec<u8> = vec![0; 8]; 
    
    for _i in 0..n {
        pass_vec.push(a[rng.gen_range(0..88)]);
    }
    
    let password = String::from_utf8(pass_vec)?;
    
    Ok(password)
}

fn main() ->Result<(), PassgenError> {
    
    let n = 10; 
    println!("Password {} symbols length: {}", n, passgen(n)?); 
    Ok(())
}
