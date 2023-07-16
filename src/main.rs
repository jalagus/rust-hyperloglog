use rand::{Rng, SeedableRng};

mod cardinality_estimation {
    pub fn naive_cardinality(items: &Vec<String>) -> usize {
        let mut unique_items = std::collections::HashSet::new();
        unique_items.extend(items);
        unique_items.len()
    }

    fn rho(s: u32) -> u32 {
        s.leading_zeros() + 1
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

    fn fill_buckets(items: &Vec<String>, b: u32) -> Vec<u32> {
        let m: usize = 1 << b;
        let mut bucket: Vec<u32> = vec![0; m];
            
        for item in items.iter() {
            let h = hash(item);
            let j = (h >> (32 - b)) as usize;
            let w = h << b;

            bucket[j] = std::cmp::max(bucket[j], rho(w));
        }

        bucket
    }

    pub fn loglog(items: &Vec<String>, b: u32) -> f64 {
        let m: usize = 1 << b;
        let bucket: Vec<u32> = fill_buckets(items, b);
        let alpha = 0.697;

        let k = bucket.into_iter().reduce(|a, b| a + b).expect("Could not sum.") as f64;
        let multiplier = m as f64;
    
        multiplier * alpha * (2.0_f64).powf(k / multiplier)
    }    

    pub fn hyperloglog(items: &Vec<String>, b: u32) -> f64 {
        if (b < 4) || (b > 16) {
            panic!("Bit count must be within [4, 16]. Was '{b}'");
        }

        let m: usize = 1 << b;
        let bucket: Vec<u32> = fill_buckets(items, b);
        let alpha = match m {
            16 => 0.678,
            32 => 0.697,
            64 => 0.709,
            other => 0.7213 / (1.0 + (1.079 / (other as f64)))
        };

        let z = bucket.iter()
            .map(|x| (2.0_f64).powf(-(*x as f64))).into_iter()
            .reduce(|x, y| x + y)
            .expect("Calculation failed.");
        
        let multiplier = m as f64;
        let e = (alpha * multiplier * multiplier) / z;
        const COEF: f64 = (1_u64 << 32) as f64;

        if e < ((5.0/2.0) * multiplier) {
            let non_empty_buckets = bucket.iter().fold(0, 
                |a, b| if *b > 0 {a + 1} else {a}
            );
            if non_empty_buckets > 0 {
                return multiplier * (multiplier / non_empty_buckets as f64).ln();
            }
            return e;
        } else if e <= COEF / 30.0 {
            return e;
        } else {
            return -COEF * (1.0 - e / COEF);
        }
    }  
  
}


fn generate_test_data(n: u32, seed: u64) -> Vec<String> {
    let mut data: Vec<String> = Vec::new();
    let contents = std::fs::read_to_string("words.txt")
        .expect("File reading failed.");
    let words:Vec<&str> = contents.split_whitespace().collect();

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

    for _ in 0..n {
        data.push(words[rng.gen_range(0..words.len())].to_string());
    }

    data
}

fn main() {
    for iter in 1..6 {
        let test_data =  generate_test_data(10 * 10_u32.pow(iter), 42);
        for b in [4, 5, 6, 7, 8] {
            let cardinality = cardinality_estimation::naive_cardinality(&test_data);    
            let ll_approx_cardinality = cardinality_estimation::loglog(&test_data, b);    
            let hll_approx_cardinality = cardinality_estimation::hyperloglog(&test_data, b);    

            println!("b = {b} :: True: {cardinality} LL: {ll_approx_cardinality:.2} HLL: {hll_approx_cardinality:.2}");
        }
        println!("-------");
    }
}
