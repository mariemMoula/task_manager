//use std::env;
use std::io;
use std::io::Write; 
struct Task{
    description: String,
    status: Status,
}
impl Task{
    fn new(description: String,)->Self{
        Self { description, status: Status::NotStarted }
    }
    fn set_task_status(&mut self, status: Status){
        self.status = status;
    }
    fn get_task_status(&self)->Status{
        self.status.clone()
    }
}
#[derive(Clone, Debug)] 
enum Status{
    NotStarted,
    InProgress,
    Completed,
}

    
struct TaskManager{
    tasks: Vec<Task>,
}

impl TaskManager{
    fn new()->Self{
        Self { tasks: Vec::new() }
    }
    fn add_task(&mut self, task: Task){

        self.tasks.push(task);
    }
    fn view_tasks(&self){
        for task in &self.tasks{
            println!("Task description: {} , Task Status: {:?}", task.description,task.get_task_status());
        }
    }
    fn update_status_task(&mut self, task: &str, status: Status)->Result<(), String>{
        for t in &mut self.tasks{
            if t.description == task{
                t.set_task_status(status);
                return Ok(());
            }
        }
        Err("Task not found".to_string())
    }

    fn delete_task(&mut self, task: &str)->Result<(), String>{
        let initial_len = self.tasks.len();
        self.tasks.retain(|t| t.description != task);
        if self.tasks.len() == initial_len {
            Err(format!("Task '{}' not found", task))
        } else {
            Ok(())
        }
    }


} 

pub struct Cli_input{
    command: String,
    task: String,
    status: Option<Status>, 

}
impl Cli_input{
    fn new(args:&[String])->Result<Self,String>{
        //if args.len() < 2{
        //    return Err("Please provide a command".to_string());
        //}

            let command = args[0].clone();
            let task = if args.len() > 1 { args[1].clone() } else { "".to_string() };
            
            let status: Option<Status> = if args.len() > 2 {
                match args[2].to_lowercase().as_str() {
                    "notstarted" => Some(Status::NotStarted),
                    "inprogress" => Some(Status::InProgress),
                    "completed" => Some(Status::Completed),
                    &_ => Some(Status::NotStarted),
                }
            } else {
                None
            };

            match command.as_str() {
                "add" | "delete" if args.len() < 1 => {
                    return Err(format!("'{}' command requires a task description", command));
                }
                "update" if args.len() < 2 => {
                    return Err("The 'update' command requires a task and a status.".to_string());
                }

                _ => Ok(Self { command, task, status })
        
    }
}
}
fn main() {
    let mut task_manager = TaskManager::new();
   // let command = cli_input.command.as_str();
    //let status = cli_input.status.unwrap_or(Status::NotStarted);
   // let args: Vec<String> = env::args().collect();


    println!("Hello, I am your task manager");
    loop {
        print!("Please Type 'view' to view all the tasks, 'update' to update the status of a task, 'add' to add a task,'delete' to delete a task,'exit' to exit the program ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let args: Vec<String> = input.trim().split_whitespace().map(String::from).collect();

        if args.is_empty() {
            continue;
        }

        // let cli_input = match Cli_input::new(&args) {
        //     Ok(input) => input,
        //     Err(e) => {
        //         eprintln!("{}", e);
        //         continue; 
        //     }


    

    let cli_input = match Cli_input::new(&args) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    match cli_input.command.as_str() {
        "view" => task_manager.view_tasks(),
        "add" => {
            if cli_input.task.is_empty() {
                eprintln!("Please provide the task description.");
            } else {
                task_manager.add_task(Task::new(cli_input.task.clone()));
                println!("Task added successfully!");
            }
        }
        "update" => {
            let status = cli_input.status;
            match task_manager.update_status_task(&cli_input.task, status.clone().unwrap()) {
                Ok(_) => println!("Task updated successfully to '{:?}'", status.unwrap()),
                Err(e) => eprintln!("{}", e),
            }
        }
        "delete" => {
            match task_manager.delete_task(&cli_input.task) {
                Ok(_) => println!("Task deleted successfully !."),
                Err(e) => eprintln!("{}", e),
            }
        }
        "exit" => {
            println!("Exited succesfully  !");
            break; // Exit the loop
        }
        _ => eprintln!("Invalid Command !"),
    }

    
        // if command == "view"{
        //     task_manager.view_tasks();
        // }
        // if command == "add"{
        //     if cli_input.task.is_empty() {
        //         eprintln!("Please provide a task description.");
        //     } else {
        //         task_manager.add_task(Task::new(cli_input.task.clone()));
        //     }
        // }
        // if command == "update"{
        //     match task_manager.update_status_task(&cli_input.task, status) {
        //         Ok(_) => println!("Task updated successfully."),
        //         Err(e) => eprintln!("{}", e),
        //     }
        // } else {
        //     eprintln!("Please provide a valid status.");
        // }
        // if command == "delete"{
        //     match task_manager.delete_task(&cli_input.task) {
        //         Ok(_) => println!("Task deleted successfully."),
        //         Err(e) => eprintln!("{}", e),
        //     }
        // if command == "exit"{
        //     println!("Exiting program");
        //     return ; 
        // }
        // else {
        //     eprintln!("Invalid command !");
        // }
}
}





