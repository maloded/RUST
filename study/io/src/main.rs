use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_owned(),
            Priority::Medium => "Medium".to_owned(),
            Priority::High => "High".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    description: String,
    priority: Priority,
    add_time: DateTime<Local>,
}

impl Task {
    fn new(name: String, description: String, priority: Priority) -> Self {
        Self {
            name,
            description,
            priority,
            add_time: Local::now(),
        }
    }

    fn new_from_console() -> Self {
        let name = ConsoleManager::input("Enter new task name: ").unwrap();
        let description = ConsoleManager::input("Enter new task description: ").unwrap();
        let priority = match ConsoleManager::input("Enter new task priority: ")
            .unwrap()
            .to_lowercase()
            .as_str()
        {
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            _ => {
                println!("Invalid priority, setting to low");
                Priority::Low
            }
        };
        Self::new(name, description, priority)
    }

    fn print_task(&self) {
        println!(
            "{}, {}, {}, '{}'",
            self.name,
            self.priority.to_string(),
            self.add_time.format("%d-%m-%Y %H:%M:%S"),
            self.description
        );
    }
}

struct TasksManager {
    tasks: Vec<Task>,
}

impl TasksManager {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn print_tasks(&self) {
        for task in &self.tasks {
            task.print_task();
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            self.tasks.remove(index);
            Ok(format!("Task '{}' removed successfully", name))
        } else {
            Err(format!("Task with name '{}' doesn't exist", name))
        }
    }

    fn find_task(&self, name: &str) -> Option<usize> {
        self.tasks.iter().position(|task| task.name == name)
    }

    fn edit_task(&mut self, name: &str, updated_tusk: Task) -> Result<String, String> {
        if let Some(index) = self.find_task(&name) {
            match self.tasks.get_mut(index) {
                Some(task) => {
                    task.name = updated_tusk.name;
                    task.description = updated_tusk.description;
                    task.priority = updated_tusk.priority;
                    Ok(format!("Task '{}' updated successfully", name))
                }
                None => Err("Error borrowing task".to_owned()),
            }
        } else {
            Err(format!("Task with name '{}' doesn't exist", name))
        }
    }

    fn store_to_file(&self, filename: &str) -> Result<String, String> {
        if !Path::new(filename).exists() {
            let file =
                File::create(filename).map_err(|err| format!("Error creating file: {}", err))?;

            let _ = match serde_json::to_writer(&file, &self.tasks) {
                Ok(_) => Ok("Data stored successfully".to_owned()),
                Err(err) => Err(format!("Error saving data: {}", err)),
            };

            Ok("".to_owned())
        } else {
            Err("Filename '{filename}' already exists".to_owned())
        }
    }

    fn read_from_file(&mut self, filename: &str) -> Result<String, String> {
        if Path::new(filename).exists() {
            let file =
                File::open(filename).map_err(|err| format!("Error creating file: {}", err))?;

            let reader = BufReader::new(file);
            self.tasks = match serde_json::from_reader(reader) {
                Ok(data) => data,
                Err(err) => return Err(format!("Error reading file, {}", err)),
            };
            Ok("".to_owned())
        } else {
            Err("Filename '{filename}' doesn't exist".to_owned())
        }
    }
}

struct ConsoleManager {
    tasks_manager: TasksManager,
    menu_options: Vec<String>,
}

impl ConsoleManager {
    fn new() -> Self {
        Self {
            tasks_manager: TasksManager::new(),
            menu_options: vec![
                "Add task".to_owned(),
                "Find task".to_owned(),
                "Edit task".to_owned(),
                "Remove task".to_owned(),
                "Print task".to_owned(),
                "Store task to file".to_owned(),
                "Read task from file".to_owned(),
            ],
        }
    }

    fn print_menu(&self) {
        for (index, menu_option) in self.menu_options.iter().enumerate() {
            println!("{}. {};", index + 1, menu_option);
        }
    }

    fn input(query: &str) -> std::io::Result<String> {
        print!("{}", query);
        std::io::stdout().flush()?;

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }

    fn process_command(&mut self) {
        match Self::input("Enter command index: ") {
            Ok(command) => match command.as_str() {
                "1" => {
                    self.tasks_manager.add_task(Task::new_from_console());
                }
                "2" => {
                    let name = match Self::input("Enter task name to find: ") {
                        Ok(name) => name,
                        Err(err) => {
                            println!("Error getting user input: {}", err);
                            return;
                        }
                    };
                    match self.tasks_manager.find_task(name.as_str()) {
                        Some(index) => self.tasks_manager.tasks.get(index).unwrap().print_task(),
                        None => println!("Task with name '{}' doesn't exist", name),
                    }
                }
                "3" => {
                    let name = match Self::input("Enter task name to edit: ") {
                        Ok(name) => name,
                        Err(err) => {
                            println!("Error getting user input: {}", err);
                            return;
                        }
                    };
                    match self
                        .tasks_manager
                        .edit_task(name.as_str(), Task::new_from_console())
                    {
                        Ok(msg) => println!("{}", msg),
                        Err(err) => println!("{}", err),
                    }
                }
                "4" => {
                    let name = match Self::input("Enter task name to remove: ") {
                        Ok(name) => name,
                        Err(err) => {
                            println!("Error getting user input: {}", err);
                            return;
                        }
                    };
                    match self.tasks_manager.remove_task(name.as_str()) {
                        Ok(msg) => println!("{}", msg),
                        Err(err) => println!("{}", err),
                    }
                }
                "5" => self.tasks_manager.print_tasks(),
                "6" => {
                    let filename = match Self::input("Enter filename to store data in: ") {
                        Ok(filename) => filename,
                        Err(err) => {
                            println!("Error getting user input: {}", err);
                            return;
                        }
                    };
                    match self.tasks_manager.store_to_file(filename.as_str()) {
                        Ok(msg) => println!("{}", msg),
                        Err(err) => println!("{}", err),
                    }
                }
                "7" => {
                    let filename = match Self::input("Enter filename to read data from: ") {
                        Ok(filename) => filename,
                        Err(err) => {
                            println!("Error getting user input: {}", err);
                            return;
                        }
                    };
                    match self.tasks_manager.read_from_file(filename.as_str()) {
                        Ok(msg) => println!("{}", msg),
                        Err(err) => println!("{}", err),
                    }
                }
                _ => {
                    println!("I don't understand this command :(");
                }
            },
            Err(err) => {
                println!("Error getting user input: {err};");
            }
        }
    }
}

fn main() {
    let mut manager = ConsoleManager::new();
    manager.print_menu();

    loop {
        manager.process_command();
    }
}
