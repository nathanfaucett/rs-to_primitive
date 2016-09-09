#![no_std]


pub trait ToPrimitive {
    fn to_usize(self) -> usize;
    fn to_u8(self) -> u8;
    fn to_u16(self) -> u16;
    fn to_u32(self) -> u32;
    fn to_u64(self) -> u64;

    fn to_isize(self) -> isize;
    fn to_i8(self) -> i8;
    fn to_i16(self) -> i16;
    fn to_i32(self) -> i32;
    fn to_i64(self) -> i64;

    fn to_f32(self) -> f32;
    fn to_f64(self) -> f64;
}

macro_rules! to_primitive {
    ($n:ident, $t:ident) => (
        #[inline(always)]
        fn $n(self) -> $t { self as $t }
    );
}

macro_rules! trait_to_primitive {
    ($t:ident) => (
        impl ToPrimitive for $t {
            to_primitive!(to_usize, usize);
            to_primitive!(to_u8, u8);
            to_primitive!(to_u16, u16);
            to_primitive!(to_u32, u32);
            to_primitive!(to_u64, u64);

            to_primitive!(to_isize, isize);
            to_primitive!(to_i8, i8);
            to_primitive!(to_i16, i16);
            to_primitive!(to_i32, i32);
            to_primitive!(to_i64, i64);

            to_primitive!(to_f32, f32);
            to_primitive!(to_f64, f64);
        }
    );
}

trait_to_primitive!(usize);
trait_to_primitive!(u8);
trait_to_primitive!(u16);
trait_to_primitive!(u32);
trait_to_primitive!(u64);

trait_to_primitive!(isize);
trait_to_primitive!(i8);
trait_to_primitive!(i16);
trait_to_primitive!(i32);
trait_to_primitive!(i64);

trait_to_primitive!(f32);
trait_to_primitive!(f64);


#[test]
fn test() {
    assert_eq!(10_f32.to_usize(), 10_usize);
    assert_eq!(10_u8.to_i8(), 10_i8);
    assert_eq!(10_f64.to_f64(), 10_f64);
}
