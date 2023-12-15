use rust_solutions::{
    parse_games,
    find_min,
    create_matrix,
    get_part_numbers,
    calc_total_cards,
    Card,
    parse_almanac,
    parse_mappings,
};
fn main() {
    run_day_5_pt_1();
}

fn run_day_5_pt_1() {
    let input = include_str!("../../inputs/tests/day-5.txt");

    let (seeds, mappings) = parse_almanac(input);
    println!("Seeds: {seeds:?}");
    println!("Mappings:\n{mappings:?}");
    
    for i in parse_mappings(mappings) {
        println!("{:?}", i);
    }
}

fn _run_day_4_pt_2() {
    // Load input
    let input = include_str!("../../inputs/day-4.txt");

    // Parse input into cards
    let cards: Vec<_> = input.lines().map(|line| {
        Card::try_from(line).unwrap()
    }).collect();

    // Calculate answer
    let count = calc_total_cards(&cards);

    println!("Number of cards: {count}");
}

fn _run_day_4_pt_1() {
    let input = include_str!("../../inputs/day-4.txt");
    println!("Input:\n{}", input);

    // Generate card structs from input
    let cards: Vec<_> = input.lines().map(|line| {
        let card = Card::try_from(line).unwrap();
        println!("{:?}", card);
        card
    }).collect();

    let sum: u32 = cards.iter().map(|c| 2u32.pow(c.calculate_matches()) / 2).sum();

    println!("Point total: {}", sum);
}

fn _run_day_3_pt_1() {
    let input = include_str!("../../inputs/day-3.txt");
    println!("{input}");

    let matrix = create_matrix(input);

    let mut part_nums = get_part_numbers(&matrix);

    part_nums.sort();
    part_nums.dedup();

    let sum: u32 = part_nums.into_iter().sum();
    println!("Sum: {sum}");
}

fn _run_day_2() {
    let day_2_in = include_str!("../../inputs/day-2.txt");
    let games = parse_games(day_2_in).unwrap();
    println!("Games: {:?}", games);

    let minimums = find_min(games);
    println!("Minimums: {:?}", minimums);

    let powers = minimums.iter().map(|mins| {
        mins.0 * mins.1 * mins.2
    });

    let res: usize = powers.sum();

    println!("Result: {}", res);
}
