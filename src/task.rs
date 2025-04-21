#[derive(Debug, Clone)]
struct Task{
    status : bool,
    title: String
}

pub  struct TaskList{ 
    list :  Vec<Task>
}

impl Default for TaskList{
    fn default() -> Self {
        Self::new()
    }
}
impl TaskList {
    pub fn new() -> Self {
        TaskList{list : Vec::new()}
    }

    pub fn from_task(task : Task) -> Self{
        TaskList{list : vec![task]}
    }

    pub fn add(&mut self, title : &str) {
        self.list.push(Task{title: title.to_string(), status : false});
    }
    pub fn rem(&mut self, title : &str) {
        self.list.retain(|task| task.title != title);
    }
    pub fn display(& self) {
        for task in &self.list{
            println!("{}: {} \n", task.status, task.title);
        }
    }
}
