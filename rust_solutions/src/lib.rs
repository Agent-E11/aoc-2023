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
