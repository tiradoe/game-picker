use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use spreadsheet_ods::{Sheet, WorkBook};

const EMPTY_FIELD: &str = "-";

struct Game {
    title: String,
    year: u32,
    console: String,
    genre: String,
    quality: String,
    played: String,
}

impl Game {
    pub fn random(&self) -> &String {
        &self.title
    }

    pub fn parse_game(spreadsheet: &Sheet, i: u32) -> Game {

        Game {
            title: spreadsheet.value(i + 1, 0)
                .as_str_or(EMPTY_FIELD)
                .to_string(),
            year: spreadsheet.value(i + 1, 1)
                .as_u32_or(0000),
            console: spreadsheet.value(i + 1, 2)
                .as_str_or(EMPTY_FIELD)
                .to_string(),
            genre: spreadsheet.value(i + 1, 3)
                .as_str_or(EMPTY_FIELD)
                .to_string(),
            quality: spreadsheet.value(i + 1, 4)
                .as_str_or(EMPTY_FIELD)
                .to_string(),
            played: spreadsheet.value(i + 1, 5)
                .as_str_or(EMPTY_FIELD)
                .to_string(),
        }
    }
}

fn main() {
    let wb = get_workbook();
    let spreadsheet = wb.sheet(0);

    let mut headers: Vec<&str> = Vec::new();
    let mut games: Vec<Game> = Vec::new();
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

        i = i + 1;
    }

    for header in headers {
        println!("{}", header);
    }

    for game in games {
        println!("Title: {}, Year: {}", game.title, game.year);
    }
}

fn get_workbook() -> WorkBook {
    let file = match File::open("config.txt") {
        Err(err) => panic!("Could not find config file: {}", err),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let mut path_string: String = String::from("");

    for (_index, line) in reader.lines().enumerate() {
        let line: String = line.expect("Error: Could not open config file.");
        path_string = line;
    }

    let path = Path::new(&path_string);

    let wb = if path.exists() {
        match spreadsheet_ods::read_ods(path) {
            Err(err) => panic!("Error: {}", err),
            Ok(workbook) => workbook,
        }
    } else {
        panic!("Error: Spreadsheet not found!")
    };

    wb
}