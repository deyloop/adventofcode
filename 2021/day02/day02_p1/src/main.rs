use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut position = (0, 0);

    while let Some(Ok(line)) = lines.next() {
        let mut line = line.trim().split(' ').into_iter();
        let direction = line.next().unwrap();
        let steps = line.next().unwrap().trim().parse::<i32>().unwrap();

        match direction {
            "forward" => position.0 += steps,
            "down" => position.1 += steps,
            "up" => position.1 -= steps,
            _ => unreachable!(),
        };
    }

    println!("{}", position.0 * position.1);

    Ok(())
}
