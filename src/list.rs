use std::vec;
use crate::input;
use crate::file;

#[derive(Clone)]
struct Task {
    pub name:String,
    pub more_info:String,
    pub done:bool,
}


impl Task {
    pub fn new(p_name:String, p_info:String) -> Task {
        Task { name: p_name, more_info: p_info ,done: false }
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
            6 => self.print_more_info(),
            7 => self.set_more_info(),
            8 => self.save(),
            9 => self.load(),
            10 => return false, //Exit the program
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
        let name = input::trimmed();
        println!("What do you want to put in the more info section\nenter a empty line to mork the end of the text");
        let more_info = input::more_info();
        self.tasks.push(Task::new(name,more_info));
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
    
    fn print_more_info(&self) {
        println!("What is the number of the task you want to get more info on?");
        let task = input::as_usize();
        if !self.is_index_valid_with_print(task, true) {
            return;
        }
        println!("{}",self.tasks[task].more_info)
    }
    
    fn set_more_info(&mut self) {
        println!("What is the number of the task you want to get more info on?");
        let task = input::as_usize();
        if !self.is_index_valid_with_print(task, true) {
            return;
        }
        println!("Do you want to change \"more info\" from {} (Y/n)", self.tasks[task].name);
        if input::confirm() {
            println!("What do you want to put in the more info section\nenter a empty line to mork the end of the text");
            self.tasks[task].more_info = input::more_info();
        }
    }
    fn save(&self) {
        const TASK_SEPARATOR:u16 = 0xFFFF;
        const TASK_FILED_SEPARATOR:u16 = 0xFFFE;
        let spacer: String = String::from_utf16(&[TASK_SEPARATOR]).unwrap();
        let task_filed_separator: String = String::from_utf16(&[TASK_FILED_SEPARATOR]).unwrap();

        println!("Where do you want to save the program");
        let path = input::trimmed();

        let mut save_text= String::new();

        for task in self.tasks.clone(){
            save_text.push_str(&task.name);
            save_text.push_str(&task_filed_separator);
            save_text.push_str(&task.more_info);
            save_text.push_str(&task_filed_separator);
            if task.done{
                save_text.push_str("T");
            }
            save_text.push_str(&spacer);

        }

        file::write(&path, &save_text);
    }

    fn load(&mut self) {
        const TASK_SEPARATOR:u16 = 0xFFFF;
        const TASK_FILED_SEPARATOR:u16 = 0xFFFE;
        let spacer: String = String::from_utf16(&[TASK_SEPARATOR]).unwrap();
        let task_filed_separator: String = String::from_utf16(&[TASK_FILED_SEPARATOR]).unwrap();

        println!("Where is the file you want to load");
        let path = input::trimmed();

        let contents = match file::read(&path) {
            Some(s) => s,
            None => return,
        };
        for task in contents.split(&spacer) {
            if task == "" {
                continue;
            }
            let mut fields = task.split(&task_filed_separator);
            let name = match fields.next() {
                Some(s) => s.to_string(),
                None => {eprintln!("Loading error loading tasks, unable to get name of a task");continue}
            }; 
            let more_info = match fields.next() {
                Some(s) => s.to_string(),
                None => {eprintln!("Loading error loading tasks, unable to get more info field of a task");String::from("")}
            }; 
            let task_marked_as_done = fields.next() == Some("T");
            self.tasks.push(Task{name:name,more_info:more_info,done:task_marked_as_done});
        }
    }
    

}
