use crate::defs::*;
use crate::param::Param;

pub mod sine;

pub trait Instrument: Param {
	fn new(sample_rate: f32) -> Self
	where
		Self: Sized;
	fn cv(&mut self, pitch: f32, vel: f32);
	fn process(&mut self, buffer: &mut [StereoSample]);
	fn note(&mut self, pitch: f32, vel: f32, id: usize);
}

pub trait Effect: Param {
	fn new(sample_rate: f32) -> Self
	where
		Self: Sized;
	fn process(&mut self, buffer: &mut [StereoSample]);
}
