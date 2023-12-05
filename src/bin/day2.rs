
#[derive(Debug)]
struct Sample {
    red: u32,
    green: u32,
    blue: u32
}

impl Sample {
    pub fn new() -> Sample {
        Sample {red: 0, green: 0, blue: 0}
    }
}

#[derive(Debug)]
struct Game {
    idx: u32,
    events: Vec<Sample>,
}

impl Game {
    fn min_power(&self) -> u32 {
        let min_red: u32 = self.events.iter().map(|s| s.red).max().unwrap_or(0);
        let min_green: u32 = self.events.iter().map(|s| s.green).max().unwrap_or(0);
        let min_blue: u32 = self.events.iter().map(|s| s.blue).max().unwrap_or(0);

        return min_red * min_green * min_blue;
    }
}

fn read_events(text: &str) -> Vec<Sample> {
    let mut events = vec![];
    for event_str in text.trim().split(';') {
        let mut sample = Sample::new();
        for num_and_colour in event_str.trim().split(',') {
            let mut words = num_and_colour.trim().split(' ');
            let num: u32 = words.next()
                .map(|w| w.trim())
                .and_then(|w| w.parse().ok())
                .expect("no number found");
            let colour_str = words.next()
                .expect("no colour word found")
                .trim();
            match colour_str {
                "green" => sample.green = num,
                "red" => sample.red = num,
                "blue" => sample.blue = num,
                _ => panic!("no such colour: {}", colour_str)
            }
        }
        events.push(sample);
    }
    return events;
}

fn parse_game_idx(text: &str) -> u32 {
    let mut parts = text.trim().split(' ');
    let _game_word = parts.next()
        .filter(|&w| w.trim() == "Game")
        .expect("expected literal `Game`");
    let game_idx: u32 = parts.next()
        .map(|w| w.trim())
        .and_then(|w| w.parse().ok())
        .expect("expected game id");
    return game_idx;
}

fn read_game(text: &str) -> Game {
    let mut parts = text.trim().split(':');
    let idx = parse_game_idx(parts.next().expect("expected game prefix"));
    let events = read_events(parts.next().expect("expected game description"));
    Game{events, idx}
}

fn task1() {
    let input = "day2task1.in";
    let file_content = std::fs::read_to_string(input).expect("failed to read input file");
    let mut possible_sum = 0u32;
    for line in file_content.lines() {
        let game = read_game(line);

        let mut possible = true;
        for sample in &game.events {
            if sample.red > 12 || sample.green > 13 || sample.blue > 14 {
                possible = false;
                println!("{sample:?} is not possible");
            }
        }

        if possible {
            println!("{:?} is possible", game);
            possible_sum += game.idx;
        }
    }

    println!("Sum of possible games: {}", possible_sum);
}

fn task2() {
    let input = "day2task2.in";
    let file_content = std::fs::read_to_string(input).expect("failed to read input file");

    let mut power_sum: u32 = 0;

    for line in file_content.lines() {
        let game = read_game(line);
        power_sum += game.min_power();
    }

    println!("Sum of powers is: {}", power_sum);
}

fn main() {
    task2();
}