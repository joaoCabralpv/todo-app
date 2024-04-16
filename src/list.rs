use std::vec;
use std::io;
use crate::input;

struct Task {
    name:String,
    pub done:bool,
}

impl Task {
    pub fn new(p_name:String) -> Task {
        Task { name: p_name, done: false }
    }
    pub fn to_string(&self) -> String{
        let done = match self.done {
            true => "(Task is done)",
            false => "",
        };
        self.name.clone()+done
    }
}

pub struct List {
    tasks: vec::Vec<Task>,
}

impl List {
    pub fn new() -> List {
        List{
            tasks:Vec::new()
        }
    }
    pub fn update(&mut self,option:u8) -> bool{
        match option {
            1 => self.list_tasks(),
            2 => self.add_task(),
            3 => self.remove_task(),
            4 => self.mark_as_done(),
            5 => return false,
            _ => println!("Invalid option"),
        }
        return true;
    }

    fn list_tasks(&self) {
        for i in 0..self.tasks.len() {
            println!("{}. {}",i,self.tasks[i].to_string());
        }
        println!("Press enter to go to the main menu");
        _ = io::stdin().read_line(&mut String::new());
    }

    fn add_task(&mut self) {
        println!("What is the name of the task you want to add");
        self.tasks.push(Task::new(input::trimmed()));
    }

    fn remove_task(&mut self) {
        println!("What is the number of the task you want to remove");
        let task = input::as_usize();
        println!("Do you want to to delete {} (Y/n)",self.tasks[task].to_string());

        if input::confirm() {
            self.tasks.remove(task);
        }

    }
    
    fn mark_as_done(&mut self) {
        println!("What is the number of the task you want to mark as done");
        let task = input::as_usize();
        println!("Do you want to mark {} as done (Y/n)", self.tasks[task].to_string());
        if input::confirm() {
            self.tasks[task].done = true;
        }
    }

}