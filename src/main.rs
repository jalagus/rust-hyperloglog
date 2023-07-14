use rand::Rng;


fn naive_cardinality(items: &Vec<String>) -> usize {
    let mut unique_items = std::collections::HashSet::new();

    for item in items.iter() {
        unique_items.insert(item);
    }

    unique_items.len()
}

fn hash(x: &String) -> u32 {
    let mut hash: u32 = 0x1436781;
    // One-byte-at-a-time hash based on Murmur's mix
    // Source: https://github.com/aappleby/smhasher/blob/master/src/Hashes.cpp
    for &c in x.as_bytes() {
        hash ^= c as u32;
        hash = (std::num::Wrapping(hash) *  std::num::Wrapping(0x5bd1e995)).0;
        hash ^= hash >> 15;
    }

    hash
}

fn p(s: u32) -> u32 {
    s.leading_zeros() + 1
}

fn hyperloglog(items: &Vec<String>, b: usize) -> f64 {
    let m: usize = 2 << (b - 1);
    let am: f64 = 0.7213 / (1.0 + (1.079 / (m as f64)));
    let mut memory: Vec<u32> = vec![0; m];

    let mask = (2 << (31 - b)) - 1;

    for item in items.iter() {
        let h = hash(item);
        let j = (h >> (32 - b)) as usize;
        let w = h & mask;

        memory[j] = std::cmp::max(memory[j], p(w));
    }

    let z = memory.into_iter().map(|x| (2.0_f64).powf(-(x as f64)))
        .into_iter().reduce(|x, y| x + y)
        .expect("Calculation failed.");
    
    (am * ((m << 1) as f64)) / z
}

fn generate_test_data(n: u32) -> Vec<String> {
    let mut data: Vec<String> = Vec::new();
    let contents = std::fs::read_to_string("words.txt")
        .expect("Should have been able to read the file");
    let words:Vec<&str> = contents.split_whitespace().collect();

    let mut rng = rand::thread_rng();
    for _ in 0..n {
        data.push(words[rng.gen_range(0..words.len())].to_string());
    }

    data
}

fn main() {
    let test_data =  generate_test_data(100000);

    let cardinality = naive_cardinality(&test_data);    
    let approx_cardinality = hyperloglog(&test_data, 4);    

    println!("Real cardinality: {cardinality} Approx. cardinality: {approx_cardinality}");
}
