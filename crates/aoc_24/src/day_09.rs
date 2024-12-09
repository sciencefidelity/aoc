use std::fs;

fn disk_fragmenter(path: &str) -> i32 {
    let message = fs::read_to_string(path).unwrap();
    let mut compressed = Vec::new();
    let (mut j, mut k) = (0, 0);
    for (i, b) in message.trim().bytes().enumerate() {
        let repeats: i32 = String::from_utf8(vec![b]).unwrap().parse().unwrap();
        if i % 2 == 0 {
            for _ in 0..repeats {
                compressed.push(j);
                k += 1;
            }
            j += 1;
        } else {
            for _ in 0..repeats {
                compressed.push(i32::MAX);
            }
        }
    }
    for i in 0..k {
        if compressed[i] == i32::MAX {
            compressed[i] = compressed.pop().unwrap();
        }
    }
    println!("{:?}", compressed);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_fragmenter_part_1_a() {
        assert_eq!(disk_fragmenter("data/day_09_a.txt"), 1928);
    }
}
