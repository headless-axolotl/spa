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
        b"AAAAAAAAA",
        b"ABCDE",
        b"ABCD",
        b"ABC",
        b"AB",
        b"AB",
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
        println!("{:3}: {}", string_index, fmt(&strings[string_index]));
    }
}

fn lsd_sort(strings: &[Vec<u8>], alpha_start: u8, sigma: usize) -> Vec<usize> {
    let n = strings.len();
    assert!(n > 0);

    let mut length_sorted_indices = Vec::<usize>::with_capacity(n);
    // When we do count sorts by character we initially add the strings with length == round from
    // back to front. To preserve the stability of the algorithm reverse the indices initially.
    length_sorted_indices.extend((0..n).rev());
    let mut output = Vec::<usize>::with_capacity(n);
    let mut buffer = vec![0_usize; n];

    let rounds = strings.iter().map(|string| string.len()).max().unwrap();
    let mut length_buckets = vec![0_usize; rounds + 1];
    count_sort(&mut length_sorted_indices, &mut buffer, &mut length_buckets, |index| strings[index].len() - 1);

    let mut previously_added = n;
    let mut char_buckets = vec![0_usize; sigma + 2];
    for round in (0..rounds).rev() {
        loop {
            if previously_added == 0 {
                break;
            }
            let index_to_add = length_sorted_indices[previously_added - 1];
            if strings[index_to_add].len() >= round {
                output.push(index_to_add);
                previously_added -= 1;
            } else {
                break;
            }
        }
        buffer.resize(output.len(), 0);
        count_sort(
            &mut output,
            &mut buffer,
            &mut char_buckets,
            |index| {
                if round >= strings[index].len() {
                    0 // imaginary character before the start of the alphabet
                } else {
                    (strings[index][round] - alpha_start) as usize + 1
                }
            }
        );
    }

    output
}

fn count_sort<F: Fn(usize) -> usize>(
    buffer_a: &mut Vec<usize>,
    buffer_b: &mut Vec<usize>,
    buckets: &mut [usize],
    index_to_bucket: F
) {
    for &index in buffer_a.iter() {
        buckets[index_to_bucket(index) + 1] += 1;
    }
    for bucket in 1..buckets.len() {
        buckets[bucket] += buckets[bucket - 1];
    }
    for &index in buffer_a.iter() {
        let bucket = index_to_bucket(index);
        let position = buckets[bucket];
        buckets[bucket] += 1;
        buffer_b[position] = index;
    }
    for bucket in buckets {
        *bucket = 0;
    }
    std::mem::swap(buffer_a, buffer_b)
}

