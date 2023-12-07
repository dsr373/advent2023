struct Symbol {
    line: usize,
    column: usize
}

type Table = Vec<Vec<char>>;

fn read_table(text: &str) -> Table {
    let table: Vec<_> = text
        .lines()
        .map(|line|
            line
            .chars()
            .collect::<Vec<_>>())
        .collect();
    return table;
}

fn find_symbols(table: &Table) -> Vec<Symbol> {
    let mut symbols = Vec::new();
    for (line, chars) in table.iter().enumerate() {
        for (column, char) in chars.iter().enumerate() {
            if !char.is_digit(10) && *char != '.' {
                symbols.push(Symbol{line, column});
            }
        }
    }
    return symbols;
}

/// change in (line, column) to move in every direction
const DISPLACEMENT: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn visit_all_neighbours(table: &Table, visited: &mut Vec<Vec<bool>>, symbol: &Symbol) -> Vec<u32> {
    // TODO
    // iterate through neighbours of symbols
    // find numbers and debuffer entire thing
    // mark as visited in table

    return vec![];
}

fn task1() {
    let input = "day3task1.in";
    let input_content = std::fs::read_to_string(input).expect("failed to read input");
    let table = read_table(&input_content);
    let symbols = find_symbols(&table);
    let mut visitation_table: Vec<Vec<bool>> = table.iter()
        .map(|line| vec![false; line.len()]).collect();

    let mut sum_of_parts = 0u32;
    for symbol in symbols {
        sum_of_parts += visit_all_neighbours(&table, &mut visitation_table, &symbol)
            .iter().sum::<u32>();
    }
    println!("Sum of all parts: {}", sum_of_parts);
}

fn main() {
    task1();
}