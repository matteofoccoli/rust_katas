pub struct Scoreboard {}

impl Scoreboard {
    pub fn display(&self) -> String {
        "Player A: 0, Player B: 0".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_started() {
        let scoreboard = Scoreboard {};

        let expected_result = "Player A: 0, Player B: 0";
        assert_eq!(expected_result, scoreboard.display());
    }
}
