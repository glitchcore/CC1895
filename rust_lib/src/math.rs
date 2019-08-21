pub trait Math {
	fn sin(self) -> Self;
	fn cos(self) -> Self;
}

impl Math for f32 {
    fn sin(self) -> Self {
    	unsafe {core::intrinsics::sinf32(self)}
    }

    fn cos(self) -> Self {
    	unsafe {core::intrinsics::cosf32(self)}
    }
}