use std::fmt;

pub trait Currency {
    const SYMBLE: &'static str;
    const CODE: &'static str;
    const RITIO: u8;
}
#[derive(Debug, Copy, Clone)]
pub struct Amount<C> {
    value: u32,
    _phantom: std::marker::PhantomData<C>,
}

impl<C: Currency> Amount<C> {
    pub fn new(value: u32, _currency: C) -> Self {
        Amount {
            value,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub trait Convert<T, F> {
    fn convert(amount: Amount<T>) -> Amount<F>;
}

impl<T: Currency + Forex<F>, F: Currency> Convert<T, F> for Amount<F>
where
    T: Forex<F>,
{
    fn convert(amount: Amount<T>) -> Amount<F> {
        Amount {
            value: (amount.value as f64 * T::INTO_RATIO as f64) as u32, // Example conversion logic
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<T: Currency> Amount<T> {
    pub fn to<F: Currency>(self) -> Amount<F>
    where
        T: Forex<F>,
    {
        Amount::<F>::convert(self)
    }
}

impl<C> fmt::Display for Amount<C>
where
    C: Currency,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let major = self.value / C::RITIO as u32;
        let minor = self.value % C::RITIO as u32;
        if f.alternate() {
            write!(f, "{}.{}{}", major, minor, C::CODE)
        } else {
            write!(f, "{}.{}{}", major, minor, C::SYMBLE)
        }
    }
}

// implement add operator
impl<C: Currency> std::ops::Add for Amount<C> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Amount {
            value: self.value + other.value,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub trait Forex<A> {
    const INTO_RATIO: f64;
}

// #[derive(Debug)]
// // pub struct Wrapper<C>{Amount<C>};
// pub struct Convert<T: Currency>(pub Amount<T>);

// impl<T: Currency, F: Currency> From<Convert<T>> for Amount<F> {
//     fn from(w: Convert<T>) -> Self {
//         Amount {
//             value: (w.0.value as f64 * 0.06) as u32,
//             _phantom: std::marker::PhantomData,
//         }
//     }
// }

// impl<T: Currency, F: Currency> Into<Convert<T>> for Amount<F> {
//     fn into(self) -> Convert<T> {
//         Convert(Amount {
//             value: self.value * 2,
//             _phantom: std::marker::PhantomData,
//         })
//     }
// }

// impl<T: Currency, F: Currency> Into<Amount<T>> for Convert<F> {
//     fn into(self) -> Amount<T> {
//         Amount {
//             value: self.0.value * 2,
//             _phantom: std::marker::PhantomData,
//         }
//     }
// }
