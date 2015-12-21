use super::{Matrix};

//-------------------------------------------------------------------------------------------------
use std::ops::Add;

impl Add for Matrix<String> {
    type Output = Result<Matrix<String>, String>;
    fn add(self, m: Matrix<String>) -> Result<Matrix<String>, String> {
        if (self.rows != m.rows) || (self.columns != m.columns) {
            Err("Dissimilar multidimensional matrix".to_string())
        }
        else {
            let zv = self.values.iter().zip(&m.values);
            let v: Vec<String> = zv.map(|(a, b)| {
                let mut s = String::from("");
                s.push_str(a);
                s.push_str(b);
                s
            }).collect();
            Ok(Matrix { rows: self.rows, columns: self.columns, values: v})
        }
    }
}

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl Add for Matrix<$t> {
            type Output = Result<Matrix<$t>, String>;
            #[inline]
            fn add(self, m: Matrix<$t>) -> Result<Matrix<$t>, String> {
                if (self.rows != m.rows) && (self.columns != m.columns) {
                    Err("Dissimilar multidimensional matrix".to_string())
                }
                else {
                    let zv = self.values.iter().zip(&m.values);
                    let v: Vec<$t> = zv.map(|(a, b)| {
                        a + b
                    }).collect();
                    Ok(Matrix { rows: self.rows, columns: self.columns, values: v})
                }
            }
        }
    )*)
}

add_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

//-------------------------------------------------------------------------------------------------
use std::ops::Mul;

impl<T> Mul for Matrix<T> where T: Default + Clone + Add<Output=T> + Mul<Output=T> {
    type Output = Result<Matrix<T>, String>;
    fn mul(self, m: Matrix<T>) -> Result<Matrix<T>, String> {
        if self.columns != m.rows {
            Err("Dissimilar multidimensional matrix".to_string())
        }
        else {
            let mt = m.transpose();
            let mut mr: Vec<T> = Vec::new();
            for va in self.values.chunks(self.columns) {
                for vb in mt.values.chunks(m.rows) {
                    let s: Vec<_> = va.iter().zip(vb).collect();
                    let c: T = s.iter()
                        .map(|&(a,b)| {
                            a.clone() * b.clone()
                        })
                        .fold(T::default(), |acc, item| acc + item);
                    mr.push(c);
                }
            }
            Ok( Matrix::from((self.rows, m.columns, mr)) )
        }
    }
}

macro_rules! mul_by_num_impl {
    ($($t:ty)*) => ($(
        impl Mul<Matrix<$t>> for $t {
            type Output = Result<Matrix<$t>, String>;
            #[inline]
            fn mul(self, m: Matrix<$t>) -> Result<Matrix<$t>, String> {
                let d = m.values.iter().map(|x| x * self).collect();
                let m = Matrix::from((m.rows, m.columns, d));
                Ok(m.transpose())
            }
        }
    )*)
}

mul_by_num_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
