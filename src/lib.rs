mod position;
use position::Position;
mod size;
use size::Size;

#[derive(Debug, Default)]
pub struct M<T> {
    rows: usize,
    columns: usize,
    values: Vec<T>
}

trait MyClone {
    fn clone(&self) -> Self;
}

impl MyClone for u16 {
    fn clone(&self) -> Self { *self }
}

impl MyClone for String {
    fn clone(&self) -> Self {
        let mut s = String::from("");
        s.push_str(self);
        s
    }
}

pub trait Convert<T> {
    fn into_matrix(&self, s: &Size) -> M<T>;
}

impl<T> Convert<T> for Vec<T>
    where T: Default + MyClone {
    fn into_matrix(&self, s: &Size) -> M<T> {
        let mut v: Vec<T> = Vec::new();
        let count = s.rows() * s.columns();
        let mut it = self.iter();
        let mut idx = 0;
        while idx < count {
            let value: T = match it.next() {
                Some(v) => v.clone(),
                None => Default::default()
            };
            v.push(value);
            idx += 1;
        }
        M { rows: s.rows(), columns: s.columns(), values: v }
    }
}

pub trait MatrixMath<T> {
    fn sum(&self, m: &M<T>) -> M<T>;
}

impl MatrixMath<u16> for M<u16> {
    fn sum(&self, m: &M<u16>) -> M<u16> {
        assert!((self.rows == m.rows) & (self.columns == m.columns));
        let zv = self.values.iter().zip(&m.values);
        let v: Vec<u16> = zv.map(|(a, b)| {
            a + b
        }).collect();
        M { rows: self.rows, columns: self.columns, values: v}
    }
}

impl MatrixMath<String> for M<String> {
    fn sum(&self, m: &M<String>) -> M<String> {
        assert!((self.rows == m.rows) & (self.columns == m.columns));
        let zv = self.values.iter().zip(&m.values);
        let v: Vec<String> = zv.map(|(a, b)| {
            let mut s = String::from("");
            s.push_str(a);
            s.push_str(b);
            s
        }).collect();
        M { rows: self.rows, columns: self.columns, values: v}
    }
}

pub trait Matrix<T> {
    fn set(&mut self, p: &Position, v: T);
    fn get(&self, p: &Position) -> T;
    fn transpose(&self) -> M<T>;
}

impl<T> Matrix<T> for M<T> where T: Default + MyClone {
    fn set(&mut self, p: &Position, v: T) {
        self.values[p.column() + (p.row() * self.columns)] = v.clone();
    }
    fn get(&self, p: &Position) -> T {
        let v = &self.values[p.column() + (p.row() * self.columns)];
        v.clone()
    }
    fn transpose(&self) -> M<T> {
        let mut v: Vec<T> = Vec::new();
        for column in 0..self.columns {
            for row in 0..self.rows {
                let value = self.get(&(row, column));
                v.push(value.clone());
            }
        }
        v.into_matrix(&(self.columns, self.rows))
    }
}

#[test]
fn it_works() {
    let v: Vec<u16> = vec![1,2,3];
    let mut m = v.into_matrix(&(3,3));
    assert_eq!(m.values, vec![1, 2, 3, 0, 0, 0, 0, 0, 0]);
    let p = &(1,1);
    m.set(p, 2);
    assert_eq!(m.values, vec![1, 2, 3, 0, 2, 0, 0, 0, 0]);
    let gv = m.get(p);
    assert_eq!(gv, 2);
    let mt: M<u16> = m.transpose();
    assert_eq!(mt.values, vec![1, 0, 0, 2, 2, 0, 3, 0, 0]);
    assert_eq!(m.sum(&mt).values, vec![2, 2, 3, 2, 4, 0, 3, 0, 0]);

    // Strings
    let v: Vec<String> = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let m: M<String> = v.into_matrix(&(3,3));
    assert_eq!(m.values, vec!["A", "B", "C", "", "", "", "", "", ""]);
    let mt: M<String> = m.transpose();
    assert_eq!(mt.values, vec!["A", "", "", "B", "", "", "C", "", ""]);
    assert_eq!(m.sum(&mt).values, vec!["AA", "B", "C", "B", "", "", "C", "", ""]);

    // assert!(false);
}
