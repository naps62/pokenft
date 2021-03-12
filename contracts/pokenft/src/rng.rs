use rand::distributions::WeightedIndex;
// use rand::RngCore;
use rand::prelude::*;
use rand_chacha::ChaChaRng;
// use rand_distr::Normal;

// indexes are pokemon IDs. values are weights within entire population.
// e.g. #0 is Bulbasaur, which has 376 specimen within a total population of 10 million
pub const POKEMON_LIST: [u32; 151] = [
    376, 251, 188, 376, 251, 188, 376, 251, 188, 187967, 9398, 4699, 187967, 9398, 4699, 187967,
    187967, 187967, 187967, 187967, 187967, 187967, 187967, 187967, 376, 342, 187967, 187967,
    187967, 187967, 187967, 187967, 187967, 31328, 93983, 23496, 37593, 18797, 187967, 18797,
    187967, 187967, 187967, 93983, 187967, 187967, 93983, 187967, 93983, 187967, 18797, 187967,
    93983, 187967, 187967, 187967, 37593, 18797, 9398, 37593, 18797, 9398, 37593, 18797, 9398,
    37593, 18797, 9398, 37593, 18797, 9398, 187967, 93983, 187967, 93983, 46992, 46992, 31328,
    187967, 93983, 187967, 62656, 62656, 187967, 62656, 187967, 62656, 187967, 62656, 31328, 18797,
    37593, 18797, 9398, 187967, 187967, 46992, 187967, 62656, 187967, 46992, 23496, 18797, 46992,
    26852, 6266, 6266, 9398, 18797, 9398, 18797, 9398, 9398, 9398, 9398, 7519, 4699, 7519, 4699,
    7519, 4699, 3759, 3759, 3759, 3759, 3759, 3759, 4699, 3759, 3759, 3759, 3759, 3759, 1880, 1880,
    1880, 1880, 1213, 964, 1213, 964, 964, 964, 188, 188, 188, 627, 470, 372, 31, 20,
];

type Seed = [u8; 32];

#[derive(Debug, PartialEq)]
pub enum Error {
    SeedNot32Bytes,
}

pub trait IntoSeed {
    fn into_seed(&self) -> Result<Seed, Error>;
}

impl IntoSeed for Seed {
    fn into_seed(&self) -> Result<Seed, Error> {
        Ok(*self)
    }
}

impl IntoSeed for String {
    fn into_seed(&self) -> Result<Seed, Error> {
        let mut seed = [0u8; 32];

        let bytes = self.as_bytes();

        if bytes.len() != 32 {
            return Err(Error::SeedNot32Bytes);
        }

        for (i, v) in bytes.iter().enumerate() {
            seed[i] = *v as u8;
        }

        Ok(seed)
    }
}

pub fn sample<T>(seed: T) -> Result<u32, Error>
where
    T: IntoSeed,
{
    let mut rng = ChaChaRng::from_seed(seed.into_seed()?);
    let dist = WeightedIndex::new(&POKEMON_LIST).unwrap();

    Ok(dist.sample(&mut rng) as u32 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let result = sample([8u8; 32]);

        assert_eq!(result, Ok(80));
    }

    #[test]
    fn test_with_string() {
        let result = sample(String::from("12345678901234567890123456789012"));

        assert_eq!(result, Ok(23));
    }

    #[test]
    fn test_with_short_string() {
        let result = sample(String::from("1"));

        assert_eq!(result, Err(Error::SeedNot32Bytes));
    }

    #[test]
    fn test_with_long_string() {
        let result = sample(String::from("1234567890123456789012345678901245"));

        assert_eq!(result, Err(Error::SeedNot32Bytes));
    }
}
