extern crate rusqlite;
use rusqlite::{params, Connection, Result};

pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

impl Task {
    // A constructor method which is also an associated function
    pub fn new(id: u32, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
        }
    }

    // An instance method (uses self)
    // pub fn complete(&mut self) {
    //     self.completed = true;
    // }

    // Associated methods (doesn't use self)
    pub fn add_task(conn: &Connection, description: &str) -> Result<()> {
        conn.execute(
            "INSERT INTO task (description, completed) VALUES (?1, ?2)",
            params![description, 0],
        )?;
        Ok(())
    }

    pub fn get_tasks(conn: &Connection) -> Result<Vec<Task>> {
        let mut stmt = conn.prepare("SELECT id, description, completed FROM task")?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                completed: row.get::<_, i32>(2)? == 1,
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    pub fn get_task_by_id(conn: &Connection, task_id: u32) -> Result<Option<Task>> {
        let mut stmt = conn.prepare("SELECT id, description, completed FROM task WHERE id =?1")?;
        let mut task_iter = stmt.query_map(params![task_id], |row| {
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                completed: row.get::<_, i32>(2)? == 1,
            })
        })?;

        if let Some(task) = task_iter.next() {
            task.map(Some)
        } else {
            Ok(None)
        }
    }

    pub fn update_task(
        conn: &Connection,
        id: u32,
        new_description: Option<&str>,
        completed: Option<bool>,
    ) -> Result<()> {
        if let Some(desc) = new_description {
            conn.execute(
                "UPDATE task SET description = ?1 WHERE id = ?2",
                params![desc, id],
            )?;
        }

        if let Some(comp) = completed {
            conn.execute(
                "UPDATE task SET completed = ?1 WHERE id = ?2",
                params![comp, id],
            )?;
        }
        Ok(())
    }

    pub fn delete_task(conn: &Connection, id: u32) -> Result<()> {
        conn.execute("DELETE FROM task WHERE id =?1", params![id])?;
        Ok(())
    }
}
