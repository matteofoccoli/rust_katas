#[derive(PartialEq, Debug)]
pub struct Score {
    player_1_points: Points,
    player_2_points: Points,
}

impl Score {
    fn new(player_1_points: Points, player_2_points: Points) -> Self {
        Self {
            player_1_points,
            player_2_points,
        }
    }

    fn next_for(&self, scorer: Scorer) -> Score {
        match scorer {
            Scorer::Player1 => {
                if self.player_2_points <= Points::Forty && self.player_1_points < Points::Forty {
                    Score::new(self.player_1_points.next(), self.player_2_points.clone())
                } else {
                    todo!()
                }
            }
            Scorer::Player2 => Score::new(Points::Love, Points::Fifteen),
        }
    }
}

pub enum Scorer {
    Player1,
    Player2,
}

#[derive(PartialEq, Debug, PartialOrd, Clone)]
pub enum Points {
    Love,
    Fifteen,
    Thirty,
    Forty,
    Adv,
    Deuce,
    Game,
}

impl Points {
    fn next(&self) -> Points {
        match self {
            Points::Love => Points::Fifteen,
            Points::Fifteen => Points::Thirty,
            Points::Thirty => Points::Forty,
            Points::Forty => Points::Adv,
            Points::Adv => Points::Game,
            Points::Deuce => Points::Adv,
            Points::Game => Points::Game,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_score_is_15_0() {
        let score = Score::new(Points::Love, Points::Love);

        assert_eq!(
            Score::new(Points::Fifteen, Points::Love),
            score.next_for(Scorer::Player1)
        )
    }

    #[test]
    fn new_score_is_0_15() {
        let score = Score::new(Points::Love, Points::Love);

        assert_eq!(
            Score::new(Points::Love, Points::Fifteen),
            score.next_for(Scorer::Player2)
        )
    }

    #[test]
    fn new_score_is_30_0() {
        let mut score = Score::new(Points::Love, Points::Love);
        score = score.next_for(Scorer::Player1);

        assert_eq!(
            Score::new(Points::Thirty, Points::Love),
            score.next_for(Scorer::Player1)
        )
    }
}
