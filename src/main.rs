pub mod todo;
use todo::Todo;
use std::io;

use crate::todo::{print_todos, add_todo, uncomplete_todo, complete_todo, delete_todo, TodoCommand};

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    println!("Welcome to the Todo CLI! Type 'quit' to exit.");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.split_whitespace();
        let command_word = parts.next().unwrap();
        let command = TodoCommand::from_str(command_word);
        let word = parts.next();

        match command {
            TodoCommand::Quit => break,
            TodoCommand::Add => add_todo(&mut todos, word),
            TodoCommand::List => print_todos(&todos),
            TodoCommand::Complete => complete_todo(&mut todos, word),
            TodoCommand::Uncomplete => uncomplete_todo(&mut todos, word),
            TodoCommand::Delete => delete_todo(&mut todos, word),
            TodoCommand::Help => println!("Commands: add <Title>, list, complete <id>, delete <id>, help, quit"),
            TodoCommand::InputError => println!("Unknown command. Type 'help' for a list of commands."),
        }

    }
}
