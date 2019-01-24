const ROLLING_WINDOW_SIZE: usize = 7;

pub struct Hash {
    x: u32,
    y: u32,
    z: u32,
    c: u32,
    window: [u8; ROLLING_WINDOW_SIZE],
}

/// Pesudo-random value based on current context of input
impl Hash {
    pub fn new() -> Hash {
        Hash {
            x: 0,
            y: 0,
            z: 0,
            c: 0,
            window: [0; ROLLING_WINDOW_SIZE],
        }
    }

    pub fn sum(&self) -> u32 {
        self.x.wrapping_add(self.y.wrapping_add(self.z))
    }

    pub fn update(&mut self, byte: u8) {
        self.y -= self.x;
        self.y += ROLLING_WINDOW_SIZE as u32 * byte as u32;

        self.x += byte as u32;
        let idx = self.c as usize % ROLLING_WINDOW_SIZE;
        self.x -= self.window[idx] as u32;
        self.window[idx] = byte;

        self.c += 1;

        self.z <<= 5;
        self.z ^= byte as u32;
        // println!("{:?}", byte);
    }
}
