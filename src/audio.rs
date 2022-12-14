pub use std::time::Duration;
pub use rodio::Source;

pub struct WavetableOscillator {
    wave_table: Vec<f32>,
    table_size: usize,
    sample_rate: u32,
    table_deltas: Vec<f32>,
    table_indexes: Vec<f32>
}

impl WavetableOscillator {
    pub fn new(table_size: usize, sample_rate: u32) -> WavetableOscillator {
        let wave_table = generate_wave_table(table_size);
        return WavetableOscillator {
            wave_table,
            table_size,
            sample_rate,
            table_deltas: Vec::new(),
            table_indexes: Vec::new()
        };
    }

    pub fn add_frequency(&mut self, frequency: f32) {
        let table_delta = frequency * self.table_size as f32 / self.sample_rate as f32;
        self.table_deltas.push(table_delta);
        self.table_indexes.push(0.0);
    }

    pub fn clear_frequencies(&mut self) {
        self.table_deltas.clear();
        self.table_indexes.clear();
    }

    pub fn get_next_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        for index in 0..self.table_deltas.len() {
            let current_index = self.table_indexes[index] as usize;
            let next_index = (current_index + 1) % self.table_size;
            let lerp_frac = self.table_indexes[index] - current_index as f32;
            let current_value = self.wave_table[current_index];
            let next_value = self.wave_table[next_index];
            let lerp_value = current_value + lerp_frac * (next_value - current_value);
            sample += lerp_value / self.table_deltas.len() as f32;
            self.table_indexes[index] += self.table_deltas[index];
            self.table_indexes[index] %= self.table_size as f32;
        }
        return sample;
    }
}

pub fn generate_wave_table(table_size: usize) -> Vec<f32> {
    let mut wave_table: Vec<f32> = Vec::with_capacity(table_size);
    for i in 0..table_size {
        let time_value = 2.0 * std::f32::consts::PI * i as f32 / table_size as f32;
        let wave_value = time_value.sin();
        wave_table.push(wave_value);
    }
    return wave_table;
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_next_sample());
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}