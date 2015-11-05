mod position;
use position::Position;
mod size;
use size::Size;

pub struct M {
    rows: usize,
    columns: usize,
    values: Vec<u8>
}

pub trait Convert {
    fn into_matrix(self, s: &Size) -> M;
}
impl Convert for Vec<u8>{
    fn into_matrix(self, s: &Size) -> M {
        let mut v: Vec<u8> = Vec::new();
        let count = s.rows() * s.columns();
        let mut it = self.iter();
        let mut idx = 0;
        while idx < count {
            let value: u8 = match it.next() {
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
    fn set(&mut self, p: &Position, v: u8);
    fn get(&self, p: &Position) -> u8;
    fn transpose(&self) -> M;
}

impl Matrix for M {
    fn set(&mut self, p: &Position, v: u8) {
        self.values[p.column() + (p.row() * self.rows)] = v;
    }
    fn get(&self, p: &Position) -> u8 {
        let row = p.row();
        let col = p.column();
        let v = self.values[col + (row * self.rows)];
        v
    }
    fn transpose(&self) -> M {
        let mut v: Vec<u8> = Vec::new();
        for column in 0..self.columns {
            for row in 0..self.rows {
                let p = (row, column);
                v.push(self.get(&p));
            }
        }
        M { rows: self.columns, columns: self.rows, values: v}
    }
}

#[test]
fn it_works() {
    let mut m: M = vec![1,2,3].into_matrix(&(2,2));
    let mt: M = m.transpose();

    assert_eq!(m.values, vec![1,2,3,0]);

    let p = (1,1);

    m.set(&p, 2);
    let gv = m.get(&p);

    assert_eq!(m.values, vec![1,2,3,2]);
    assert_eq!(gv, 2);
    assert_eq!(mt.values, vec![1,3,2,0]);
}
