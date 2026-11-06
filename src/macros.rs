
macro_rules! seqs {
    ($($seq:expr),* $(,)?) => {
        vec![$($seq.to_vec(),)*]
    };
}

pub(crate) use seqs;

