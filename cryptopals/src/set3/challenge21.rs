// == Seeding ==
const F: u32 = 0x6C078965;

// == Mersenne Twister parameters ==
const W: u32 = 32;
const N: usize = 624;
const M: usize = 397;
const R: u32 = 31;

const W_BITS: u32 = u32::MAX;

const LOWER_MASK: u32 = (1 << R) - 1;
const UPPER_MASK: u32 = W_BITS & !LOWER_MASK;

const A: u32 = 0x9908B0DF;

// = Tempering parameters =
const U: u32 = 11;
const D: u32 = 0xFFFFFFFF;
const S: u32 = 7;
const B: u32 = 0x9D2C5680;
const T: u32 = 15;
const C: u32 = 0xEFC60000;

const L: u32 = 18;

pub struct Twister {
    mt: [u32; N],
    index: usize,
}

impl Twister {
    pub fn new(seed: u32) -> Self {
        let mut mt = [0; N];
        mt[0] = seed;

        for i in 1..N {
            mt[i] = W_BITS & (F.wrapping_mul(mt[i - 1] ^ (mt[i - 1] >> (W - 2))) + i as u32);
        }

        Self { index: N, mt }
    }

    fn twist(&mut self) {
        for i in 0..N {
            let x = (self.mt[i] & UPPER_MASK) + (self.mt[(i + 1) % N] & LOWER_MASK);
            let x_a = if x % 2 == 0 { x >> 1 } else { (x >> 1) ^ A };

            self.mt[i] = self.mt[(i + M) % N] ^ x_a;
        }
    }
}

impl Default for Twister {
    fn default() -> Self {
        Twister::new(5489)
    }
}

impl Iterator for Twister {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= N {
            self.twist();
            self.index = 0;
        }

        let mut y = self.mt[self.index];

        self.index += 1;
        y = y ^ ((y >> U) & D);
        y = y ^ ((y << S) & B);
        y = y ^ ((y << T) & C);

        y = y ^ (y >> L);

        Some(y & W_BITS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let mut rng = Twister::default();

        let mut out = [0; 1000];
        for i in 0..1000 {
            out[i] = rng.next().unwrap();
        }

        let formatted = out.map(|entry| format!("{:08X}", entry)).join(" ");

        assert_eq!(formatted, include_str!("data/challenge21.txt"));
    }
}
