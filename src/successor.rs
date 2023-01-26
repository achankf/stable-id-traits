use crate::Successor;

impl Successor for u8 {
    fn next_value(self) -> Self {
        assert!(self < Self::max_value());
        self + 1
    }
}

impl Successor for u16 {
    fn next_value(self) -> Self {
        assert!(self < Self::max_value());
        self + 1
    }
}

impl Successor for u32 {
    fn next_value(self) -> Self {
        assert!(self < Self::max_value());
        self + 1
    }
}

impl Successor for u64 {
    fn next_value(self) -> Self {
        assert!(self < Self::max_value());
        self + 1
    }
}

impl Successor for usize {
    fn next_value(self) -> Self {
        assert!(self < Self::max_value());
        self + 1
    }
}
