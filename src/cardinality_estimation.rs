pub fn naive_cardinality(items: &Vec<String>) -> usize {
    let mut unique_items = std::collections::HashSet::new();
    unique_items.extend(items);
    unique_items.len()
}

fn rho(s: u32) -> u32 {
    s.leading_zeros() + 1
}

fn fill_buckets(items: &Vec<String>, b: u32, hash_fn: fn(&String) -> u32) -> Vec<u32> {
    let m: usize = 1 << b;
    let mut bucket: Vec<u32> = vec![0; m];
        
    for item in items.iter() {
        let h = hash_fn(item);
        let j = (h >> (32 - b)) as usize;
        let w = h << b;

        bucket[j] = std::cmp::max(bucket[j], rho(w));
    }

    bucket
}

pub fn loglog(items: &Vec<String>, b: u32, hash_fn: fn(&String) -> u32) -> f64 {
    let m: usize = 1 << b;
    let bucket: Vec<u32> = fill_buckets(items, b, hash_fn);
    let alpha = 0.697;

    let k = bucket.into_iter().reduce(|a, b| a + b).expect("Could not sum.") as f64;
    let multiplier = m as f64;

    multiplier * alpha * (2.0_f64).powf(k / multiplier)
}    

pub fn hyperloglog(items: &Vec<String>, b: u32, hash_fn: fn(&String) -> u32) -> f64 {
    if (b < 4) || (b > 16) {
        panic!("Bit count must be within [4, 16]. Was '{b}'");
    }

    let m: usize = 1 << b;
    let bucket: Vec<u32> = fill_buckets(items, b, hash_fn);
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