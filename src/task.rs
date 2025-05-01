use eframe::{self, Frame};
use egui::{self, menu, Checkbox, ComboBox, Context, ProgressBar, Ui};
use serde::{self, Serialize, Deserialize};
use crate::utils::{self, *};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    status: bool,
    title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskList {
pub   list: Vec<Task>,
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
    pub save_path :String 
}

impl Default for App {
    fn default() -> Self {
        App {
            new_task_title: String::new(),
            save_path: "todo.json".to_string(),
            task_list: load_from_file("todo.json").unwrap_or(TaskList::new()),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        load_from_file(self.save_path.as_str());
        egui::CentralPanel::default().show(ctx, |ui| {
            menu::menu_button(ui, "configs", |ui| {
                ui.label("Saved!");
            });

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task_title);
                if ui.button("Add task").clicked() {
                    self.add(&self.new_task_title.clone());
                    save_to_file(&self.task_list, &self.save_path.as_str());
                    self.new_task_title.clear();
                }
            });
            ui.separator();
            let mut changed = false;
            for task in &mut self.task_list.list {
            if ui.checkbox(&mut task.status, &task.title).changed(){
                    changed = true;
                }
            }

                if changed{
                    save_to_file(&self.task_list, &self.save_path);
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
    fn from_file(&mut self) {
        self.task_list = utils::load_from_file(self.save_path.as_str()).expect("kljgsaklj"); 
    }
    //create function to process keyboard input
}




