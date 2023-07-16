use rand::{Rng, SeedableRng};

mod cardinality_estimation;
mod hashing;

fn generate_test_data(n: u32, max_cardinality: usize, seed: u64) -> Vec<String> {
    let mut data: Vec<String> = Vec::new();
    let contents = std::fs::read_to_string("words.txt")
        .expect("File reading failed.");
    let words:Vec<&str> = contents.split_whitespace().collect();

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

    let max_len = std::cmp::min(max_cardinality, words.len());
    for _ in 0..n {
        data.push(words[rng.gen_range(0..max_len)].to_string());
    }
    data
}

fn main() {
    for iter in 1..5 {
        let max_cardinality = 20 * 10_usize.pow(iter);
        let test_n = (max_cardinality * 10) as u32;
        
        let test_data =  generate_test_data(test_n, max_cardinality, 42);
        let cardinality = cardinality_estimation::naive_cardinality(&test_data);    

        println!("-- Dataset: {} items, cardinality {} --", test_data.len(), cardinality);
        for b in [4, 5, 6, 7, 8] {
            let ll_approx_cardinality = cardinality_estimation::loglog(&test_data, b, hashing::murmur_oaat);    
            let hll_approx_cardinality = cardinality_estimation::hyperloglog(&test_data, b, hashing::murmur_oaat);    

            println!("b = {b} :: LL esimate: {ll_approx_cardinality:.2} HLL esimate: {hll_approx_cardinality:.2}");
        }
    }
}
