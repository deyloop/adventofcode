use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut counts: HashMap<usize, i32> = HashMap::new();

    let mut linecount = 0;

    while let Some(line) = lines.next() {
        linecount += 1;

        let number = line?.trim().to_owned();
        number.chars().enumerate().for_each(|p| {
            *counts.entry(p.0).or_insert(0) += match p.1 {
                '1' => 1,
                '0' => 0,
                _ => unreachable!(),
            }
        });
    }

    println!("linecount = {}", linecount);
    println!("counts size = {}", counts.len());
    let width = counts.len();

    let mut gamma = 0u64;
    let mut epsilon = 0u64;
    for (ind, num_ones) in counts.into_iter() {
        let num_zeros = linecount - num_ones;
        let (gamma_bit, epsilon_bit) = if num_ones > num_zeros { (1, 0) } else { (0, 1) };
        let bit_value = 0b1 << (width - 1 - ind);
        println!(
            "{} {} {} {} {}",
            ind, num_ones, num_zeros, gamma_bit, bit_value
        );

        gamma += bit_value * gamma_bit;
        epsilon += bit_value * epsilon_bit;
    }

    println!();
    println!("{} {} {}", gamma, epsilon, gamma * epsilon);

    Ok(())
}
