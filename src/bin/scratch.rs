fn get_divisors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    for i in 2..=n/2 { 
        if n % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

fn split_equally(chunk_size: u64, s: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    if chunk_size == 0 { return chunks; } 
    for chunk in s.as_bytes().chunks(chunk_size as usize) {
        let str_chunk = std::str::from_utf8(chunk).unwrap();
        chunks.push(str_chunk.to_string()); 
    }
    chunks
}

fn equal_check_shortcirc(a: &Vec<String>) -> bool {
    if a.is_empty() { return false; } // Empty split isn't "valid" usually
    let first = &a[0];
    a.iter().all(|item| item == first)
}

fn main() {
    let mut v = (100000..100005).collect::<Vec<u64>>();
    v.push(121212); 

    let res: Vec<_> = v.iter()
        .map(|x| x.to_string()) // Convert to String immediately
        .filter(|s| {
            let len = s.len() as u64;
            let divisors = get_divisors(len);
            divisors.iter().any(|&d| {
                let chunks = split_equally(d, s);
                equal_check_shortcirc(&chunks)
            })
        })
        .collect();

    println!("Numbers with at least one valid equal split pattern:");
    for s in res {
        println!("{}", s);
    }
}