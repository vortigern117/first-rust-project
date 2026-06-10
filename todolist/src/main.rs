use std::io::Read;
use std::process;

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

fn view_tasks(task_vector: &mut Vec<String>)
{
    for item in task_vector
    {
        println!("{}", item);
    }
}

fn remove_tasks(task_vector: &mut Vec<String>)
{
    println!("Which task would you like to remove?");
    for (i, item) in task_vector.iter().enumerate()
    {
        println!("{}: {}", i+1, item);
    }
}

fn menu()
{
    println!("1. Add tasks\n2. View tasks\n3. Remove tasks\n4. Exit");
}

fn main() 
{
    let mut task_vector: Vec<String> = Vec::new();

    loop
    {
        menu();
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).expect("failed");
        println!("selected {}", user_input);

        match user_input.trim()
        {
            "1"=>add_tasks(&mut task_vector),
            "2"=>view_tasks(&mut task_vector),
            "3"=>remove_tasks(&mut task_vector),
            "4"=>process::exit(0),
            _ => println!("invalid"),
        }
    }
}