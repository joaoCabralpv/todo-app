mod list;
use std::io;
use list::List;
fn main() {
    let mut list = List::new();
    loop {
    println!("Todo app v2.0");
    println!("1-List all tasks");
    println!("2-Add new task");
    println!("3-Remove task");
    println!("4-Exit program");

    let mut option = String::new();
    io::stdin()
    .read_line(&mut option)
    .expect("Failed to read line");

    
    let option: u8 = match option.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Please enter a number");
            continue;
        }
    };

    if option == 4{
        break;
    }

    list.update(option)


    }
}
