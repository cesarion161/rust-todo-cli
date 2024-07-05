# To-Do List CLI Application

This is a command-line interface (CLI) application for managing a to-do list. The application allows you to add items to
the to-do list, list all items, and save the list to a JSON file.

## Features

- Add items to the to-do list
- List all to-do items
- Save the to-do list to a JSON file
- Load the to-do list from a JSON file

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Make sure you have Rust and Cargo installed)

## Setup

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/todo-list-cli.git
   cd todo-cli

2. Install dependencies:
    ```sh
   cargo build
   
## Usage
   ```sh
   cargo run
   ```

### Commands
To add new item to the list:
```shell
-- add --item "Your new to-do item"
```
To list all existing items:
```shell
-- list
```
To persist all items from the list to the file:
```shell
-- save
```
All items are saved to file and then being read after the restart.
