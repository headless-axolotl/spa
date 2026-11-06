#![allow(unused)]

mod macros;
mod vlen_lsd_radix;

enum Examples {
    VlenLsdRadix,
}

impl std::str::FromStr for Examples {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            "lsd-radix" => Self::VlenLsdRadix,
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
        Examples::VlenLsdRadix => vlen_lsd_radix::example(),
    }
}

