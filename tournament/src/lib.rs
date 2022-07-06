use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::Write as _;

#[derive(Default, Debug)]
struct Score {
    team: String,
    wins: u32,
    losses: u32,
    draws: u32,
}

impl Score {
    fn new(team: String) -> Self {
        Self {
            team,
            ..Default::default()
        }
    }
    fn inc_win(&mut self) {
        self.wins += 1;
    }
    fn inc_loss(&mut self) {
        self.losses += 1;
    }
    fn inc_draw(&mut self) {
        self.draws += 1;
    }
    fn matches_played(&self) -> u32 {
        self.wins + self.losses + self.draws
    }
    fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }
    fn merge(&mut self, other: Score) {
        self.team = other.team;
        self.wins += other.wins;
        self.losses += other.losses;
        self.draws += other.draws;
    }
}

pub fn tally(match_results: &str) -> String {
    let scores = match_results
        .lines()
        .flat_map(|line| {
            let result: Vec<&str> = line.split(';').map(|s| s.trim()).collect();
            assert_eq!(3, result.len(), "Invalid line: {}", line);
            new_scores(&result).into_iter()
        })
        .fold(HashMap::<String, Score>::new(), |mut acc, s| {
            acc.entry(s.team.clone()).or_default().merge(s);
            acc
        });

    formatted_scores(scores)
}

fn new_scores(result: &[&str]) -> Vec<Score> {
    match result[2] {
        "win" => {
            let mut winner = Score::new(result[0].to_string());
            let mut loser = Score::new(result[1].to_string());
            winner.inc_win();
            loser.inc_loss();

            vec![winner, loser]
        }
        "loss" => {
            let mut winner = Score::new(result[1].to_string());
            let mut loser = Score::new(result[0].to_string());
            winner.inc_win();
            loser.inc_loss();

            vec![winner, loser]
        }
        _ => {
            let mut team1 = Score::new(result[0].to_string());
            let mut team2 = Score::new(result[1].to_string());
            team1.inc_draw();
            team2.inc_draw();

            vec![team1, team2]
        }
    }
}

fn formatted_scores(scores: HashMap<String, Score>) -> String {
    let mut output = String::new();
    let _ = write!(
        output,
        "{: <30} |{: >3} | {: >2} | {: >2} | {: >2} | {: >2}",
        "Team", "MP", "W", "D", "L", "P"
    );

    let mut results: Vec<Score> = scores.into_iter().map(|(_, score)| score).collect();
    results.sort_by_key(|s| (Reverse(s.points()), s.team.clone()));

    for score in results {
        let _ = write!(
            output,
            "\n{: <30} |{: >3} | {: >2} | {: >2} | {: >2} | {: >2}",
            score.team.to_string(),
            score.matches_played(),
            score.wins,
            score.draws,
            score.losses,
            score.points()
        );
    }
    output
}
