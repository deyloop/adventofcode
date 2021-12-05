use std::io::{self, BufRead};

struct Submarine {
    pub position: (i32, i32),
    pub aim: i32,
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut submarine = Submarine {
        position: (0, 0),
        aim: 0,
    };

    while let Some(Ok(line)) = lines.next() {
        let mut line = line.trim().split(' ').into_iter();
        let direction = line.next().unwrap();
        let steps = line.next().unwrap().trim().parse::<i32>().unwrap();

        match direction {
            "forward" => {
                submarine.position.0 += steps;
                submarine.position.1 += submarine.aim * steps;
            }
            "down" => submarine.aim += steps,
            "up" => submarine.aim -= steps,
            _ => unreachable!(),
        };
    }

    println!("{}", submarine.position.0 * submarine.position.1);

    Ok(())
}
