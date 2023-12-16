use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::ops::RangeInclusive;

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

// ----- Day 3 -----
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartNumber {
    pub y: usize,
    pub x: usize,
    pub num: u32,
}

impl std::iter::Sum<PartNumber> for u32 {
    fn sum<I: Iterator<Item = PartNumber>>(iter: I) -> Self {
        iter.fold(0, |acc, x| acc + x.num)
    }
}
// impl std::iter::Sum for PartNumber {
//     k
// }

pub fn create_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let max = matrix.iter().fold(0, |acc, x| {
        acc.max(x.len())
    });

    for line in &mut matrix {
        line.resize(max, '.');
    }

    matrix
}

pub fn print_matrix<T: Display>(matrix: &[Vec<T>]) {
    for line in matrix {
        for item in line {
            print!("{item}");
        }
        println!();
    }
}

pub fn get_neighbors_index<T>(matrix: &[Vec<T>], index: (usize, usize)) -> Vec<(usize, usize, &T)> {
    const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [ // (y, x)
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    let mut neighbors: Vec<(usize, usize, &T)> = Vec::new();
    for offset in NEIGHBOR_OFFSETS {
        let y = index.0 as i32 + offset.0;
        let x = index.1 as i32 + offset.1;
        if y < 0 || x < 0 {
            continue;
        }
        let (y, x): (usize, usize) =
            (y.try_into().unwrap(), x.try_into().unwrap());
        
        match &matrix.get(y) {
            None => continue,
            Some(v) => match v.get(x) {
                None => continue,
                Some(n) => neighbors.push((y, x, n)),
            }
        }
    }

    neighbors
}

// TODO: Do more tests for this
pub fn get_full_number(matrix: &[Vec<char>], index: &(usize, usize)) -> Option<PartNumber> {
    if !matrix
        .get(index.0)?
        .get(index.1)?
        .is_numeric()
    {
        None?
    }

    let mut start_index = index.1;
    let mut end_index = index.1;
    println!("start_index: {}", start_index);
    println!("end_index: {}", end_index);
    if let Some(row) = matrix.get(index.0) {
        println!("valid row index: {}", index.0);

        if start_index > 0 {
            while let Some(c) = row.get(start_index-1) {
                println!("start_index: {}", start_index);
                if c.is_numeric() {
                    start_index -= 1;
                } else { break; }
                if start_index == 0 { break; }
            }
        }

        if end_index < row.len()-1 {
            while let Some(c) = row.get(end_index+1) {
                if c.is_numeric() {
                    end_index += 1;
                } else { break; }
            }
        }

        let num_string: String = row.get(start_index..end_index+1).unwrap()
            .iter()
            .collect::<String>();

        Some(PartNumber {
            y: index.0,
            x: start_index,
            num: num_string.parse().unwrap(),
        })

    } else {
        None
    }
}

pub fn get_part_numbers(matrix: &[Vec<char>]) -> Vec<PartNumber> {
    let mut parts: Vec<PartNumber> = vec![];
    // Loop through rows in matrix, and items in rows
    for (y, row) in matrix.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            // Check if the item is not a number and not '.'
            if !item.is_numeric() && item != &'.' {
                // Loop through its neighbors
                for (n_y, n_x, neighbor) in get_neighbors_index(matrix, (y, x)) {
                    if neighbor.is_numeric() {
                        if let Some(part) = get_full_number(matrix, &(n_y, n_x)) {
                            parts.push(part);
                        }
                    }
                }
            }
        }
    }

    parts
}

pub fn remove_duplicates<T: Hash + Eq>(list: Vec<T>) -> Vec<T> {
    let set: HashSet<_> = list.into_iter().collect();
    set.into_iter().collect()
}

// ----- Day 4 -----

