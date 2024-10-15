mod db;

use db::{init_db, add_todo, get_todo};

fn main() -> rusqlite::Result<()> {
    let con = init_db()?;
    let _ = add_todo(&con, "The todo list");
    let todos = get_todo(&con);
    if let Ok(todo) = todos {
        print!("{:?}", todo);
    }

    Ok(())
}
