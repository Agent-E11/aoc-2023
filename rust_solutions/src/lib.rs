use std::collections::HashMap;

// ----- Day 1 -----

pub fn day1_pt1(input: &str) -> u32 {

    let mut nums = Vec::new();
    for line in input.lines() {
        let mut first_char = '0';
        let mut last_char = '0';
        for c in line.chars() {
            if c.is_numeric() {
                first_char = c;
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_numeric() {
                last_char = c;
                break;
            }
        }

        let num: u32 = format!("{}{}", first_char, last_char).parse().unwrap();
        println!("{num} : {line}");
        nums.push(num);
    }

    nums.iter().sum()
}

// ----- Day 2 -----

type Game = (usize, Vec<(usize, usize, usize)>);

pub fn parse_games(input: &str) -> Result<Vec<Game>, String> {

    let color_index_map: HashMap<String, usize> = HashMap::from([
        ("red".to_owned(), 0),
        ("green".to_owned(), 1),
        ("blue".to_owned(), 2),
    ]);

    let mut parsed_games: Vec<Game> = Vec::new();

    for line in input.lines() {
        let (idx_str, cube_sets_str) = line.split_once(':').unwrap();
        let game_idx: usize = idx_str.split_once(' ').unwrap().1.parse().unwrap();
        let cube_sets: Vec<(usize, usize, usize)> = cube_sets_str.split(';').map(|set| {
            println!("Set: {set}");
            let collections = set.splitn(3, ',');

            let mut set_array = [0, 0, 0];

            for collection in collections {
                let (num, color) = collection.trim().split_once(' ').unwrap();
                println!("Number: {num}, Color: {color}");
                let idx = color_index_map[&color.to_lowercase()];
                set_array[idx] = num.parse().unwrap();
            }
            set_array.into()
        }).collect();

        parsed_games.push((game_idx, cube_sets));
    }

    Ok(parsed_games)
}

pub fn filter_limits(games: Vec<Game>, limits: (usize, usize, usize)) -> Vec<usize> {
    let (red_max, green_max, blue_max) = limits;
    let mut game_ids = Vec::new();
    for game in games {
        let mut include_game = true;
        for (r, g, b) in game.1 {
            if r>red_max || g>green_max || b>blue_max {
                include_game = false;
            }
        }
        if include_game { game_ids.push(game.0) }
    }
    game_ids
}

pub fn find_min(games: Vec<Game>) -> Vec<(usize, usize, usize)> {
    games.iter().map(|game| {
        let mut maxes = (0, 0, 0);
        for collection in &game.1 {
            if collection.0 > maxes.0 { maxes.0 = collection.0 }
            if collection.1 > maxes.1 { maxes.1 = collection.1 }
            if collection.2 > maxes.2 { maxes.2 = collection.2 }
        }
        maxes
    }).collect()
}
