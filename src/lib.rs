mod position;
use position::Position;
mod size;
use size::Size;

#[derive(Debug)]
pub struct M {
    rows: usize,
    columns: usize,
    values: Vec<u16>
}

pub trait Convert {
    fn into_matrix(self, s: &Size) -> M;
}
impl Convert for Vec<u16>{
    fn into_matrix(self, s: &Size) -> M {
        let mut v: Vec<u16> = Vec::new();
        let count = s.rows() * s.columns();
        let mut it = self.iter();
        let mut idx = 0;
        while idx < count {
            let value: u16 = match it.next() {
                Some(v) => v.clone(),
                None => 0
            };
            v.push(value);
            idx += 1;
        }
        M { rows: s.rows(), columns: s.columns(), values: v }
    }
}

pub trait Matrix {
    fn set(&mut self, p: &Position, v: u16);
    fn get(&self, p: &Position) -> u16;
    fn sum(&self, m: &M) -> M;
    fn transpose(&self) -> M;
}

impl Matrix for M {
    fn set(&mut self, p: &Position, v: u16) {
        self.values[p.column() + (p.row() * self.columns)] = v;
    }
    fn get(&self, p: &Position) -> u16 {
        self.values[p.column() + (p.row() * self.columns)]
    }
    fn sum(&self, m: &M) -> M {
        assert!((self.rows == m.rows) & (self.columns == m.columns));
        let zv = self.values.iter().zip(&m.values);
        let v: Vec<u16> = zv.map(|(a, b)| {
            a + b
        }).collect();
        M { rows: self.rows, columns: self.columns, values: v}
    }
    fn transpose(&self) -> M {
        let mut v: Vec<u16> = Vec::new();
        for column in 0..self.columns {
            for row in 0..self.rows {
                let value = self.get(&(row, column));
                v.push(value);
            }
        }
        v.into_matrix(&(self.columns, self.rows))
    }
}

#[test]
fn it_works() {
    let mut m: M = vec![1,2,3].into_matrix(&(3,3));
    assert_eq!(m.values, vec![1, 2, 3, 0, 0, 0, 0, 0, 0]);

    let p = (1,1);
    m.set(&p, 2);
    assert_eq!(m.values, vec![1, 2, 3, 0, 2, 0, 0, 0, 0]);
    let gv = m.get(&p);
    assert_eq!(gv, 2);

    let mt: M = m.transpose();
    assert_eq!(mt.values, vec![1, 0, 0, 2, 2, 0, 3, 0, 0]);

    assert_eq!(m.sum(&mt).values, vec![2, 2, 3, 2, 4, 0, 3, 0, 0]);
}
