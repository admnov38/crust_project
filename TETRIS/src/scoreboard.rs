use std::ffi::CString;
use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};

pub enum ScoreUpdated {
    NewHighScore,
    NotImproved
}

pub struct HighScore {
    username: String,
    score: i32,
}

pub struct ScoreBoard {
    pub highscores: Vec<HighScore>,
    pub formatted_highscores: CString
}


impl ScoreBoard {

    pub fn new(filename: &str) -> Self {
        let highscores = Self::read_highscores_from_file(filename);

        ScoreBoard { 
            highscores: highscores, 
            formatted_highscores: CString::new("").unwrap() 
        }
    }

    fn read_highscores_from_file(filename: &str) -> Vec<HighScore> {
        let file = File::open(filename).expect("Failed to open file");
        let reader = BufReader::new(file);

        reader.lines()
              .filter_map(|line| line.ok())
              .map(|line| {
                   let parts: Vec<&str> = line.split(": ").collect();
                   HighScore {
                    username: parts[0].to_string(),
                    score: parts[1].parse().unwrap(),
                }
               })
              .collect()
    }

    pub fn format_highscores(&mut self, delimiter: &str) {
        let formatted = self.highscores
                            .iter()
                            .map(|highscore| format!("{}: {}", highscore.username, highscore.score))
                            .collect::<Vec<_>>()
                            .join(delimiter);

        self.formatted_highscores = CString::new(formatted).unwrap();
    }

    pub fn update_highscore(&mut self, username: &str, score: i32) -> ScoreUpdated {
        let mut result = ScoreUpdated::NewHighScore;

        if let Some(highscore) = self.highscores.iter_mut().find(|highscore| highscore.username == username) {
            if score > highscore.score {
                highscore.score = score;
            } else {
                result = ScoreUpdated::NotImproved;                
            }
        } else {
            self.highscores.push(HighScore { username: username.to_string(), score });
        }
        
        self.highscores.sort_by_key(|score| -score.score);
        self.format_highscores(";");
        return result
    }

    pub fn save_highscores(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename.to_string())?;
        let mut writer = BufWriter::new(file);

        for highscore in &self.highscores {
            writer.write_all(highscore.username.as_bytes())?;
            writer.write_all(b": ")?;
            writer.write_all(highscore.score.to_string().as_bytes())?;
            writer.write_all(b"\n")?;
        }

        Ok(())
    }

    pub fn get_users_highscore(&self, username: &str) -> Option<i32> {
        if let Some(highscore) = self.highscores.iter().find(|h| h.username == username) {
            Some(highscore.score)
        } else {
            None
        }
    }

}
