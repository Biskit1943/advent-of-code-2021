use std::fs;

fn comp(a1: i32, a2b1: i32, a3b2: i32, b3: i32, ups: &mut u32) {
    let a = a1 + a2b1 + a3b2;
    let b = a2b1 + a3b2 + b3;

    if b > a {
        *ups += 1;
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to load input.txt");
    let mut lines = contents.lines();

    let mut a1: i32 = lines.next().unwrap().parse::<i32>().unwrap();
    let mut a2b1: i32 = lines.next().unwrap().parse::<i32>().unwrap();
    let mut a3b2: i32 = lines.next().unwrap().parse::<i32>().unwrap();

    let mut ups: u32 = 0;

    for line in lines {
        let b3: i32 = line.parse::<i32>().unwrap();
        comp(a1, a2b1, a3b2, b3, &mut ups);

        a1 = a2b1;
        a2b1 = a3b2;
        a3b2 = b3;
    }

    println!("{ups}");
}
