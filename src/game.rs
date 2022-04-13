use spreadsheet_ods::Sheet;
use rand::prelude::SliceRandom;

const EMPTY_FIELD: &str = "-";

pub struct Game {
    pub title: String,
    pub year: u32,
    pub console: String,
    pub genre: String,
    pub quality: String,
    pub played: String,
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Game {
            title: self.title.clone(),
            year: self.year,
            console: self.console.clone(),
            genre: self.genre.clone(),
            quality: self.quality.clone(),
            played: self.played.clone(),
        }
    }
}

impl Game {
    pub fn random(games: Vec<Game>) -> Game {
        let game= match games.choose(&mut rand::thread_rng()) {
            None => panic!("Error while selecting game"),
            Some(game) => game,
        };

        game.clone()
    }

    pub fn parse_game(spreadsheet: &Sheet, i: u32) -> Game {
        Game {
            title: spreadsheet
                .value(i + 1, 0)
                .as_str_or(EMPTY_FIELD)
                .to_string(),
            year: spreadsheet.value(i + 1, 1).as_u32_or(0000),
            console: spreadsheet
                .value(i + 1, 2)
                .as_str_or(EMPTY_FIELD)
                .to_string(),
            genre: spreadsheet
                .value(i + 1, 3)
                .as_str_or(EMPTY_FIELD)
                .to_string(),
            quality: spreadsheet
                .value(i + 1, 4)
                .as_str_or(EMPTY_FIELD)
                .to_string(),
            played: spreadsheet
                .value(i + 1, 5)
                .as_str_or(EMPTY_FIELD)
                .to_string(),
        }
    }
}