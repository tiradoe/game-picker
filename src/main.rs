mod game;

use game::Game;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use spreadsheet_ods::WorkBook;

const EMPTY_FIELD: &str = "-";

fn main() {
    let wb = get_workbook();
    let spreadsheet = wb.sheet(0);

    let mut headers: Vec<&str> = Vec::new();
    let mut games: Vec<game::Game> = Vec::new();
    let mut i = 0;

    loop {
        let header = spreadsheet.value(0, i).as_str_or(EMPTY_FIELD);
        let game = Game::parse_game(spreadsheet, i);

        if header == EMPTY_FIELD && game.title == EMPTY_FIELD {
            break;
        } else if header != EMPTY_FIELD {
            headers.push(header);
        }

        if game.title != EMPTY_FIELD {
            games.push(game);
        }

        i += 1;
    }

    let random_game = Game::random(games);

    println!("{}", random_game.title);
}

fn get_workbook() -> WorkBook {
    let file = match File::open("config.txt") {
        Err(err) => panic!("Could not find config file: {}", err),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let mut path_string: String = String::from("");

    for (_index, line) in reader.lines().enumerate() {
        let line: String = match line {
            Err(err) => panic!("Could not find config file: {}", err),
            Ok(data) => data,
        };
        path_string = line;
    }

    let path = Path::new(&path_string);

    let wb = if path.exists() {
        match spreadsheet_ods::read_ods(path) {
            Err(err) => panic!("Error: {}", err),
            Ok(workbook) => workbook,
        }
    } else {
        panic!("Spreadsheet not found!")
    };

    wb
}
