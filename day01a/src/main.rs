use std::fs;

fn comp(prev: i32, curr: i32, ups: &mut u32) {
    if prev < curr {
        *ups += 1;
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to load input.txt");


    let mut ups: u32 = 0;
    let mut lines = contents.lines();
    let mut prev: i32 = lines.next().unwrap().parse::<i32>().unwrap();

    for line in lines {
        let curr = line.parse::<i32>().unwrap();

        comp(prev, curr, &mut ups);
        prev = curr;
    }

    println!("{ups}");
}
