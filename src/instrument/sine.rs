use crate::dsp::env::*;
use crate::dsp::*;
use crate::instrument::*;
use crate::param::Param;
use std::iter::zip;

#[derive(Debug, Default)]
pub struct Sine {
	accum: f32,
	freq: Smoothed,
	vel: SmoothedEnv,
	sample_rate: f32,
	prev: f32,
	pub feedback: f32,
}

impl Instrument for Sine {
	fn new(sample_rate: f32) -> Self {
		Sine {
			freq: Smoothed::new(0.0, 50.0, sample_rate),
			vel: SmoothedEnv::new(0.0, 50.0, 20.0, sample_rate),
			sample_rate,
			..Default::default()
		}
	}

	fn cv(&mut self, pitch: f32, vel: f32) {
		let p = pitch_to_f(pitch, self.sample_rate);
		self.freq.set(p);
		self.vel.set(vel);
	}

	fn process(&mut self, buffer: &mut [&mut [f32]; 2]) {
		let [bl, br] = buffer;

		for (l, r) in zip(bl.iter_mut(), br.iter_mut()) {
			self.vel.update();
			self.freq.update();
			self.accum += self.freq.get();
			self.accum = self.accum.fract();
			let mut out = (self.accum * TWO_PI + self.feedback * self.prev).sin();
			out *= self.vel.get();

			self.prev = out;

			*l = out;
			*r = out;
		}
	}

	fn note(&mut self, pitch: f32, vel: f32, _id: usize) {
		let p = pitch_to_f(pitch, self.sample_rate);
		self.freq.set_hard(p);

		// self.vel.set_hard(vel);
		// self.accum = 0.0;

		if self.vel.get() < 0.01 {
			self.vel.set_hard(vel);
			self.accum = 0.0;
		} else {
			self.vel.set(vel);
		}
	}
}

impl Param for Sine {
	fn set_param(&mut self, index: usize, value: f32) {
		match index {
			0 => self.feedback = value,
			_ => eprintln!("Parameter with index {} not found", index),
		}
	}
}
