pub struct SwzRandom {
    index: usize, // this can only be 0-15... no need for usize
    state: [u32; 16],
}

impl SwzRandom {
    pub fn new(seed: u32) -> SwzRandom {
        let mut state = [0u32; 16];
        state[0] = seed;
        for i in 1..16 {
            let prev = state[i - 1];
            let mix1 = prev ^ (prev >> 30);
            let mix2 = mix1.wrapping_mul(0x6C078965u32);
            state[i] = mix2.wrapping_add(i as u32);
        }
        return SwzRandom {
            index: 0,
            state: state,
        };
    }

    pub fn next(&mut self) -> u32 {
        let new_index = (self.index + 15) % 16;
        // compute reslt
        let a1 = self.state[self.index];
        let b1 = self.state[(self.index + 13) % 16];
        let c = a1 ^ (a1 << 16) ^ b1 ^ (b1 << 15);
        let b2 = self.state[(self.index + 9) % 16];
        let b3 = b2 ^ (b2 >> 11);
        let a2 = b3 ^ c;
        let d = a2 ^ ((a2 << 5) & 0xDA442D24u32);
        let a3 = self.state[new_index];
        let result = a3 ^ (a3 << 2) ^ (b3 << 28) ^ c ^ (c << 18) ^ d;
        // update state
        self.state[self.index] = a2;
        self.state[new_index] = result;
        // update index
        self.index = new_index;
        // return
        return result;
    }
}
