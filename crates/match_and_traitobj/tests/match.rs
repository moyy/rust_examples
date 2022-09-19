// 结论：fn 最快，枚举次之，traitobject最慢
// 12个项，各1倍

#![feature(test)]
extern crate test;

use test::Bencher;

#[bench]
fn bench_fn(b: &mut Bencher) {
    fn compute(v: f64) -> f64 {
        v
    }

    let s = [
        1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
    ];

    let mut r = R(0.0);
    b.iter(|| {
        r = R(0.0);

        // 一次迭代 12万条
        for _ in 0..10000 {
            for v in s.iter() {
                r.0 += compute(*v);
            }
        }
    });
    println!("bench_fn, r = {:?}", r);
}

#[bench]
fn bench_method(b: &mut Bencher) {
    struct Data(f64);

    impl Data {
        fn compute(&self) -> f64 {
            self.0
        }
    }

    let s = [
        Data(1.0),
        Data(2.0),
        Data(3.0),
        Data(4.0),
        Data(5.0),
        Data(6.0),
        Data(7.0),
        Data(8.0),
        Data(9.0),
        Data(10.0),
        Data(11.0),
        Data(12.0),
    ];

    let mut r = R(0.0);
    b.iter(|| {
        r = R(0.0);

        // 一次迭代 12万条
        for _ in 0..10000 {
            for v in s.iter() {
                r.0 += v.compute();
            }
        }
    });
    println!("bench_method, r = {:?}", r);
}

#[bench]
fn bench_match_enum(b: &mut Bencher) {
    let s = [
        1i32.into(),
        2i64.into(),
        3u32.into(),
        4u64.into(),
        5.0f32.into(),
        6.0f64.into(),
        7i8.into(),
        8u8.into(),
        9i16.into(),
        10u16.into(),
        11i128.into(),
        12u128.into(),
    ];

    let mut r = R(0.0);
    b.iter(|| {
        r = R(0.0);
        // 一次迭代 12万条
        for _ in 0..10000 {
            for v in s.iter() {
                r.0 += match v {
                    TestData::F32(v) => *v as f64,
                    TestData::F64(v) => *v as f64,
                    TestData::U32(v) => *v as f64,
                    TestData::I32(v) => *v as f64,
                    TestData::U64(v) => *v as f64,
                    TestData::I64(v) => *v as f64,
                    TestData::I8(v) => *v as f64,
                    TestData::U8(v) => *v as f64,
                    TestData::I16(v) => *v as f64,
                    TestData::U16(v) => *v as f64,
                    TestData::I128(v) => *v as f64,
                    TestData::U128(v) => *v as f64,
                }
            }
        }
    });
    println!("bench_match_enum, r = {:?}", r);
}

#[bench]
fn bench_fn_trait(b: &mut Bencher) {
    let s: [&dyn Fn() -> f64; 12] = [
        &move || 1.0,
        &move || 2.0,
        &move || 3.0,
        &move || 4.0,
        &move || 5.0,
        &move || 6.0,
        &move || 7.0,
        &move || 8.0,
        &move || 9.0,
        &move || 10.0,
        &move || 11.0,
        &move || 12.0,
    ];

    let mut r = R(0.0);
    b.iter(|| {
        r = R(0.0);

        // 一次迭代 12万条
        for _ in 0..10000 {
            for v in s.iter() {
                r.0 += v();
            }
        }
    });
    println!("bench_fn_trait, r = {:?}", r);
}

#[bench]
fn bench_1(b: &mut Bencher) {
    let s = [
        move || 1.0,
        move || 2.0,
        move || 3.0,
        move || 4.0,
        move || 5.0,
        move || 6.0,
        move || 7.0,
        move || 8.0,
        move || 9.0,
        move || 10.0,
        move || 11.0,
        move || 12.0,
    ];

    let mut r = R(0.0);
    b.iter(|| {
        r = R(0.0);

        // 一次迭代 12万条
        for _ in 0..10000 {
            for v in s.iter() {
                r.0 += v();
            }
        }
    });
    println!("bench_1, r = {:?}", r);
}

#[bench]
fn bench_traitobj(b: &mut Bencher) {
    let s: [Box<dyn TestTrait>; 12] = [
        Box::new(1i32),
        Box::new(2i64),
        Box::new(3u32),
        Box::new(4u64),
        Box::new(5.0f32),
        Box::new(6.0f64),
        Box::new(7i8),
        Box::new(8u8),
        Box::new(9i16),
        Box::new(10u16),
        Box::new(11i128),
        Box::new(12u128),
    ];

    let mut r = R(0.0);
    b.iter(|| {
        r = R(0.0);

        // 一次迭代 12万条
        for _ in 0..10000 {
            for v in s.iter() {
                r.0 += v.compute();
            }
        }
    });
    println!("bench_traitobj, r = {:?}", r);
}

#[derive(Debug)]
struct R(f64);

#[derive(Debug)]
enum TestData {
    F32(f32),
    F64(f64),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I128(i128),
    U128(u128),
}

impl From<f32> for TestData {
    fn from(v: f32) -> Self {
        TestData::F32(v)
    }
}

impl From<f64> for TestData {
    fn from(v: f64) -> Self {
        TestData::F64(v)
    }
}

impl From<i32> for TestData {
    fn from(v: i32) -> Self {
        TestData::I32(v)
    }
}

impl From<u32> for TestData {
    fn from(v: u32) -> Self {
        TestData::U32(v)
    }
}

impl From<i64> for TestData {
    fn from(v: i64) -> Self {
        TestData::I64(v)
    }
}

impl From<u64> for TestData {
    fn from(v: u64) -> Self {
        TestData::U64(v)
    }
}

impl From<i8> for TestData {
    fn from(v: i8) -> Self {
        TestData::I8(v)
    }
}

impl From<u8> for TestData {
    fn from(v: u8) -> Self {
        TestData::U8(v)
    }
}

impl From<i16> for TestData {
    fn from(v: i16) -> Self {
        TestData::I16(v)
    }
}

impl From<u16> for TestData {
    fn from(v: u16) -> Self {
        TestData::U16(v)
    }
}

impl From<i128> for TestData {
    fn from(v: i128) -> Self {
        TestData::I128(v)
    }
}

impl From<u128> for TestData {
    fn from(v: u128) -> Self {
        TestData::U128(v)
    }
}

trait TestTrait {
    fn compute(&self) -> f64;
}

impl TestTrait for f32 {
    fn compute(&self) -> f64 {
        *self as f64
    }
}

impl TestTrait for f64 {
    fn compute(&self) -> f64 {
        *self as f64
    }
}

impl TestTrait for i32 {
    fn compute(&self) -> f64 {
        *self as f64
    }
}

impl TestTrait for u32 {
    fn compute(&self) -> f64 {
        *self as f64
    }
}

impl TestTrait for i64 {
    fn compute(&self) -> f64 {
        *self as f64
    }
}

impl TestTrait for u64 {
    fn compute(&self) -> f64 {
        *self as f64
    }
}
impl TestTrait for i8 {
    fn compute(&self) -> f64 {
        *self as f64
    }
}

impl TestTrait for u8 {
    fn compute(&self) -> f64 {
        *self as f64
    }
}

impl TestTrait for i16 {
    fn compute(&self) -> f64 {
        *self as f64
    }
}

impl TestTrait for u16 {
    fn compute(&self) -> f64 {
        *self as f64
    }
}

impl TestTrait for i128 {
    fn compute(&self) -> f64 {
        *self as f64
    }
}

impl TestTrait for u128 {
    fn compute(&self) -> f64 {
        *self as f64
    }
}
