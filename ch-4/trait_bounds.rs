struct Game;

struct Enemy;
struct Hero;

trait Loadable {
    fn init(&self);
}

impl Loadable for Enemy {
    fn init(&self) {
        println!("Initilizing Enemy");
    }
}

impl Loadable for Hero {
    fn init(&self) {
        println!("Initilizing Hero");
    }
}

impl Game {
    fn load<T: Loadable>(&self, entity: T) {
        entity.init();
    }
}
fn main() {
    let game = Game;
    game.load(Enemy);
    game.load(Hero);
}
