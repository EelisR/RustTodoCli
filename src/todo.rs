pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}


pub enum TodoCommand {
    Add,
    List,
    Complete,
    Uncomplete,
    Delete,
    Quit,
    Help,
    InputError
}

#[derive(Debug)]
struct InputError;

impl TodoCommand {
    pub fn from_str(s: &str) -> TodoCommand {
        match s {
            "add" => TodoCommand::Add,
            "list" => TodoCommand::List,
            "complete" => TodoCommand::Complete,
            "uncomplete" => TodoCommand::Uncomplete,
            "delete" => TodoCommand::Delete,
            "quit" => TodoCommand::Quit,
            "help" => TodoCommand::Help,
            _ => TodoCommand::InputError,
        }
    }
}

pub fn add_todo(todos: &mut Vec<Todo>, title: Option<&str>) {
    let title = match title {
        Some(title) => title,
        None => {
            println!("Please provide a title for the todo.");
            return;
        }
    };
    let todo = Todo {
        id: 1 + todos.len() as i32,
        title: title.to_string(),
        completed: false,
    };
    todos.push(todo);
    print_todos(todos)
}

fn try_parse_id(id: Option<&str>) -> i32 {
    match id {
        Some(id) => match id.parse::<i32>() {
            Ok(id) => id,
            Err(_) => {
                println!("Please provide a valid id.");
                0
            }
        },
        None => {
            println!("Please provide an id.");
            0
        }
    }
}


pub fn complete_todo(todos: &mut Vec<Todo>, id: Option<&str>) {
    let id = try_parse_id(id);
    match id {
        0 => (),
        _ => {
            if (id as usize) > todos.len() {
                println!("Please provide a valid id.");
                return;
            }
            let todo = todos.iter_mut().find(|todo| todo.id == id).unwrap();
            todo.completed = true;
            print_todos(todos)
        }
    }
}

pub fn uncomplete_todo(todos: &mut Vec<Todo>, id: Option<&str>) {
    let id = try_parse_id(id);
    match id {
        0 => (),
        _ => {
            if (id as usize) > todos.len() {
                println!("Please provide a valid id.");
                return;
            }
            let todo = todos.iter_mut().find(|todo| todo.id == id).unwrap();
            todo.completed = false;
            print_todos(todos)
        }
    }
}

pub fn delete_todo(todos: &mut Vec<Todo>, id: Option<&str>) {
    let id = try_parse_id(id);
    match id {
        0 => (),
        _ => {
            if (id as usize) > todos.len() {
                println!("Please provide a valid id.");
                return;
            }
            todos.retain(|todo| todo.id != id);
            reset_ids(todos);
            print_todos(todos)
        }
    }
}

pub fn print_todos(todos: &Vec<Todo>) {
    for todo in todos {
        if todo.completed {
            println!("{}: |X| {}", todo.id, todo.title);
        } else {
            println!("{}: | | {}", todo.id, todo.title);
        }
    }
}

pub fn reset_ids(todos: &mut Vec<Todo>) {
    for (i, todo) in todos.iter_mut().enumerate() {
        todo.id = i as i32 + 1;
    }
}