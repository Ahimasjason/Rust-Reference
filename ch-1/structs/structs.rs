// Unit Structs
// Zero sized Value

#[derive(Debug)]
struct Dummy;

// tuple struct

struct Color(u8, u8, u8);

// Usual struct

struct Player {
    name: String,
    iq: u8,
    friends: u8,
    score: u16,
}

impl Player {
    fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            iq: 100,
            friends: 100,
            score: 100,
        }
    }

    fn get_friends(&self) -> u8 {
        self.friends
    }

    fn set_friends(&mut self, count: u8) {
        self.friends = count;
    }
}

fn bump_player_score(mut player: Player, score: u16) {
    player.score += score;
    println!("Name : {}", player.name);
    println!("IQ {}", player.iq);
    println!("Score {}", player.score);

    println!("Player Friends {:?}", player.get_friends());
    player.set_friends(120);
    println!(
        "Player Friends  after set tp 120 {:?}",
        player.get_friends()
    );
    println!();
}

fn main() {
    let orange = Color(255, 165, 0);

    let dummy = Dummy;

    let Color(a, b, c) = orange;
    println!("Color A : {} , B {} , C {} ", a, b, c);
    println!();

    println!("Dummy {:?}", dummy);

    let player = Player {
        name: "Alice".to_string(),
        iq: 117,
        friends: 134,
        score: 1129,
    };

    bump_player_score(player, 120);
}
