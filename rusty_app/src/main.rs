use std::fs::{OpenOptions, read_to_string};
use std::io::{self, Write};

fn main() {
    loop {
        println!("1. Add Task\n2. View Tasks\n3. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => add_task(),
            "2" => view_tasks(),
            "3" => break,
            _ => println!("Invalid choice, please try again"),
        }
    }
}

fn add_task() {
    println!("Enter task description:");
    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Failed to read line");

    let mut file = OpenOptions::new().append(true).open("tasks.txt").unwrap();
    writeln!(file, "{}", task.trim()).unwrap();
    println!("Task added.");
}

fn view_tasks() {
    let tasks = read_to_string("tasks.txt").unwrap();
    println!("Tasks:\n{}", tasks);
}
