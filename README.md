# Task Manager CLI

A modular command-line task management application built in Rust. This project demonstrates core Rust concepts including module organization, error handling with Result, and optional data management with Option.

## Project Structure

The source code is organized into a nested module hierarchy to ensure clear separation of concerns:

- **src/main.rs**: The application entry point. Handles the primary execution flow and module declarations.
- **src/tasks.rs**: The parent module (bridge) that manages and re-exports sub-modules.
- **src/tasks/models.rs**: Defines the data structures, including the Task struct and Priority enum.
- **src/tasks/logic.rs**: Contains the business logic for creating tasks and manipulating their state.

## Core Concepts Implemented

### Error Handling
The project utilizes the `Result<T, E>` type to handle task validation. If a task is created with an empty name, the system returns a descriptive `Err(String)` instead of panicking.

### Option Type
The `description` field of a Task is wrapped in an `Option<String>`. This allows the application to handle tasks that may or may not have additional metadata without using null values.

### Ownership and Borrowing
The application demonstrates the use of mutable references (`&mut`) to update the completion status of tasks stored within a collection.

### Functional Patterns
Closures and the `.map()` method are used to transform data types safely within the Option and Result contexts.

## Usage

### Building the Project
To compile the project, navigate to the root directory and run:
```bash
cargo build
