#![allow(dead_code)]
use rand::prelude::*;
use std::fs;
use std::io::prelude::*;
use log::{info, debug};

#[derive(Debug, Copy, Clone)]
/*
    DrumGenSequence is a struct that represents a sequence of 32 steps.
    Each step can have a value between 0 - 10 that determines the probability of the step (also from 0-10).
    The sequence is stored in an array of 32 u8 values.
*/
pub struct DrumGenSequence {
    pub steps: [u8; 32],
}

impl DrumGenSequence {
    pub fn new() -> DrumGenSequence {
        DrumGenSequence {
            steps: [0; 32],
        }
    }
    
    pub fn set_step(&mut self, step: usize, value: u8) {
        match step {
            0..=31 => {
                let final_value = match value {
                    0..=15 => value,
                    16..=255 => 15,
                };
                self.steps[step] = final_value
            },
            _ => panic!("Step out of range"),
        }
    }

    /*
        Encode the sequence to a byte array.
    */
    pub fn convert(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![0; 16];
        for i in 0..self.steps.len() {
            let step_value = self.steps[i];
            let index = i / 2;
            if i % 2 == 0 {
                result[index] = (step_value % 16) & 15;
            } else {
                let mut value = (step_value % 16) & 15;
                value = value << 4;
                result[index] = result[index] | value;
            }

        }
        result
    }

    pub fn decode(&mut self, data: Vec<u8>) {
        debug!("Decoding data: {:?} ({} bytes)", data, data.len());
        for i in 0..(self.steps.len() - 1) {
            let index = i / 2;
            let value = data[index];
            debug!("i: {}, index: {}, value: {}", i, index, value);
            if i % 2 == 0 {
                self.steps[i] = value & 15;
            } else {
                self.steps[i] = value >> 4;
            }
        }
    }

    pub fn randomize(&mut self, min: u8, max: u8, probability: u8) {
        // Create random object
        let mut rng = rand::thread_rng();
        for i in 0..self.steps.len() {
            // Generate random value between 0 - 10
            if rng.gen_range(0..100) < probability {
                if min > max {
                    let value : u8 = rng.gen_range(max..min) as u8;
                    self.set_step(i, value);

                } else 
                if max - min == 0 {
                    self.set_step(i, min);
                } else {
                    let value : u8 = rng.gen_range(min..max) as u8;
                    self.set_step(i, value);
                }
            }
        }
    }

    pub fn repeat(&mut self, start: usize, steps: usize) {
        for i in start..self.steps.len() {
            let step = self.steps[start + ((i - start) % steps)];
            self.set_step(i, step);
        }
    }

    pub fn get_step(&self, step: usize) -> u8 {
        if step < self.steps.len() {
            self.steps[step]
        } else {
            0
        }
    }

    pub fn shift(&mut self, value: i8) {
        let mut new_steps: [u8; 32] = [0; 32];
        for i in 0..self.steps.len() {
            let index: i16 = (i as i16 + value as i16) % self.steps.len() as i16;
            if index < 0 {
                new_steps[i] = self.steps[(self.steps.len() as i16 + index) as usize];
            } else {
                new_steps[i] = self.steps[index as usize];
            }
        }
        self.steps = new_steps;
    }

    pub fn add(&mut self, value:i8) {
        for i in 0..self.steps.len() {
            let mut new_value = self.steps[i] as i8 + value;
            new_value = {
                if new_value < 0 {
                    0
                } else if new_value > 10 {
                    10
                } else {
                    new_value
                }
            };
            self.steps[i] = new_value as u8;
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.steps.len() {
            self.steps[i] = 0;
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DrumGenLayer {
    pub sequence: [DrumGenSequence; 3]
}

impl DrumGenLayer {
    pub fn new() -> DrumGenLayer {
        DrumGenLayer {
            sequence: [DrumGenSequence::new(); 3]
        }
    }

    pub fn convert(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![0; 48];
        for i in 0..self.sequence.len() {
            let sequence = &self.sequence[i];
            let sequence_result = sequence.convert();
            for j in 0..sequence_result.len() {
                let index = j + (i * 16);
                result[index] = sequence_result[j];
            }
        }
        result
    }

    pub fn decode(&mut self, data: Vec<u8>) {
        for i in 0..self.sequence.len() {
            let mut sequence_data: Vec<u8> = Vec::new();
            for j in 0..16 {
                let index = j + (i * 16);
                sequence_data.push(data[index]);
            }
            self.sequence[i].decode(sequence_data);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DrumGen {
    pub parts: [DrumGenLayer; 4]
}

impl DrumGen {
    pub fn new() -> DrumGen {
        DrumGen {
            parts: [DrumGenLayer::new(); 4]
        }
    }

    pub fn convert(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        for i in 0..self.parts.len() {
            let layer = &self.parts[i];
            let layer_result = layer.convert();
            for j in 0..layer_result.len() {
                result.push(layer_result[j]);
            }
        }
        result
    }

    pub fn parse_file(&mut self, filepath: String) {
        let mut file = fs::File::open(&filepath).unwrap();

        // Read file content and parse to bytes
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).unwrap();
        info!("Parsing file: {}", filepath);
        info!("File size: {}", data.len());
        debug!("File content: {:?}", data);
        self.decode(data);
    }

    pub fn save_file(&self, filepath: String) {
        let mut file = fs::File::create(&filepath).unwrap();
        let data = self.convert();
        // Save data to file
        info!("Saving file to: {}", filepath);
        let _ = file.write_all(&data);
        info!("File size: {}", data.len());
        debug!("File content: {:?}", data);
    }

    pub fn decode(&mut self, data: Vec<u8>) {
        for i in 0..self.parts.len() {
            let mut layer_data: Vec<u8> = Vec::new();
            for j in 0..48 {
                let index = j + (i * 48);
                layer_data.push(data[index]);
            }
            self.parts[i].decode(layer_data);
        }
    }


}

