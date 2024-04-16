use std::vec;
use crate::input;

struct Task {
    pub name:String,
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

    fn is_index_valid_with_print(&self, index: usize, print: bool) -> bool {
        if index <= self.tasks.len()-1 {
            return true;
        }
        if print {
            println!("\nTask is not present\npress enter to continue");
            input::wait_until_enter();
        }
        false
    }

    pub fn update(&mut self,option:u8) -> bool{
        match option {
            1 => self.list_tasks(),
            2 => self.add_task(),
            3 => self.remove_task(),
            4 => self.mark_as_done(),
            5 => self.mark_task_as_not_done(),
            6 => return false,
            _ => println!("Invalid option"),
        }
        return true;
    }

    fn list_tasks(&self) {
        for i in 0..self.tasks.len() {
            println!("{}. {}",i,self.tasks[i].to_string());
        }
        println!("Press enter to go to the main menu");
        input::wait_until_enter();
    }

    fn add_task(&mut self) {
        println!("What is the name of the task you want to add");
        self.tasks.push(Task::new(input::trimmed()));
    }

    fn remove_task(&mut self) {
        println!("What is the number of the task you want to remove");
        let task = input::as_usize();
        if !self.is_index_valid_with_print(task, true) {
            return;
        }
        println!("Do you want to to delete {} (Y/n)",self.tasks[task].to_string());

        if input::confirm() {
            self.tasks.remove(task);
        }

    }
    
    fn mark_as_done(&mut self) {
        println!("What is the number of the task you want to mark as done");
        let task = input::as_usize();
        if !self.is_index_valid_with_print(task, true) {
            return;
        }
        println!("Do you want to mark {} as done (Y/n)", self.tasks[task].name);
        if input::confirm() {
            self.tasks[task].done = true;
        }
    }

    fn mark_task_as_not_done(&mut self) {
        println!("What is the number of the task you want to mark as not done");
        let task = input::as_usize();
        if !self.is_index_valid_with_print(task, true) {
            return;
        }
        println!("Do you want to mark {} as not done (Y/n)", self.tasks[task].name);
        if input::confirm() {
            self.tasks[task].done = false;
        }
    }

}
