use super::macros::seqs;

fn fmt(bytes: &[u8]) -> &str {
    str::from_utf8(bytes).unwrap()
}

pub fn example() {
    let strings = seqs![
        b"CDEFG",
        b"CDEF",
        b"CDE",
        b"CD",
        b"C",
        b"BCDEG",
        b"BCDEF",
        b"BCDE",
        b"BCD",
        b"BC",
        b"B",
        b"AAAAAAAAA",
        b"ABCDE",
        b"ABCD",
        b"ABC",
        b"AB",
        b"A",
        b"EDCBA",
        b"DCBA",
        b"CBA",
        b"BA",
    ];
    let alpha_start = b'A';
    let sigma = (b'Z' - b'A' + 1) as usize;

    let order = lsd_sort(&strings, alpha_start, sigma);
    println!("before:");
    for string in &strings {
        println!("{}", fmt(string));
    }
    println!("\nafter:");
    for string_index in order {
        println!("{}", fmt(&strings[string_index]));
    }
}

fn lsd_sort(strings: &[Vec<u8>], alpha_start: u8, sigma: usize) -> Vec<usize> {
    let n = strings.len();

    let mut output = vec![0_usize; n];
    let mut rounds = 0_usize;

    let mut pointers = vec![0_usize; sigma + 1];
    for string in strings {
        let bucket = (string[0] - alpha_start) as usize;
        pointers[bucket + 1] += 1;
        rounds = rounds.max(string.len());
    }
    for bucket in 1..sigma {
        pointers[bucket] += pointers[bucket - 1];
    }

    let mut index_buffer_a = Vec::<usize>::with_capacity(n);
    index_buffer_a.extend(0..n);
    let mut index_buffer_b = vec![0_usize; n];
    let mut buckets = vec![0_usize; sigma + 1];

    for round in 1..=rounds {
        let mut count_to_sort = 0;
        for &string_index in &index_buffer_a {
            let string = &strings[string_index];
            let char_index = string.len() - round;
            let bucket = (string[char_index] - alpha_start) as usize;
            if char_index == 0 {
                let pointer = pointers[bucket];
                pointers[bucket] += 1;
                output[pointer] = string_index;
            } else {
                buckets[bucket + 1] += 1;
                count_to_sort += 1;
            }
        }
        for bucket in 1..sigma {
            buckets[bucket] += buckets[bucket - 1];
        }
        index_buffer_b.resize(count_to_sort, 0);
        for &string_index in &index_buffer_a {
            let string = &strings[string_index];
            let char_index = string.len() - round;
            let bucket = (string[char_index] - alpha_start) as usize;
            if char_index > 0 {
                let pointer = buckets[bucket];
                buckets[bucket] += 1;
                index_buffer_b[pointer] = string_index;
            }
        }
        std::mem::swap(&mut index_buffer_a, &mut index_buffer_b);
        #[allow(clippy::needless_range_loop)]
        for bucket in 0..sigma {
            buckets[bucket] = 0;
        }
    }

    output
}

