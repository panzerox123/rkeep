use super::club::open_connection;
use rusqlite::params;

pub fn get_path(clubname: String) -> String {
    let connection = open_connection();
    let path = connection.query_row("select path from clubs where name = ?1", params![clubname], |row| {
            row.get(0)
    });
    match path{
        Ok(p) => p,
        Err(err) => panic!("Could not find club : {}", err)
    }
}

pub fn create_note(notename: String, clubname: String){
    let path = get_path(clubname);
    
}