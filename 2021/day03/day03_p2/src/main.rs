use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut counts = HashMap::new();

    let mut width = 0;

    let mut numbers: Vec<usize> = lines
        .map(|line| {
            let line = line.unwrap();
            width = line.len();
            line.trim()
                .chars()
                .enumerate()
                .fold(0, |number, (ind, digit_char)| {
                    let bit = to_bit(&digit_char);

                    number + (0b1 << width - 1 - ind) * bit
                })
        })
        .collect();

    let saved_numbers = numbers.clone();

    tally_bits(&numbers, width, &mut counts);

    println!("{:#?} {:#?}", numbers, counts);

    for index in (0..width).rev() {
        if numbers.len() > 1 {
            let mask = 0b1 << index;
            let num_ones = counts.get(&index).unwrap().clone();
            let num_zeros = numbers.len() - num_ones;
            let bit = if num_ones >= num_zeros { 1 } else { 0 };

            println!("{} {} {}", num_ones, num_zeros, bit);

            numbers = numbers
                .into_iter()
                .filter(|&number| {
                    println!("{} {}", number & mask, bit << index);
                    (number & mask) == (bit << index)
                })
                .collect();

            tally_bits(&numbers, width, &mut counts);

            println!("{:#?}", numbers);
        } else {
            break;
        }
    }

    let oxygen_gen_rating = numbers[0];

    let mut numbers = saved_numbers;

    tally_bits(&numbers, width, &mut counts);

    for index in (0..width).rev() {
        if numbers.len() > 1 {
            let mask = 0b1 << index;
            let num_ones = counts.get(&index).unwrap().clone();
            let num_zeros = numbers.len() - num_ones;
            let bit = if num_ones < num_zeros { 1 } else { 0 };

            println!("{} {} {}", num_ones, num_zeros, bit);

            numbers = numbers
                .into_iter()
                .filter(|&number| {
                    println!("{} {}", number & mask, bit << index);
                    (number & mask) == (bit << index)
                })
                .collect();

            tally_bits(&numbers, width, &mut counts);

            println!("{:#?}", numbers);
        } else {
            break;
        }
    }

    let co2_scubber_rating = numbers[0];

    println!(
        "{} {} {}",
        oxygen_gen_rating,
        co2_scubber_rating,
        oxygen_gen_rating * co2_scubber_rating
    );

    Ok(())
}

fn tally_bits(numbers: &[usize], width: usize, counts: &mut HashMap<usize, usize>) {
    counts.clear();
    numbers.iter().for_each(|number| {
        for i in 0..width {
            let mask = 0b1 << i;
            *counts.entry(i).or_insert(0) += (*number & mask) >> i;
        }
    });
}

fn to_bit(character: &char) -> usize {
    match character {
        '1' => 1,
        '0' => 0,
        _ => unreachable!(),
    }
}