#[derive(Debug, Clone)]
pub struct Card {
    pub index: usize,
    pub winning_nums: Vec<u32>,
    pub my_nums: Vec<u32>,
}
impl Card {
    pub fn calculate_matches(&self) -> u32 {
        self.my_nums.iter().filter(|num| self.winning_nums.contains(num)).count() as u32
    }
}
impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = match value.strip_prefix("Card") {
            None => Err(String::from("Card must start with `Card`"))?,
            Some(v) => v.trim(),
        };

        let (idx_str, nums_str) = match value.split_once(':') {
            None => Err(String::from("Card must contain a `:`"))?,
            Some(strs) => strs,
        };

        let index = idx_str.parse::<usize>().map_err(|_| String::from("Card must have a valid index"))?;

        let (winning_nums_str, my_nums_str) = match nums_str.split_once('|') {
            None => Err(String::from("Card numbers must be separated with a `|`"))?,
            Some((w, m)) => (w.trim(), m.trim()),
        };

        let winning_nums = winning_nums_str.split(' ').filter(|x| x != &"").map(|num| {
            num.parse::<u32>()
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| String::from("All numbers must be valid"))?;

        let my_nums = my_nums_str.split(' ').filter(|x| x != &"").map(|num| {
            num.parse::<u32>()
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| String::from("All numbers must be valid"))?;

        Ok(Card { index, winning_nums, my_nums })
    }
}

pub fn calc_total_cards(cards: &[Card]) -> u32 {
    // Create vector the same length as `cards`, initialize all counts to 1
    let mut counts: Vec<u32> = (0..cards.len()).map(|_| 1).collect();

    for (i, card) in cards.iter().enumerate() {
        let matches = card.calculate_matches();
        let count = counts[i];

        // Loop through self+1 to self+matches
        for offset in 1..(matches+1) as usize {
            // Check if exists
            match counts.get_mut(i+offset) {
                None => break,
                Some(c) => *c += count, // If it does, add the current card's count to the other card's count
            }
        }
    }

    counts.iter().sum()
}

// ----- Day 5 -----

#[derive(Debug)]
pub struct MapRange {
    pub range: RangeInclusive<u32>,
    pub offset: i32,
}

pub type Mapping = Vec<MapRange>;

pub fn parse_almanac(input: &str) 
-> (Vec<u32>, Vec<Vec<[u32; 3]>>) {

    let mut lines = input.lines().filter(|l| l != &"");
    let seeds: Vec<_> = lines
        .next().unwrap()
        .split_once(": ").unwrap()
        .1.split(' ').map(|n| n.parse::<u32>().unwrap())
        .collect();

    lines.next(); // Skip first example line

    let mut maps: Vec<Vec<[u32; 3]>> = vec![vec![]];
    let mut i = 0;
    for line in lines {
        if !line.chars().next().unwrap().is_numeric() {
            maps.push(Vec::new());
            i += 1;
            continue;
        }

        maps[i].push(
            line.split(' ')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
                [0..3].try_into().unwrap()
        )
    }

    (seeds, maps)
}

pub fn parse_mappings(mapping_collections: Vec<Vec<[u32; 3]>>) -> Vec<Mapping> {

    mapping_collections.iter().map(|collection| {
        collection.iter().map(|mapping| {
            // 0: destination range start
            // 1: source range start
            // 2: range length
            let start = mapping[1];
            let end = start + mapping[2];
            let offset = mapping[0] as i32 - start as i32;

            MapRange { range: start..=end, offset }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}
/// 
pub fn follow_mappings(initial: u32, mappings: &Vec<Mapping>) -> u32 {
    // println!("\n\nFollowing mappings...");
    let mut value = initial as i32;

    for mapping in mappings {
        // print!("{value} -> ");
        // println!("\nMapping: {mapping:?}");
        // println!("Start value: {value}");
        for map_range in mapping {
            // println!("Range, offset: {:?}, {}", map_range.range, map_range.offset);
            if map_range.range.contains(&(value as u32)) {
                // println!("Value is in range, applying offset");
                value += map_range.offset;
                break;
            }
        }
        // println!("End value: {value}");
        // print!("{value}; ");
    }

    value as u32
}
