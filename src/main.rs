mod list;
mod input;
mod file;
use list::List;
fn main() {
    let mut list = List::new();
    let mut running = true;
    while running{
    println!("Todo app v2.0");
    println!("1-List all tasks");
    println!("2-Add new task");
    println!("3-Remove task");
    println!("4-Mark task as done");
    println!("5-Mark task as not done");
    println!("6-Look \"more info\" for a task");
    println!("7-Set \"more info\" for a task");
    println!("8-Save tasks");
    println!("9-Load tasks");
    println!("10-Exit program");

    let option = input::as_u8();

    let option = match option {
        Some(i) => i,
        None => continue,
    };

    running = list.update(option)


    }
}
