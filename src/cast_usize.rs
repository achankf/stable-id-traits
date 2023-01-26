use crate::CastUsize;

impl CastUsize for u8 {
    fn cast_to(self) -> usize {
        self as usize
    }

    fn cast_from(val: usize) -> Self {
        assert!(val < Self::max_value().into());

        val as Self
    }
}

impl CastUsize for u16 {
    fn cast_to(self) -> usize {
        self as usize
    }

    fn cast_from(val: usize) -> Self {
        assert!(val < Self::max_value().into());

        val as Self
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl CastUsize for u32 {
    fn cast_to(self) -> usize {
        self as usize
    }

    fn cast_from(val: usize) -> Self {
        assert!(val < Self::max_value() as usize);

        val as Self
    }
}

#[cfg(any(target_pointer_width = "64"))]
impl CastUsize for u64 {
    fn cast_to(self) -> usize {
        self as usize
    }

    fn cast_from(val: usize) -> Self {
        assert!(val < Self::max_value() as usize);

        val as Self
    }
}

impl CastUsize for usize {
    fn cast_to(self) -> usize {
        self
    }

    fn cast_from(val: usize) -> Self {
        val
    }
}
