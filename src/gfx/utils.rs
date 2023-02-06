#[derive(Default)]
pub struct BitFields<T> {
    bits: T,
}

impl<
        T: std::ops::BitAndAssign
            + std::ops::BitAnd
            + std::ops::BitOrAssign
            + std::ops::BitXorAssign
            + std::ops::Not<Output = T>
            + Default,
    > BitFields<T>
where
    <T as std::ops::BitAnd>::Output: PartialEq<T>,
    T: Copy,
{
    pub fn is_set(&self, flag: T) -> bool {
        self.bits & flag != T::default()
    }

    pub fn set(&mut self, flag: T) {
        self.bits |= flag
    }

    pub fn unset(&mut self, flag: T) {
        self.bits &= !flag
    }

    #[allow(dead_code)]
    pub fn toggle(&mut self, flag: T) {
        self.bits ^= flag
    }
}
