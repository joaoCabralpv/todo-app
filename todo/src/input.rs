use std::io;

pub fn input() -> String {
    let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

pub fn trimmed() -> String {
    input().trim().to_string()
}

pub fn as_usize() -> usize {
    let input: usize;
    loop {
         input = match trimmed().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
        return input;
    }
}

pub fn as_u8() -> u8 {
    let input: u8;
    loop {
         input = match trimmed().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
        return input;
    }
}
pub fn confirm() -> bool {
   trimmed().to_lowercase() == "y"
}
    