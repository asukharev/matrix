use super::{Matrix};

//-------------------------------------------------------------------------------------------------
use std::ops::Add;

impl Add for Matrix<String> {
    type Output = Result<Matrix<String>, String>;
    fn add(self, m: Matrix<String>) -> Result<Matrix<String>, String> {
        if (self.rows != m.rows) || (self.columns != m.columns) {
            Err("Some error message1".to_string())
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
                    Err("Some error message2".to_string())
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
        // forward_ref_binop! { impl Mul, mul for $t, $t }
    )*)
}

add_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

//-------------------------------------------------------------------------------------------------
use std::ops::Mul;

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        impl Mul for Matrix<$t> {
            type Output = Result<Matrix<$t>, String>;
            #[inline]
            fn mul(self, m: Matrix<$t>) -> Result<Matrix<$t>, String> {
                if self.columns != m.rows {
                    Err("Some error message3".to_string())
                }
                else {
                    let mt = m.transpose();
                    let mut mr: Vec<$t> = Vec::new();
                    for va in self.values.chunks(self.columns) {
                        for vb in mt.values.chunks(m.rows) {
                            let s: Vec<_> = va.iter().zip(vb).collect();
                            let c: $t = s.iter()
                            .map(|&(a,b)| {
                                a*b
                            })
                            .fold(0 as $t, |acc, item| acc + item);
                            mr.push(c);
                        }
                    }
                    Ok( Matrix::from((self.rows, m.columns, mr)) )
                }
            }
        }
    )*)
}

mul_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

macro_rules! mul_by_num_impl {
    ($($t:ty)*) => ($(
        impl Mul<Matrix<$t>> for $t {
            type Output = Result<Matrix<$t>, String>;
            #[inline]
            fn mul(self, m: Matrix<$t>) -> Result<Matrix<$t>, String> {
                let d = m.values.iter().map(|x| x * self).collect();
                Ok(Matrix { rows: m.rows, columns: m.columns, values: d})
            }
        }
    )*)
}

mul_by_num_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

// #[test]
// fn t() {
//     let ma = vec!["a","b","c","d","e","f"];
//     let mb = vec!["w","x","y","z"];
//     let mut mr: Vec<(String,String)> = Vec::new();
//     let mb_rows = 2;
//     let ma_column = 1;
//
//     for va in ma.chunks(2) {
//         let mut m_row: Vec<Vec<String>> = Vec::new();
//         for column in 0..va.len() {
//             let vb: Vec<_> = mb.iter().enumerate().filter_map(|(idx, &x)|
//                 if idx % mb_rows == column { Some(x) } else { None }
//             ).collect();
//             let r: Vec<String> = va.iter().zip(&vb)
//                 .map(|(a,b)| {
//                     let mut rs = String::new();
//                     rs.push_str(a);
//                     rs.push_str("*");
//                     rs.push_str(b);
//                     rs.to_string()
//                 })
//                 .collect();
//             // let rr = r.join("+");
//             // println!("{:?}", rr);
//             m_row.push(r);
//         }
//         println!("{:?}", m_row);
//     }
// }
