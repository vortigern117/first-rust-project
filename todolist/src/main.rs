use std::io::Read;

struct Task
{
    task_name: String,
}

fn add_tasks(task_vector: &mut Vec<String>)
{
    println!("Enter task name:");
    let mut task_name: String = String::new();
    std::io::stdin().read_line(&mut task_name).expect("Failed to read task name.");

    task_vector.push(task_name);
}

fn view_tasks()
{
    println!("placeholder");
}

fn menu()
{
    println!("1. Add tasks\n2. View tasks");
}

fn main() 
{
    let mut task_vector: Vec<String> = Vec::new();

    menu();
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("failed");
    println!("selected {}", user_input);

    match user_input.trim()
    {
        "1"=>add_tasks(&mut task_vector),
        "2"=>view_tasks(),
        _ => println!("invalid"),
    }
}