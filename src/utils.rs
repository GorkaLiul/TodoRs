use std::{fs::{self, File}, io::{self, Read, Write}, path::Path};
use crate::task::TaskList;
use serde_json;

pub fn save_to_file(task_list: &TaskList, file_path: &str) -> io::Result<()>{
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, task_list)?;

    Ok(())
}

pub fn load_from_file(file_path: &str) -> io::Result<TaskList>{
    let contents = fs::read_to_string(file_path)?;
    let task_list : TaskList =  serde_json::from_str(&contents)?;
    
    Ok(task_list)

}
