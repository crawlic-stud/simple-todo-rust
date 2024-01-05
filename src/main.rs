use colored::Colorize;
use std::io;

const BREAKLINE: &str = "======================================";

enum Action {
    New,
    MarkDone,
    Remove,
    ViewTasks,
    ViewDone,
    ViewAll,
}

struct Task {
    task: String,
    done: bool,
}

fn choose_action(ch: &str) -> Action {
    match ch {
        "a" => Action::ViewAll,
        "d" => Action::MarkDone,
        "r" => Action::Remove,
        "t" => Action::ViewTasks,
        "v" => Action::ViewDone,
        _ => Action::New,
    }
}

fn read_user_input(buffer: &mut String) {
    match io::stdin().read_line(buffer) {
        Ok(_) => breakline(),
        Err(error) => println!("Error: {error}. Please try again."),
    }
}

fn add_todo_to_db(todo: Task, db: &mut Vec<Task>) {
    db.push(todo);
}

fn print_all_tasks(tasks: &Vec<Task>) {
    for (i, item) in tasks.iter().enumerate() {
        if item.done {
            println!(
                "{}. {}",
                i + 1,
                format!("{}✓", item.task).to_string().green()
            );
        } else {
            println!("{}. {}", i + 1, item.task);
        }
    }
}

fn print_done_tasks(tasks: &Vec<Task>) {
    for (i, item) in tasks.iter().enumerate() {
        if item.done {
            println!(
                "{}. {}",
                i + 1,
                format!("{}✓", item.task).to_string().green()
            );
        }
    }
}

fn print_not_done_tasks(tasks: &Vec<Task>) {
    for (i, item) in tasks.iter().enumerate() {
        if !item.done {
            println!("{}. {}", i + 1, item.task);
        }
    }
}

fn populate_commands(vec: &mut Vec<(&str, String)>) {
    vec.push(("a", "View all todos".to_string()));
    vec.push(("d", "Mark task as done".to_string()));
    vec.push(("r", "Remove a task".to_string()));
    vec.push(("t", "View uncomplete tasks".to_string()));
    vec.push(("v", "View done tasks".to_string()));
}

fn print_menu(commands: &Vec<(&str, String)>) {
    for (command, descr) in commands {
        println!("{} - {}", command, descr);
    }
}

fn breakline() {
    println!("{}", BREAKLINE.yellow());
}

fn mark_task_done(tasks: &mut Vec<Task>, number: usize) {
    for (i, item) in tasks.iter_mut().enumerate() {
        if i == (number - 1) {
            item.done = true;
            return;
        }
    }
    println!("{}{number}", "No task was found for number ".red());
}

fn remove_task(tasks: &mut Vec<Task>, number: usize) {
    if number > tasks.len() {
        println!("{}", "Number is beyond limits!".red());
        return;
    }
    tasks.remove(number - 1);
}

fn main() {
    let mut todos: Vec<Task> = Vec::new();

    let mut commands: Vec<(&str, String)> = Vec::new();
    populate_commands(&mut commands);

    loop {
        println!(
            "{}{}{}{}{}{}",
            "=================".yellow(),
            "M".yellow(),
            "E".yellow(),
            "N".yellow(),
            "U".yellow(),
            "=================".yellow(),
        );
        print_menu(&commands);
        breakline();

        let mut input = String::new();

        println!(
            "{}",
            "Type to add new TODO or enter a command: ".blue().bold()
        );
        read_user_input(&mut input);

        let action = choose_action(&input.trim());

        match action {
            Action::New => {
                let task = Task {
                    task: input.trim().to_string(),
                    done: false,
                };
                add_todo_to_db(task, &mut todos);
                println!("Task successfully added!");
                print_all_tasks(&todos);
            }
            Action::Remove => {
                let mut input = String::new();
                print_all_tasks(&todos);
                println!("{}", "Enter number of task to remove:".red());
                read_user_input(&mut input);
                match input.trim().parse() {
                    Ok(v) => {
                        remove_task(&mut todos, v);
                        println!("Task succesfully removed!");
                    }
                    Err(_) => println!("{}", "Not a number!".red()),
                }
            }
            Action::MarkDone => {
                let mut input = String::new();
                print_all_tasks(&todos);
                println!("{}", "Enter number of task to mark as done:".green());
                read_user_input(&mut input);
                match input.trim().parse() {
                    Ok(v) => {
                        mark_task_done(&mut todos, v);
                        println!("Task succesfully marked as done!");
                    }
                    Err(_) => println!("{}", "Not a number!".red()),
                }
            }
            Action::ViewDone => {
                println!("{}", "Your done tasks are:".green());
                print_done_tasks(&todos);
            }
            Action::ViewTasks => {
                println!("{}", "Your current tasks are:".yellow());
                print_not_done_tasks(&todos);
            }
            Action::ViewAll => {
                println!("{}", "All tasks:".yellow());
                print_all_tasks(&todos);
                let mut input = String::new();
                println!("Press any key to continue ->");
                read_user_input(&mut input);
            }
        }
    }
}
