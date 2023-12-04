
struct Sample {
    red: u32,
    green: u32,
    blue: u32
}

struct Game {
    events: Vec<Sample>
}

fn read_game(text: &str) -> Game {
    let mut game = Game{events: vec![]};
    for event_str in text.split(';') {
        for num_and_colour in event_str.trim().split(',') {
            let words = num_and_colour.trim().split(' ').collect::<Vec<_>>();
            if words.len() != 2 {
                panic!("incorrect phrasing: {}", num_and_colour);
            }
        }
    }
    return game;
}

fn task1() {

}

fn main() {
}