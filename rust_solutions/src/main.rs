use rust_solutions::{parse_games, filter_limits, find_min};
fn main() {
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

    // let limits = (12, 13, 14);
    // let game_ids = filter_limits(games, limits);
    // println!("Filtered: {game_ids:?}\nLimits: {limits:?}");
    // println!("Sum: {}", game_ids.iter().sum::<usize>());
}
