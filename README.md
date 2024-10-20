# Todo App in Rust

This is a simple command-line Todo application written in Rust, using the `rusqlite` library for SQLite database operations. The app supports adding, updating, viewing, and deleting tasks.

## Features

-   **Add a Task**: Add a new task with a description.
-   **View All Tasks**: List all tasks in the database.
-   **View a Task**: View details of a single task by its ID.
-   **Update a Task**: Update the description or completion status of a task.
-   **Delete a Task**: Remove a task from the database.

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/todo_app.git
    cd todo_app
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. Move the executable to a directory in your PATH (optional for easier access):
    ```bash
    sudo mv target/release/todo_app /usr/local/bin/todo_app
    ```

## Usage

### Add a Task

```bash
todo_app add "Your task description here"
```

### View All Tasks

```bash
todo_app tasks
```

### View a Task by ID

```bash
todo_app task <id>
```

### Update a Task

```bash
todo_app update <id> --desc "New description" --completed true
```

### Delete a Task

```bash
todo_app delete <id>
```

## Code Overview

### Task Struct and Methods (`task.rs`)

-   **Task Struct**: Defines the task with `id`, `description`, and `completed` fields.
-   **Methods**:
    -   `new`: Creates a new task.
    -   `add_task`: Adds a task to the database.
    -   `get_tasks`: Retrieves all tasks from the database.
    -   `get_task_by_id`: Retrieves a task by its ID.
    -   `update_task`: Updates the description or completion status of a task.
    -   `delete_task`: Deletes a task by its ID.

### Main Application (`main.rs`)

-   **setup_database**: Initializes the database and creates the `task` table if it doesn't exist.
-   **Command Handling**:
    -   `add`: Adds a new task.
    -   `tasks`: Lists all tasks.
    -   `task`: Views a single task by ID.
    -   `update`: Updates a task with new description or completion status.
    -   `delete`: Deletes a task by ID.

### Dependencies (`Cargo.toml`)

-   **rusqlite**: SQLite library for Rust.

## Example

```bash
# Add a task
todo_app add "Finish reading Rust by Example"

# View all tasks
todo_app tasks

# View a single task by ID
todo_app task 1

# Update a task
todo_app update 1 --desc "Complete the Rust project" --completed false

# Delete a task
todo_app delete 1
```

## Notes

-   Ensure that SQLite is installed on your system.
-   Use quotes for multi-word task descriptions.
