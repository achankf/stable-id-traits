use crate::Predecessor;

impl Predecessor for u8 {
    fn prev_value(self) -> Self {
        assert!(self > 0);
        self - 1
    }
}

impl Predecessor for u16 {
    fn prev_value(self) -> Self {
        assert!(self > 0);
        self - 1
    }
}

impl Predecessor for u32 {
    fn prev_value(self) -> Self {
        assert!(self > 0);
        self - 1
    }
}

impl Predecessor for u64 {
    fn prev_value(self) -> Self {
        assert!(self > 0);
        self - 1
    }
}

impl Predecessor for usize {
    fn prev_value(self) -> Self {
        assert!(self > 0);
        self - 1
    }
}
