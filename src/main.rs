use rand::Rng;

//Функція приймає довжину послідовності, яку потрібно згенерувати,
// і повертає випадкову послідовність байтів
fn generate_random_sequence(length: usize) -> Vec<u8> {
    let mut sequence = vec![0u8; length];
    let mut rng = rand::thread_rng();

    for i in 0..length {
        let random_bit: u8 = rng.gen_range(0..=1);
        sequence[i]= random_bit;
    }
    sequence
}

fn main() {
    // Приклад використання генератора випадкових послідовностей
    let random_sequence = generate_random_sequence(20000);
    println!("{:?}", random_sequence);
}
