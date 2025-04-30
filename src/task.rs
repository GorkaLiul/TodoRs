use eframe::{self, Frame};
use egui::{self, Checkbox, Context, ProgressBar, Ui};

#[derive(Debug, Clone)]
struct Task {
    status: bool,
    title: String,
}

pub struct TaskList {
    list: Vec<Task>,
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

    fn from_task(task: Task) -> Self {
        TaskList { list: vec![task] }
    }

    pub fn add(&mut self, title: &str) {
        self.list.push(Task {
            title: title.to_string(),
            status: false,
        });
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
    pub new_task_title: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            task_list: TaskList::new(),
            new_task_title: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task_title);
                if ui.button("Add task").clicked() {
                    self.add(&self.new_task_title.clone());
                    self.new_task_title.clear();
                }
            });
            ui.separator();
            for task in &mut self.task_list.list {
                ui.checkbox(&mut task.status, &task.title);
            }

            //create function to process keyboard input
        });
    }
}

impl App {
    pub fn add(&mut self, title: &String) {
        self.task_list.add(title.as_str());
    }

    pub fn rem(&mut self, title: &String) {
        self.task_list.rem(title.as_str());
    }

    //create function to process keyboard input
}
