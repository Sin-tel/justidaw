pub const TWO_PI: f32 = std::f32::consts::TAU;
pub const MAX_BUF_SIZE: usize = 1024;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct StereoSample {
	pub l: f32,
	pub r: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct C_AudioBuffer {
	pub ptr: *mut f64,
	pub len: usize,
	pub cap: usize,
}

// Message struct to pass to audio thread
// Should not contain any boxed values
#[derive(Debug)]
pub enum AudioMessage {
	CV(usize, CV),
	Note(usize, CV, usize),
	SetParam(usize, usize, usize, f32),
	Pan(usize, f32, f32),
	Mute(usize, bool),
}

#[derive(Debug)]
pub struct CV {
	pub pitch: f32,
	pub vel: f32,
}

#[repr(C)]
#[derive(Debug)]
pub enum LuaMessage {
	Cpu(f32),
	Meter(f32, f32),
}
