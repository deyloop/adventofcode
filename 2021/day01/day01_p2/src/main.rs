use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut curr_sum = 0;
    let mut prev_sum = 0;

    let mut A: (i32, i32, i32) = (0, 0, 0);

    let mut increased = 0;
    let mut ready = 0;

    while let Some(line) = lines.next() {
        if ready < 4 {
            ready += 1;
        }

        A.0 = A.1;
        A.1 = A.2;

        A.2 = line.unwrap().trim().parse().unwrap();

        println!("{:?}", A);
        prev_sum = curr_sum;
        curr_sum = A.0 + A.1 + A.2;

        if ready == 4 && curr_sum > prev_sum {
            increased = increased + 1;
        }
    }

    println!("{}", increased);

    Ok(())
}
