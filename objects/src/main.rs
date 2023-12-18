fn main() {
    let mut user1: Player = build_player(
        String::from("Joseph Langford"), 
        String::from("joseph.langford@netscape.com"), 
        String::from("misterguruman"), 
        65
    );

    user1.add_score(53);

    println!("{} ({})'s score is {}", user1.name, user1.email, user1.score);
}

fn build_player(name: String, email: String, username: String, score: i32) -> Player {
    Player {
        name,
        email, 
        username,
        score,
    }
}

struct Player {
    name: String,
    email: String,
    username: String,
    score: i32
}

impl Player {
    fn add_score(&mut self, score: i32) {
        self.score += score;
    }
}