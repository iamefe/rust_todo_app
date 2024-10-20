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
        println!("Usage: <command eg. add> <description>");
        return Ok(());
    }

    let command = &args[1];
    let description = &args[2..].join(" ");

    let mut new_task = Task::new(1, description.to_string());

    match command.as_str() {
        "add" => {
            Task::add_task(&conn, &new_task.description)?;
            new_task.complete();
            print!("{}", new_task.description);
        }
        "update" => {}
        _ => println!("Invalid command"),
    }

    let tasks = Task::get_tasks(&conn)?;
    for task in tasks {
        println!(
            "Task {}: {} [{}]",
            task.id,
            task.description,
            if task.completed { "x" } else { " " }
        );
    }

    Ok(())
}
