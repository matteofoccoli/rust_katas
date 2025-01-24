pub struct Scoreboard {}

impl Scoreboard {
    pub fn display(&self) -> String {
        "Player A: 0, Player B: 0".to_string()
    }
}

pub struct Match {
    pub first_player: Player,
    pub second_player: Player,
    pub score: Score,
}

impl Match {
    pub fn new(first_player: Player, second_player: Player) -> Self {
        Self {
            first_player,
            second_player,
            score: Score::new(Points::Love, Points::Love),
        }
    }

    pub fn first_player_scores(&mut self) {
        self.score = Score::new(Points::Fifteen, Points::Love)
    }

    pub fn second_player_scores(&mut self) {
        self.score = Score::new(Points::Love, Points::Fifteen)
    }
}

#[derive(PartialEq, Debug)]
pub struct Score {
    pub first_player_points: Points,
    pub second_player_points: Points,
}

impl Score {
    pub fn new(first_player_points: Points, second_player_points: Points) -> Self {
        Score {
            first_player_points,
            second_player_points,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Player {
    pub name: String,
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Points {
    Love,
    Fifteen,
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
        let current_match = Match::new(create_first_player(), create_second_player());

        assert_eq!(Score::new(Points::Love, Points::Love), current_match.score);
    }

    #[test]
    fn fifteen_love() {
        let mut current_match = Match::new(create_first_player(), create_second_player());

        current_match.first_player_scores();

        assert_eq!(
            Score::new(Points::Fifteen, Points::Love),
            current_match.score
        );
    }

    #[test]
    fn love_fifteen() {
        let mut current_match = Match::new(create_first_player(), create_second_player());

        current_match.second_player_scores();

        assert_eq!(
            Score::new(Points::Love, Points::Fifteen),
            current_match.score
        );
    }

    fn create_second_player() -> Player {
        let second_player = Player {
            name: "Boris".to_string(),
        };
        second_player
    }

    fn create_first_player() -> Player {
        let first_player = Player {
            name: "Pete".to_string(),
        };
        first_player
    }
}
