pub struct Music {
    sound_phase: f32,
    pub kick_freq: f32,
    pub kick_phase: f32,
    pub freq_idx: usize,
    pub bass_idx: usize,
}

impl Music {
    pub const fn new() -> Self {
        Music {
            sound_phase: 0.0,
            freq_idx: 0,
            bass_idx: 0,
            kick_phase: 0.0,
            kick_freq: 0.0,
        }
    }


    pub fn get_freq(&mut self, fs: f32) -> f32 {
        let freqs: [f32;15] = [
            750.0, 250.0, 500.0, 250.0, 500.0, 250.0, 750.0,
            250.0, 1000.0, 250.0, 500.0, 250.0, 840.0, 250.0, 890.0
        ];

        self.sound_phase += 1.0/fs;

        self.kick_phase += 1.0/fs * self.kick_freq;

        if self.sound_phase > 0.1 {
            self.sound_phase = 0.0;
            self.freq_idx += 1;

            if self.freq_idx % 25 == 0 {
                self.bass_idx += 1;
            }

            self.kick_freq = 1000.0;
        }

        if self.kick_freq > 100.0 {
            self.kick_freq -= 1.0/fs * 6000.0;
        }

        let freq = freqs[self.freq_idx % freqs.len()] * match self.bass_idx % 3 {
            0 => 1.0,
            1 => 0.8,
            2 => 0.86,
            _ => 1.0,
        };

        return freq;
    }
}