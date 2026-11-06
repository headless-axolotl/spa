#![allow(unused)]

mod macros;
mod vlen_lsb_radix;

enum Examples {
    VlenLsbRadix,
}

impl std::str::FromStr for Examples {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            "lsb-radix" => Self::VlenLsbRadix,
            _ => return Err(()),
        };
        Ok(value)
    }
}

fn main() {
    let mut args = std::env::args();
    let example: Examples = args
        .nth(1).expect("expected example argument")
        .parse().expect("expected example");
    match example {
        Examples::VlenLsbRadix => vlen_lsb_radix::example(),
    }
}

