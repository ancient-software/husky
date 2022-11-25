use crate::*;

pub struct WordMenu {
    i32: Word,
    i64: Word,
    b32: Word,
    b64: Word,
    f32: Word,
    f64: Word,
}

impl WordMenu {
    pub fn i32(&self) -> Word {
        self.i32
    }

    pub fn i64(&self) -> Word {
        self.i64
    }

    pub fn b32(&self) -> Word {
        self.b32
    }

    pub fn b64(&self) -> Word {
        self.b64
    }

    pub fn f32(&self) -> Word {
        self.f32
    }

    pub fn f64(&self) -> Word {
        self.f64
    }
}
