mod list;
mod input;
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
    println!("5-Exit program");

    let option = input::as_u8();

    running = list.update(option)


    }
}
