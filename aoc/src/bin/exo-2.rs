use std::ops::AddAssign;

// fn sum<T>(arr: &[T]) -> T
// where
//     T: AddAssign + Copy + Default,
// {
//     let mut sum: T = T::default();
//     for x in arr {
//         sum += *x;
//     }
//     sum
// }

fn sum<T, U>(items: T) -> U
where
    U: AddAssign + Copy + Default,
    T: AsRef<[U]>,
{
    let mut result: U = U::default();
    for i in items.as_ref() {
        result += *i
    }
    result
}

fn main() {
    let _a: i32 = sum(&[0, 1, 2]); // 3
    let _b: i64 = sum(&[]); // 0
    let _c: i16 = sum(&[6]); // 6
    let _d: f64 = sum(&[1.5, 5.5, 1.7]); // 8.7
    let _e: i32 = sum([7, 3, 5]); // 15
    let _f: f32 = sum(vec![4.6, 2.6, 9.7]); // 16.9
}
