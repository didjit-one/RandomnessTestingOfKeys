use rand::Rng;
use std::collections::HashMap;

//initial sequence length 2500 bytes
const SEQUENCE_LENGTH: usize = 2500;
//number of Poker blocks
const NUMBER_BLOCKS: f64 = 2.0 * SEQUENCE_LENGTH as f64;
//minimum value in sequence
const MIN: u32 = 9654;
//maximum value in sequence
const MAX: u32 = 10346;

fn main() {
    let random_sequence: String = generate_random_sequence(SEQUENCE_LENGTH);

    // println!("Random_sequence {}", random_sequence);

    println!("Monobit test passed?: {}", monobit_test(&random_sequence));
    println!(
        "Maximum series length test is passed?: {}",
        maximum_series_length_test(&random_sequence)
    );
    println!("Poker test is passed?: {}", poker_test(&random_sequence));

    println!("Serial test is passed?: {}", serial_test(&random_sequence));
}

/// Implementation of our random sequence
fn generate_random_sequence(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let sequence: String = (0..length)
        .map(|_| format!("{:08b}", rng.gen::<u8>()))
        .collect();
    sequence
}

/// Implementation of a monobit test
fn monobit_test(random_sequence: &str) -> bool {
    let check_one = random_sequence.chars().filter(|&c| c == '1').count() as u32;

    println!("Monobit test result: {}", check_one);

    check_one > MIN && check_one < MAX
}

/// Implementation of the maximum series length test
fn maximum_series_length_test(random_sequence: &str) -> bool {
    let mut check_one = 0;
    let mut check_zero = 0;
    let mut max_one = 0;
    let mut max_zero = 0;

    let sequence_chars: Vec<char> = random_sequence.chars().collect();
    let sequence_len = sequence_chars.len();

    for i in 0..sequence_len - 1 {
        if sequence_chars[i] == '1' && sequence_chars[i + 1] == '1' {
            check_one += 1;
        } else {
            check_one += 1;
            if max_one < check_one {
                max_one = check_one;
            }
            check_one = 0;
        }

        if sequence_chars[i] == '0' && sequence_chars[i + 1] == '0' {
            check_zero += 1;
        } else {
            check_zero += 1;
            if max_zero < check_zero {
                max_zero = check_zero;
            }
            check_zero = 0;
        }
    }
    println!(
        "Maximum series length test result: {} and {}",
        max_one, max_zero
    );

    return max_one < 36 && max_zero < 36;
}

///Implementation of the Poker test
fn poker_test(random_sequence: &str) -> bool {
    let mut blocks: Vec<&str> = Vec::new();

    // Splitting a sequence into blocks of 4 characters
    for i in (0..random_sequence.len()).step_by(4) {
        if i + 4 <= random_sequence.len() {
            let block = &random_sequence[i..i + 4];
            blocks.push(block);
        }
    }

    let mut block_counts: HashMap<&str, usize> = HashMap::new();

    // Counting different blocks encountered
    for block in &blocks {
        let count = block_counts.entry(block).or_insert(0);
        *count += 1;
    }

    let mut sum_count = 0.0;

    for (block, count) in block_counts {
        sum_count += (count as f64).powi(2);
        //      println!("Block: {}, Quantity: {}", block, count);
    }

    //Pokers formula to calculate
    let x3: f64 = ((2 as f64).powi(4) * (sum_count as f64) / NUMBER_BLOCKS) - NUMBER_BLOCKS;

    println!("Poker test result: {}", x3);

    //range (1.03, 57.4) by task
    return x3 > 1.03 && x3 < 57.4;
}

/// Implementation of the series length test
fn serial_test(random_sequence: &str) -> bool {
    // Convert string to bit sequence
    let sequence: Vec<u8> = random_sequence
        .chars()
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            _ => panic!("Invalid character in sequence"),
        })
        .collect();

    let mut one_series_lengths = vec![0; 6];
    let mut zero_series_lengths = vec![0; 6];

    let mut current_ones_length = 0;
    let mut current_zeroes_length = 0;

    for bit in sequence {
        if bit == 1 {
            current_ones_length += 1;
            if current_zeroes_length > 0 {
                if current_zeroes_length >= 6 {
                    zero_series_lengths[5] += 1;
                } else {
                    zero_series_lengths[current_zeroes_length - 1] += 1;
                }
                current_zeroes_length = 0;
            }
        } else {
            current_zeroes_length += 1;
            if current_ones_length > 0 {
                if current_ones_length >= 6 {
                    one_series_lengths[5] += 1;
                } else {
                    one_series_lengths[current_ones_length - 1] += 1;
                }
                current_ones_length = 0;
            }
        }
    }

    println!("Serial test result for one: {:?}", one_series_lengths);
    println!("Serial test result for zero {:?}", zero_series_lengths);

    let one_intervals = vec![
        (2267, 2733),
        (1079, 1421),
        (502, 748),
        (223, 402),
        (90, 223),
        (90, 223),
    ];
    let zero_intervals = vec![
        (2267, 2733),
        (1079, 1421),
        (502, 748),
        (223, 402),
        (90, 223),
        (90, 223),
    ];

    one_series_lengths
        .iter()
        .zip(one_intervals.iter())
        .all(|(&length, &(lower, upper))| length >= lower && length <= upper)
        && zero_series_lengths
            .iter()
            .zip(zero_intervals.iter())
            .all(|(&length, &(lower, upper))| length >= lower && length <= upper)
}
