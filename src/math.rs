use super::M;

//-------------------------------------------------------------------------------------------------
use std::ops::Add;

impl Add for M<String> {
    type Output = Result<M<String>, String>;
    fn add(self, m: M<String>) -> Result<M<String>, String> {
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
            Ok(M { rows: self.rows, columns: self.columns, values: v})
        }
    }
}

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl Add for M<$t> {
            type Output = Result<M<$t>, String>;
            #[inline]
            fn add(self, m: M<$t>) -> Result<M<$t>, String> {
                if (self.rows != m.rows) && (self.columns != m.columns) {
                    Err("Some error message2".to_string())
                }
                else {
                    let zv = self.values.iter().zip(&m.values);
                    let v: Vec<$t> = zv.map(|(a, b)| {
                        a + b
                    }).collect();
                    Ok(M { rows: self.rows, columns: self.columns, values: v})
                }
            }
        }
        // forward_ref_binop! { impl Mul, mul for $t, $t }
    )*)
}

add_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

//-------------------------------------------------------------------------------------------------
use std::ops::Mul;

impl Mul for M<u16> {
    type Output = Result<M<u16>, String>;
    fn mul(self, m: M<u16>) -> Result<M<u16>, String> {
        if (self.rows != m.rows) && (self.columns != m.columns) {
            // assert!((self.rows == m.rows) & (self.columns == m.columns));
            Err("Some error message2".to_string())
        }
        else {
            let zv = self.values.iter().zip(&m.values);
            let v: Vec<u16> = zv.map(|(a, b)| {
                a + b
            }).collect();
            Ok(M { rows: self.rows, columns: self.columns, values: v})
        }
    }
}
