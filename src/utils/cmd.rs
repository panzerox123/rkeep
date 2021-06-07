use super::club::{Club, create_club, list_club, delete_club, update_club};
use std::env;

pub fn read_args() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("NOT IMPLEMENTED");
        return;
    }
    match &args[1][..] {
        "list" => match &args[2][..] {
            "clubs" => {
                list_club_handler();
            }
            &_ => {
                println!("Try `rkeep list help`");
            }
        },
        "create" => match &args[2][..] {
            "club" => {
                let club = args[3].as_str();
                let path = args[4].as_str();
                create_club(club.to_string(), path.to_string());
            },
            &_ => {
                println!("Try `rkeep list help`");
            }
        },
        "delete" => match &args[2][..]{
            "club" => {
                let club = args[3].as_str();
                delete_club(club.to_string());   
            },
            &_ => {
                println!("Try `rkeep list help`");
            }
        }
        &_ => {
            println!("Try `rkeep help`");
        }
    }
}

fn list_club_handler(){
    let data: Vec<Club> = list_club();
    println!("ID\tNAME\tPATH");
    for club in data {
        println!("{}\t{}\t{}", club.id, club.club_name, club.club_path);
    }
}
