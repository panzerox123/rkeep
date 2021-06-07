use super::club::create_club;
use super::club::list_club;
use std::env;

pub fn read_args() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    if args.len() == 1 {
        eprintln!("NOT IMPLEMENTED");
        return;
    }
    match &args[1][..] {
        "list" => match &args[2][..] {
            "clubs" => {
                list_club();
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
            }
            &_ => {
                println!("Try `rkeep list help`");
            }
        },
        &_ => {
            println!("Try `rkeep help`");
        }
    }
}
