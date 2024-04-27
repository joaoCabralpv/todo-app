use std::fs::{self,File};
use std::io::Write;

pub fn write(path:&str,text:&str){

    let mut file = match File::create(&path){
        Err(err) => {eprintln!("Error creating {}: {}",path,err);return ;}
        Ok(file) => file,
    };

    match file.write_all(text.as_bytes()) {
        Err(err) => {eprintln!("Error writing to {}: {}",path,err); return;}
        Ok(_) => println!("Saved to {} successfully ",path)
    }

}

pub fn read(path:&str) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(s) => Some(s),
        Err(e) => {eprintln!("Error reading {}: {}",path, e); None}
    }
}