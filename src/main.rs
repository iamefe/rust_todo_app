mod task;
use task::Task;
extern crate rusqlite;
use rusqlite::{Connection, Result};
use std::env;

fn setup_database() -> Result<Connection> {
    let conn = Connection::open("tasks.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
                  id              INTEGER PRIMARY KEY,
                  description     TEXT NOT NULL,
                  completed       INTEGER NOT NULL
                  )",
        [],
    )?;
    Ok(conn)
}

fn main() -> Result<()> {
    let conn = setup_database()?;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: <command> <description>");
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            let description = &args[2..].join(" ");
            let new_task = Task::new(1, description.to_string());
            Task::add_task(&conn, &new_task.description)?;
            println!("Added task: {}", description);
        }
        "tasks" => {
            let tasks = Task::get_tasks(&conn)?;
            if tasks.is_empty() {
                println!("No tasks were found!")
            } else {
                for task in tasks {
                    println!(
                        "Task {}: {} [{}]",
                        task.id,
                        task.description,
                        if task.completed { "x" } else { " " }
                    );
                }
            }
        }
        "task" => {
            // Method 1:
            if args.len() != 3 {
                println!("Usage: task <id>");
            } else {
                let id: u32 = args[2].parse().expect("ID must be a number");
                let task = Task::get_task_by_id(&conn, id)?;
                if let Some(task) = task {
                    println!(
                        "Task {}: {} [{}]",
                        task.id,
                        task.description,
                        if task.completed { "x" } else { " " }
                    );
                } else {
                    println!("Task not found!");
                }
            }
        }
        "update" => {
            if args.len() < 4 {
                println!(
                    "Usage (use quotes for the --desc arg): update <id> [--desc|-d <new description>] [--completed|-c <true/false>]"
                );
            } else {
                let id: u32 = args[2].parse().expect("ID must be a number");

                let mut new_description = None;
                let mut completed = None;
                let mut i = 3;

                /*

                By Oserefemen
                How the while loop here works:
                1. There must be a flag at index 3.
                2. If none is found our `_ => ...` line runs.
                3. If one is found, i ends up being incremented twice which should put us in the position of the next flag.
                4. Then the loop attempts to run again:
                5. The loop won't run if the new value of i is greater than the array's length indicating that we've reach the end of the loop.

                                 */

                while i < args.len() {
                    match args[i].as_str() {
                        "--desc" | "-d" => {
                            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                                new_description = Some(args[i + 1].clone());
                            } else {
                                println!("Expected value for --desc|-d");
                                return Ok(());
                            }
                        }
                        "--completed" | "-c" => {
                            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                                completed = Some(
                                    args[i + 1]
                                        .parse::<bool>()
                                        .expect("Completed must be true or false"),
                                );
                            } else {
                                println!("Expected value for --completed|-c");
                                return Ok(());
                            }
                        }
                        _ => {
                            println!("Unknown flag: {}", args[i]);
                            return Ok(());
                        }
                    }
                    i += 2;
                }

                Task::update_task(&conn, id, new_description.as_deref(), completed)?;
                println!("Updated task {}", id);
            }
        }

        "delete" => {
            if args.len() != 3 {
                println!("Usage: delete <id>");
            } else {
                let id: u32 = args[2].parse().expect("ID must be a number");
                Task::delete_task(&conn, id)?;
                println!("Deleted task {}", id);
            }
        }

        _ => println!("Invalid command"),
    }

    Ok(())
}
