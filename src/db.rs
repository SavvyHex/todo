use rusqlite::{params, Connection, Result};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Task {
    pub id : u8,
    pub name : String,
    pub completed : bool,
}

pub fn init_db () -> Result<Connection> {
    let con = Connection::open("todo.db")?;
    con.execute(
        "CREATE TABLE IF NOT EXISTS todo (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        task TEXT NOT NULL,
        completed BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(con)
}

pub fn add_todo(con:&Connection, task:&str) -> Result<()> {
    con.execute("INSERT INTO todo (task) VALUES (?1)", params![task])?;
    Ok(())
}

pub fn get_todo(con:&Connection) -> Result<Vec<Task>> {
    let mut table = con.prepare("SELECT * FROM todo")?;
    let task_iter = table.query_map([], |row| {
        Ok(Task{
            id: row.get(0)?,
            name : row.get(1)?,
            completed : row.get(2)?
        })
    })?;

    task_iter.collect()
}

pub fn del_todo(con:&Connection, id:u8) -> Result<()> {
    con.execute("DELETE FROM todo WHERE id=(?1)", params![id])?;
    Ok(())
}

pub fn complete_task(con:&Connection, id:u8) -> Result<()> {
    con.execute("UPDATE todo SET completed=1 WHERE id=(?1)", params![id])?;
    Ok(())
}

pub fn reset_db() -> std::io::Result<()> {
    std::fs::remove_file("todo.db")?;
    Ok(())
}