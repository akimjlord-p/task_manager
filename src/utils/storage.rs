use super::task::Task;
use serde::{Deserialize, Serialize};
use std::fs;


#[derive(Deserialize, Serialize)]
pub struct Storage{
    pub tasks: Vec<Task>
}

impl Storage{
    pub fn generate() -> Self{
        let file = fs::read_to_string("storage.json");
        match file{
            Result::Ok(file) => match serde_json::from_str(&file){
                Result::Ok(tasks) => tasks,
                Result::Err(_) => {
                    let tasks: Vec<Task> = Vec::new();
                    Self{
                        tasks
                    }
                }
            },
            Result::Err(_) => {
                let tasks: Vec<Task> = Vec::new();
                Self{
                    tasks
                }
            }
        }
    }
    pub fn save(&self){
        let json = serde_json::to_string_pretty(self)
            .expect("failed to serialize Storage");

        fs::write("storage.json", json)
            .expect("failed to write storage.json");
    }    
}

