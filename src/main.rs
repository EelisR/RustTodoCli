pub mod todo;
use todo::Todo;
use std::io;

use crate::todo::{print_todos, add_todo, uncomplete_todo, complete_todo, delete_todo};

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    println!("Welcome to the Todo CLI! Type 'quit' to exit.");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let word = parts.next();
        match command {
        "quit" => break,
            "add" => add_todo(&mut todos, word),
            "list" => print_todos(&todos),
            "uncomplete" => uncomplete_todo(&mut todos, word),
            "complete" => complete_todo(&mut todos, word),
            "delete" => delete_todo(&mut todos, word),
            "help" => println!("Commands: add <Title>, list, complete <id>, delete <id>, help, quit"),
            _ => println!("Unknown command. Type 'help' for a list of commands."),
        }

    }
}
