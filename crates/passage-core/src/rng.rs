//! Générateur pseudo-aléatoire déterministe (même graine = mêmes grilles).

pub struct Rng(u64);

impl Rng {
    /// Deux numéros différents donnent toujours deux suites différentes
    /// (le numéro est brassé avec splitmix64 avant de servir de graine).
    pub fn new(seed: u64) -> Self {
        let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        Rng((z ^ (z >> 31)) | 1)
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// Entier dans `0..max`.
    pub fn below(&mut self, max: usize) -> usize {
        (self.next_u64() % max as u64) as usize
    }

    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            items.swap(i, self.below(i + 1));
        }
    }
}
