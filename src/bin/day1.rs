use std::fs::read_to_string;

fn task1() {
    let input = "day1.in";
    let file_content = read_to_string(input).unwrap();
    let lines = file_content.lines();

    let mut total_value = 0;
    for line in lines {
        let digits: Vec<i32> = line.as_bytes()
            .iter().filter(|&b| b.is_ascii_digit())
            .map(|b| (b - 48) as i32).collect();
        if digits.len() < 1 {
            panic!("no digits on line {}", line);
        }
        let line_value = 10 * digits.first().expect("no digits") + digits.last().expect("no digits");
        println!("{} => {}", line, line_value);
        total_value += line_value;
    }

    println!("total value: {}", total_value);
}

fn get_text_digit(text: &str, start: usize) -> Option<u32> {
    let pairs = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut current = None;
    for (word, value) in pairs {
        if text.len() - start < word.len() {
            continue;
        }
        if text[start..start + word.len()] == *word {
            current = current.or_else(|| Some(value));
        }
    }

    return current;
}

fn task2() {
    let input = "day1task2.in";
    let file_content = read_to_string(input).unwrap();

    let mut total_value = 0;
    for line in file_content.lines() {

        let digits: Vec<u32> = line.char_indices()
            .filter_map(|(idx, c)| c
                .to_digit(10)
                .or_else(|| get_text_digit(line, idx)))
            .collect();
        if digits.is_empty() {
            panic!("no digits on line {}", line);
        }
        let line_value = 10 * digits.first().expect("no digits") + digits.last().expect("no digits");
        println!("{} => {}", line, line_value);
        total_value += line_value;
    }

    println!("total value: {}", total_value);
}

fn main() {
    task2();
}