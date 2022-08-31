fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    let data = include_str!("../data.txt");

    let data: Vec<String> = data.lines().map(|l| l.parse::<String>().unwrap()).collect();

    let mut hor: i32 = 0;
    let mut dep: i32 = 0;

    data.iter().for_each(|el| {
        let split: Vec<&str> = el.split(' ').collect();
        let dir: &str = split.get(0).unwrap();
        let num: i32 = split.get(1).unwrap().parse().unwrap();

        match dir {
            "forward" => hor = hor + num,
            "down" => dep = dep + num,
            "up" => dep = dep - num,
            _ => panic!("nooo"),
        }
    });

    let result = hor * dep;
    println!("{}", result);
}

fn part_two() {
    let data = include_str!("../data.txt");

    let data: Vec<String> = data.lines().map(|l| l.parse::<String>().unwrap()).collect();

    let mut hor: i32 = 0;
    let mut dep: i32 = 0;
    let mut aim: i32 = 0;

    data.iter().for_each(|el| {
        let split: Vec<&str> = el.split(' ').collect();
        let dir: &str = split.get(0).unwrap();
        let num: i32 = split.get(1).unwrap().parse().unwrap();

        match dir {
            "forward" => {
                hor = hor + num;
                dep = dep + aim * num;
            }
            "down" => aim = aim + num,
            "up" => aim = aim - num,
            _ => panic!("nooo"),
        }
    });

    let result = hor * dep;
    println!("{}", result);
}
