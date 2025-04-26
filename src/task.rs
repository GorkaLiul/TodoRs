use egui::{self, Checkbox, Ui};
use eframe;

#[derive(Debug, Clone)]
struct Task{
    status : bool,
    title: String
}

pub struct TaskList { 
    list: Vec<Task>
}

impl Default for TaskList {
    fn default() -> Self {
        Self { list: Vec::new() }
    }
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { list: Vec::new() }
    }

    pub fn from_task(task: Task) -> Self {
        TaskList { list: vec![task] }
    }

    pub fn add(&mut self, title: &str) {
        self.list.push(Task { title: title.to_string(), status: false });
    }

    pub fn rem(&mut self, title: &str) {
        self.list.retain(|task| task.title != title);
    }

    pub fn display_(&self) {
        for task in &self.list {
            println!("{}: {} \n", task.status, task.title);
        }
    }
}

pub struct App {
    pub task_list: TaskList,
    pub new_task_title: String, // Add this field to store the new task input
}

impl Default for App {
    fn default() -> Self {
        Self {
            task_list: TaskList::new(),
            new_task_title: String::new(), // Initialize this field
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Text input to add a task
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task_title);
                if ui.button("Add Task").clicked() {
                    if !self.new_task_title.is_empty() {
                        self.task_list.add(&self.new_task_title);
                        self.new_task_title.clear(); // Clear the input field after adding
                    }
                }
            });

            ui.separator(); // To separate input and task list

            // Loop through all tasks and display them
            for task in &mut self.task_list.list {
                ui.checkbox(&mut task.status, &task.title); // Create checkbox for each task
            }
        });
    }
}

impl App {
    pub fn add(&mut self, title: &str) {
        self.task_list.add(title);
    }

    pub fn rem(&mut self, title: &str) {
        self.task_list.rem(title);
    }
}

