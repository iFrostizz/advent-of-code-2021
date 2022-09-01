use std::char;

fn main() {
    // part_one();
    part_two();
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

fn part_two() {
    let data = include_str!("../data.txt");
    let data = data
        .lines()
        .map(|l| l.parse::<String>().unwrap())
        .collect::<Vec<String>>();

    let data = data
        .iter()
        .map(|el| el.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let oxygen = get_material(data.clone(), true);
    let oxygen = get_dec_from_bin(oxygen);
    let co2 = get_material(data, false);
    let co2 = get_dec_from_bin(co2);
    dbg!(&oxygen, &co2);

    let life = oxygen * co2;

    println!("{}", life);
}

fn get_dec_from_bin(bin: Vec<char>) -> u32 {
    let bin = bin.iter().collect::<String>();
    u32::from_str_radix(&bin, 2).unwrap()
}

fn get_material(data: Vec<Vec<char>>, oxygen: bool) -> Vec<char> {
    let mut data = data;

    for col in 0..data[0].len() {
        let (zeros, ones) = count_bits(data.clone(), col);

        let char = if oxygen {
            if ones >= zeros {
                '1'
            } else {
                '0'
            }
        } else {
            if zeros <= ones {
                '0'
            } else {
                '1'
            }
        };

        data = get_by_bit_at_pos(data, char, col);

        if data.len() == 1 {
            break;
        }
    }

    data[0].clone()
}

fn get_oxygen(data: Vec<Vec<char>>) {}

fn get_by_bit_at_pos(data: Vec<Vec<char>>, bit: char, pos: usize) -> Vec<Vec<char>> {
    let mut got: Vec<Vec<char>> = Vec::new();

    for row in data.iter() {
        if row[pos] == bit {
            got.push(row.clone());
        }
    }

    got
}

/*
    101000
    100001
    011011
*/
fn count_bits(data: Vec<Vec<char>>, pos: usize) -> (u32, u32) {
    let mut zeros: u32 = 0;
    let mut ones: u32 = 0;

    for row in 0..data.len() {
        let col = &data[row];
        let bit = col[pos];
        if bit == '0' {
            zeros = zeros + 1;
        } else {
            ones = ones + 1;
        }
    }

    return (zeros, ones);
}
