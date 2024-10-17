mod db;

use std::env;
use prettytable::{row, Table};
use db::{init_db, add_todo, get_todo, del_todo, complete_task, reset_db,};

fn main() -> rusqlite::Result<()>{
    let con = init_db()?;
    let args :Vec<String> = env::args().collect();
    match args[1].as_str() {
        "add" => {
            for i in 2..args.len() {
                println!("Added element {} to the todo list", args[i]);
                add_todo(&con, args[i].as_str())?;
            }
        },
        "del" | "delete" => {
            for i in 2..args.len() {
                println!("Removed element {} from the todo list", args[i]);
                del_todo(&con, args[i].parse::<u8>().expect("Not a valid u8 integer"))?;
            }
        },
        "done" => {
            for i in 2..args.len() {
                println!("Marked element {} as done from the todo list", args[i]);
                complete_task(&con, args[i].parse::<u8>().expect("Not a valid u8 integer"))?;
            }
        }
        "show" => {
            let todos = get_todo(&con)?;
            let mut table = Table::new();
            table.add_row(row!["ID", "NAME" , "COMPLETED"]);
            for todo in todos {
                table.add_row(row![todo.id, todo.name, if todo.completed {"X"} else {"O"}]);
            }
            table.printstd();
        },
        "reset" => {
            println!("Reset the todo list");
            let _ = reset_db();
        },
        _ => {
            println!("Not a valid command");
        }
    }
    Ok(())
}
