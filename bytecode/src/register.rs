

pub trait RegisterSize: Clone {
    const SIZE: usize;

    type Value: Copy;
}

#[derive(Debug, Clone)]
pub struct Reg8;
impl RegisterSize for Reg8 {
    const SIZE: usize = 8;

    type Value = u8;
}

#[derive(Debug, Clone)]
pub struct Reg16;
impl RegisterSize for Reg16 {
    const SIZE: usize = 16;

    type Value = u16;
}

#[derive(Debug, Clone)]
pub struct Reg32;
impl RegisterSize for Reg32 {
    const SIZE: usize = 32;

    type Value = u32;
}

#[derive(Debug, Clone)]
pub struct Reg64;
impl RegisterSize for Reg64 {
    const SIZE: usize = 64;

    type Value = u64;
}

#[derive(Debug, Clone)]
pub struct Reg<S: RegisterSize> {
    value: S::Value
}

impl<S: RegisterSize> Reg<S> {
    pub fn new(value: S::Value) -> Self {
        Self { value }
    }

    pub fn value(&self) -> S::Value {
        self.value
    }

    pub fn set_value(&mut self, value: S::Value) {
        self.value = value;
    }

    pub fn size() -> usize {
        S::SIZE
    }
}

impl<S: RegisterSize> Copy for Reg<S> {}

pub trait RegisterTake<S: RegisterSize> {
    fn take(&mut self, src: &mut Reg<S>) -> &mut Self;
}

macro_rules! infallible_takes {
    ( $( $name:ident, $src:ident );* $(;)? ) => {
        $(
            impl RegisterTake<$src> for Reg<$name> {
                fn take(&mut self, src: &mut Reg<$src>) -> &mut Self {
                    self.set_value(src.value().into());
                    self
                }
            }
        )*
    }
}

infallible_takes! {
    Reg8, Reg8;
    Reg16, Reg8;
    Reg32, Reg8;
    Reg64, Reg8;
    Reg16, Reg16;
    Reg32, Reg16;
    Reg64, Reg16;
    Reg32, Reg32;
    Reg64, Reg32;
    Reg64, Reg64;
}
