pub struct Scoreboard {}

impl Scoreboard {
    pub fn display(&self) -> String {
        "Player A: 0, Player B: 0".to_string()
    }
}

pub struct Match {}

impl Match {
    pub fn score(&self) -> Score {
        return Score {
            first_player: Player {
                name: "Pete".to_string(),
            },
            first_player_score: Points::Zero,
            second_player: Player {
                name: "Boris".to_string(),
            },
            second_player_score: Points::Zero,
        };
    }
}

#[derive(PartialEq, Debug)]
pub struct Score {
    pub first_player: Player,
    pub first_player_score: Points,
    pub second_player: Player,
    pub second_player_score: Points,
}

#[derive(PartialEq, Debug)]
pub struct Player {
    pub name: String,
}

#[derive(PartialEq, Debug)]
pub enum Points {
    Zero,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoreboard_displays_initial_score() {
        let scoreboard = Scoreboard {};

        let expected_result = "Player A: 0, Player B: 0";
        assert_eq!(expected_result, scoreboard.display());
    }

    #[test]
    fn match_has_started() {
        let current_match = Match {};

        let expected_score = Score {
            first_player: Player {
                name: "Pete".to_string(),
            },
            first_player_score: Points::Zero,
            second_player: Player {
                name: "Boris".to_string(),
            },
            second_player_score: Points::Zero,
        };
        assert_eq!(expected_score, current_match.score());
    }
}
