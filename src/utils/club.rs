use rusqlite::{params, Connection, Result};
use std::fs;

#[derive(Debug)]

pub struct Club {
    pub id: u32,
    pub club_name: String,
    pub club_path: String,
}

pub fn open_connection() -> Connection {
    let connection = Connection::open("./testdb");
    let connection = match connection {
        Ok(c) => c,
        Err(err) => panic!("Error connecting to database : {}", err),
    };
    return connection;
}

pub fn create_club(club_name: String, path: &mut String) {
    let connection = open_connection();
    println!("clubname: {} ; path: {}", club_name, path);
    let new_club = Club {
        id: 0,
        club_name: club_name,
        club_path: path,
    };
    let table_create_res = connection.execute(
        "
        CREATE TABLE IF NOT EXISTS clubs (id INTEGER PRIMARY KEY, name TEXT NOT NULL UNIQUE, path TEXT NOT NULL UNIQUE);
    ", []);
    match table_create_res {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Table Creation Failed : {}", err);
        }
    };
    let insert_club_res = connection.execute(
        "
        INSERT INTO clubs (name, path) VALUES(?1, ?2);
    ",
        params![new_club.club_name, new_club.club_path],
    );
    match insert_club_res {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to create Club : {}", err);
            return;
        }
    };
    let path = path.as_str();
    fs::create_dir_all(path);
}

pub fn list_club() -> Vec<Club> {
    let connection = open_connection();
    let mut statement = connection.prepare("select * from clubs").unwrap();
    let club_iterable = statement.query_map([], |row| {
        Ok(Club {
            id: row.get(0)?,
            club_name: row.get(1)?,
            club_path: row.get(2)?,
        })
    });
    let club_iterable = match club_iterable {
        Ok(ci) => ci,
        Err(err) => panic!("Error loading data : {}", err),
    };
    let mut club_list: Vec<Club> = vec![];
    for club in club_iterable {
        let temp = club.unwrap();
        club_list.push(temp);
    }
    return club_list;
}

pub fn delete_club(clubname: String) {
    let connection = open_connection();
    let del_club_res = connection.execute("delete from clubs where name = ?1", params![clubname]);
    match del_club_res {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error deleting club : {}", err);
        }
    }
}

pub fn update_club(clubname: String, new_clubname: String, new_path: String) {
    let connection = open_connection();
    let update_club_res = connection.execute(
        "update clubs set name = ?1 , path = ?2 where name = ?3",
        params![new_clubname, new_path, clubname],
    );
    match update_club_res {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error updating club info : {}", err);
        }
    }
}
