pub use libfake::*;

pub trait FluentFake {
    fn faked() -> Self;
}

impl<T> FluentFake for T
where
    T: Dummy<Faker>,
{
    fn faked() -> Self {
        Self::dummy(&Faker)
    }
}