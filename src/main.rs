use rusqlite::{params, Connection};

#[derive(Debug)]
struct Person {
    id: i32, 
    name: String, 
    data: Option<Vec<u8>>,
}

fn get_all_person(conn: &Connection) -> rusqlite::Result<()> {
    

    let mut stmt = conn.prepare("SELECT id, name, data FROM person;")?; 
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?, 
            name: row.get(1)?, 
            data: row.get(2)?
        })
    })?; 

    for person in person_iter {
        println!("Fount person {:?}", person.unwrap()); 
    }
    Ok(())
}

fn insert_person(conn: &Connection, me: Person) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)", params![me.name, me.data] 
    )?; 
    Ok(())
}

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?; 
    //let conn = Connection::open("person.db")?; 
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY, 
            name TEXT NOT NULL, 
            data BLOB 
        )", []
    )?; 

    println!("Getting all values before adding"); 
    get_all_person(&conn); 
    insert_person(&conn, Person {id: 0, name: "Farooq".to_string(), data: None});
    println!("Getting all values after adding"); 
    get_all_person(&conn); 

    

    Ok(())
}
