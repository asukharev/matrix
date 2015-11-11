use super::Matrix;

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
                    let mut mr: Vec<$t> = Vec::new();
                    for va in self.values.chunks(self.columns) {
                        let mut m_row: Vec<$t> = Vec::new();
                        for column in 0..va.len() {
                            let vb: Vec<$t> = m.values.iter().enumerate().filter_map(|(idx, &x)|
                                if idx % m.rows == column { Some(x) } else { None }
                            ).collect();
                            let r: Vec<$t> = va.iter().zip(&vb)
                                .map(|(a,b)| {
                                    a * b
                                })
                                .collect();
                            let sum = r.iter().fold(0 as $t, |acc, &item| acc + item);
                            m_row.push(sum);
                        }
                        for v in m_row {
                            mr.push(v);
                        }
                    }
                    Ok(Matrix { rows: self.rows, columns: self.columns, values: mr})
                }
            }
        }
    )*)
}

mul_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

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
