use std::vec;
use std::io;
pub struct List {
    tasks:vec::Vec<String>,
}

impl List {
    pub fn new() -> List {
        List{
            tasks:Vec::new()
        }
    }
    pub fn update(&mut self,option:u8) {
        match option {
            1 => self.list_tasks(),
            2 => self.add_task(),
            3 => self.remove_task(),
            4 => eprintln!("App should have closed but it didn't"),
            _ => println!("Invalid option"),
        }

    }

    fn list_tasks(&self) {
        for i in 0..self.tasks.len() {
            println!("{}. {}",i,self.tasks[i])
        }
    }

    fn add_task(&mut self) {
        println!("What is the name of the task you want to add");
        let mut task = String::new();
        io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line");
        task = task.trim().to_string();
        self.tasks.push(task);
    }

    fn remove_task(&mut self) {
        let mut task = String::new();
        io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line");
        task = task.trim().to_string();

        
        let task: usize = match task.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("Please enter a number");
                return;
            }
        };

        self.tasks.remove(task);
    }
}
