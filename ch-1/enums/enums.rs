// enumns can be defined with or without data contained in them
// data contained can be any type of primitive type, structs, an enum
// recursuive enum type need to be behind a pointer(Box,Rc)

#[derive(Debug)]
enum Direction {
    N,
    E,
    W,
    S,
}

enum PlayerAction {
    Move { direction: Direction, speed: u8 },
    Wait,
    Attack(Direction),
}

fn main() {
    let sim_player = PlayerAction::Move {
        direction: Direction::N,
        speed: 2,
    };

    let rs = match sim_player {
        PlayerAction::Wait => println!("User is waiting !!"),
        PlayerAction::Move { direction, speed } => {
            println!(
                "Player wants to move in direction {:?} with speed {:?}",
                direction, speed
            );
            ()
        }

        PlayerAction::Attack(direction) => println!("Attacking direction {:?}", direction),
    };

    println!("rs {:?}", rs);
}
