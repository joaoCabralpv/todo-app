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

pub fn as_u8() -> Option<u8>{
        match trimmed().parse() {
            Ok(i) => return Some(i),
            Err(_) => {
                println!("Please enter a number");
                return None;
            }
        };
}
pub fn confirm() -> bool {
   trimmed().to_lowercase() == "y"
}
    
pub fn wait_until_enter() {
    io::stdin().read_line(&mut String::new()).unwrap_or(0);
}

pub fn more_info() -> String{
    let mut info = String::new();
    loop {
        // Removes last character from string
        let input = self::trimmed();
        if input == "" {
            let mut chars = info.chars();
            chars.next_back();
            return chars.as_str().to_string();
        }
        info.push_str(&input);
        info.push_str("\n");
    }
}