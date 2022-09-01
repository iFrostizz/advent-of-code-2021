use std::char;

fn main() {
    part_one();
}

// Loop over each character vertically
// epsilon |= gamma (not or but Xor finally)
// power = epsion * gamma
fn part_one() {
    let data = include_str!("../data.txt");
    let data = data
        .lines()
        .map(|l| l.parse::<String>().unwrap())
        .collect::<Vec<String>>();

    let data = data
        .iter()
        .map(|el| el.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let data = transpose(data);

    let mut gamma: Vec<char> = Vec::new();

    for row in 0..data.len() {
        let cols = &data[row];

        let mut zeros: u32 = 0;
        let mut ones: u32 = 0;

        for val in cols {
            match val {
                '0' => zeros = zeros + 1,
                '1' => ones = ones + 1,
                _ => panic!("oops"),
            };
        }

        if ones > zeros {
            gamma.push('1');
        } else {
            gamma.push('0');
        }
    }

    let gamma = gamma.iter().collect::<String>();
    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let base: u32 = 2;
    let epsilon = base.pow(12) - 1 ^ gamma;

    println!("{:08b} {:08b}", gamma, epsilon);

    println!("{}", gamma * epsilon);
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
