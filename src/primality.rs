use std::{num::NonZeroU32, str::FromStr, sync::OnceLock};

#[inline]
pub fn small_primes() -> &'static [NonZeroU32] {
    static SMALL_PRIMES: OnceLock<Box<[NonZeroU32]>> = OnceLock::new();

    #[cold]
    fn read_small_primes() -> Box<[NonZeroU32]> {
        let mut primes = include_str!("./small-primes.txt")
            .lines()
            .map(str::trim)
            .filter(|&line| {
                line.starts_with(|c: char| c.is_ascii_digit())
                    & line.ends_with(|c: char| c.is_ascii_digit())
            })
            .flat_map(|line| line.split_ascii_whitespace())
            .filter_map(|n| NonZeroU32::from_str(n).ok())
            .collect::<Vec<_>>();

        primes.sort();
        primes.dedup();

        assert!(!primes.is_empty(), "Small prime list cannot be empty!");

        primes.into()
    }

    &*SMALL_PRIMES.get_or_init(read_small_primes)
}

pub struct IsPrime {
    small: &'static [NonZeroU32],
}

impl IsPrime {
    pub fn new() -> Self {
        Self {
            small: small_primes(),
        }
    }

    #[inline]
    pub fn first_factor(&self, u: u32) -> u32 {
        self.small
            .iter()
            .copied()
            .find_map(|f| (u % f == 0).then_some(f.get()))
            .or_else(|| {
                (1u32..)
                    .map(|m| 2 * m + 1)
                    .take_while(|m| m * m <= u)
                    .find(|m| u.checked_rem(*m).is_some_and(|r| r == 0))
            })
            .unwrap_or(u)
    }

    #[inline]
    pub fn is_prime(&self, u: u32) -> bool {
        if u <= 1 {
            return false;
        }

        self.first_factor(u) == u
    }
}
