# Rust Todo App

## Overview

This Rust Todo App is a simple command-line to-do list application built in
Rust. It allows users to manage their tasks interactively. Users can add,
delete, select, deselect, and toggle the completion status of to-do items.

## Modules

The application is organized into the following modules:

-   `cli`: Handles the command-line interface and screen updates.
-   `todo_item`: Defines the structure and methods for individual to-do items.
-   `todo_list`: Manages a collection of to-do items, including operations like
    add, delete, and toggle completion.
-   `view`: Manages the current view state of the application (e.g., active or
    inactive).

## Features

-   Interactive task management.
-   Adding new to-do items.
-   Deleting existing to-do items.
-   Selecting and deselecting to-do items.
-   Toggling the completion status of to-do items.
-   Viewing active tasks.

## Usage

To use the app, run it in your terminal. Once the app is running, it listens for
specific key presses to perform actions:

-   `q`: Quit the application.
-   `e`: Toggle the current view.
-   `s`: Select or deselect a to-do item.
-   `d`: Delete the selected to-do item.
-   `a`: Add a new to-do item.
-   `Space`: Toggle the completion status of the selected to-do item.

## Dependencies

-   `crossterm`: Used for handling terminal input events.

## Building and Running

To build and run the application, ensure you have Rust and Cargo installed. Then
, use the following commands:

1. Clone the repository.
2. Navigate to the project directory.
3. Run `cargo build` to build the project.
4. Run `cargo run` to start the application.
