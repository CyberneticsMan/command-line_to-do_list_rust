struct Task {
    name: String,
    done: bool,
}
use std::io;

fn create_tasks() -> Task {
    let mut input_string: String = String::new();
    io::stdin().read_line(&mut input_string).unwrap(); // Get the stdin from the user, and put it in read_string
    let created_task = Task {name: input_string, done: false};
    return created_task;
}


fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut input_string: String = String::new();
    let mut should_run : bool = true;

    while should_run {
        io::stdin().read_line(&mut input_string).unwrap();
        
        match input_string.to_string() == "create" {
            true => {
                let created_task = create_tasks();
                tasks.push(created_task);
            }
            _ => println!("error!")
        }
        match input_string.to_string() == "exit" {
            true => should_run = false,
            _ => println!("error!")
        }
    }
    for task in tasks {
        println!("{}", task.name);
        println!("{}", task.done);
        println!("--------------------------------")
    }
    

    
}
