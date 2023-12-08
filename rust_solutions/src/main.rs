use rust_solutions::{
    parse_games,
    find_min,
    create_matrix,
    get_neighbors,
    print_matrix,
};
fn main() {
    run_day_3();
}

fn run_day_3() {
    let input = include_str!("../../inputs/day-3.txt");
    println!("{input}");

    let matrix = create_matrix(input);
    // print_matrix(&matrix);

    println!("(1, 1): {:?}", get_neighbors(&matrix, (1, 1)));
    println!("(0, 5): {:?}", get_neighbors(&matrix, (0, 5)));
    println!("(2, 1): {:?}", get_neighbors(&matrix, (2, 1)));
    println!("(1, 8): {:?}", get_neighbors(&matrix, (1, 8)));
    println!("(1, 10): {:?}", get_neighbors(&matrix, (1, 10)));
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
