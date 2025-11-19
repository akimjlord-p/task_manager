use crate::utils::task::Task;
use crate::utils::storage::Storage;
use std::io;


enum Command{
    Add(String),
    List,
    Done(u32),
    Remove(u32),
    ClearDone,
    Error,
    Save
}
pub struct CliApp{
    storage: Storage
}
impl CliApp{
    pub fn initiate() -> Self{
        Self{
            storage: Storage::generate()
        }
    }




    fn parse_input(s: String) -> Command{
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.is_empty() {
            return Command::Error;
        }
        match words[0] {
            "add" => Command::Add(words[1..].join(" ")),
            "list" => Command::List,
            "done" => {let id = match words[1].parse::<u32>() {
                                Ok(n) => n,
                                Err(_) => return Command::Error
                        };
                        Command::Done(id)
                    },
            "remove" => {let id = match words[1].parse::<u32>() {
                                Ok(n) => n,
                                Err(_) => return Command::Error
                        };
                        Command::Remove(id)
                    },
            "clear" => Command::ClearDone,
            "save" => Command::Save,
            _ => Command::Error
        }
    }


    fn execute(&mut self, command: Command){
        match command {
            Command::Add(task_name) => self.storage.tasks.push(Task::new(&task_name)),
            Command::ClearDone => self.storage.tasks.retain(|t| !t.is_done()),
            Command::Error => println!("Error in input!"),
            Command::List => {
                let mut counter = 0;
                for task in &self.storage.tasks{
                    counter += 1;
                    let task_name = &task.name;
                    match task.is_done() {
                        true => println!("{counter}. {task_name} done"),
                        _ => println!("{counter}. {task_name} in process")
                    }
                }
            },
            Command::Remove(index) => {
                let index = index - 1;
                if index as usize >= self.storage.tasks.len() {
                    println!("Нет задачи с индексом {}", index);
                } else {
                    self.storage.tasks.remove(index as usize);
                }
            }

            Command::Done(index) => {
                let index = index - 1;
                if let Some(task) = self.storage.tasks.get_mut(index as usize) {
                    task.done();
                } else {
                    println!("Нет задачи с индексом {}", index);
                }
            }
            Command::Save => self.storage.save()
        }
    }


    pub fn start_cycle(&mut self){
        loop{
            let mut input = String::new();
            println!("Enter the command >");
            io::stdin().read_line(&mut input).expect("Error with input in [fn get_input]");
            let command = Self::parse_input(input.trim().to_string());
            self.execute(command);
        }
    }
}
