use rusqlite::{params, Connection, Result};

#[derive(Debug)]

pub struct Club {
    id: u32,
    club_name: String,
    club_path: String
}

pub fn create_club(club_name: String, path: String) {
    let connection = Connection::open("./testdb");
    //let connection = Connection::open_in_memory();
    let connection = match connection{
        Ok(c) => c,
        Err(err) => panic!("Error connecting to database : {}", err)
    };
    println!("clubname: {} ; path: {}", club_name, path);
    let new_club = Club {
        id: 0,
        club_name: club_name,
        club_path: path
    };
    let table_create_res = connection.execute(
        "
        CREATE TABLE IF NOT EXISTS clubs (id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE, path TEXT NOT NULL UNIQUE);
    ", []);
    match table_create_res{
        Ok(_) => {},
        Err(err) => {eprintln!("Table Creation Failed : {}", err);}
    };
    let insert_club_res = connection.execute(
        "
        INSERT INTO clubs (name, path) VALUES(?1, ?2);
    ", params![new_club.club_name, new_club.club_path]);
    match insert_club_res{
        Ok(_)=> {},
        Err(err) => {eprintln!("Failed to create Club : {}", err);}
    };
}

pub fn list_club() -> Vec<Club>{
    let connection = Connection::open("./testdb");
    //let connection = Connection::open_in_memory();
    let connection = match connection{
        Ok(c) => c,
        Err(err) => panic!("Error connecting to database : {}", err)
    };
    let mut statement = connection.prepare("select * from clubs").unwrap();
    let club_iterable = statement.query_map([], |row| {
        Ok(Club{
            id: row.get(0)?,
            club_name: row.get(1)?,
            club_path: row.get(2)?
        })
    });
    let club_iterable = match club_iterable {
        Ok(ci) => ci,
        Err(err) => panic!("Error loading data : {}", err)
    };
    let mut club_list: Vec<Club> = vec!{};
    println!("ID\tNAME\tPATH");
    for club in club_iterable {
        let temp = club.unwrap();
        println!("{}\t{}\t{}", temp.id, temp.club_name, temp.club_path);
        club_list.push(temp);
    }
    return club_list;
}

fn delete_club() {}

fn rename_club() {}
