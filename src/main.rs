mod db;

use std::env;
use db::{init_db, add_todo, get_todo, del_todo, complete_task};

fn main() -> rusqlite::Result<()>{
    let con = init_db()?;
    let args :Vec<String> = env::args().collect();
    match args[1].as_str() {
        "add" => {
            add_todo(&con, args[2].as_str())?;
        },
        "del" | "delete" => {
            del_todo(&con, args[2].parse::<u8>().expect("Not a valid u8 integer"))?;
        },
        "done" => {
            complete_task(&con, args[2].parse::<u8>().expect("Not a valid u8 integer"))?;
        }
        "show" => {
            let todos = get_todo(&con)?;
            for todo in todos {
                println!("{}\t{}", todo.id, todo.name);
            }
        },
        _ => {
            println!("Not a valid command");
        }
    }
    Ok(())
}
