use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
enum Status{
    InProcess,
    Done
}
#[derive(Deserialize, Serialize)]
pub struct Task{
    pub name: String,
    status: Status
}

impl Task{
    pub fn new(name: &str) -> Self{
        Self{
            name: name.to_string(),
            status: Status::InProcess
        }
    }
    pub fn done(&mut self){
        self.status = Status::Done;
    }
    pub fn is_done(&self) -> bool{
        self.status == Status::Done
    }
}
