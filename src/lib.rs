mod position;
use position::Position;
mod size;
use size::Size;

#[derive(Debug)]
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
    fn sum(&self, m: &M) -> M;
    fn transpose(&self) -> M;
}

impl Matrix for M {
    fn set(&mut self, p: &Position, v: u8) {
        self.values[p.column() + (p.row() * self.columns)] = v;
    }
    fn get(&self, p: &Position) -> u8 {
        // println!("{:?} + ({:?} * {:?}) = {:?}",
        //     p.column(),
        //     p.row(),
        //     self.rows,
        //     p.column() + ( p.row() * self.columns )
        // );
        self.values[p.column() + (p.row() * self.columns)]
    }
    fn sum(&self, m: &M) -> M {
        assert!((self.rows == m.rows) & (self.columns == m.columns));
        let zv = self.values.iter().zip(&m.values);
        let v: Vec<u8> = zv.map(|(a, b)| {
            a + b
        }).collect();
        M { rows: self.rows, columns: self.columns, values: v}
    }
    fn transpose(&self) -> M {
        println!("{:?}", self);
        let count = self.rows * self.columns;
        let mut rm: M =
            (0..count)
            .map(|_| 0)
            .collect::<Vec<u8>>()
            .into_matrix(&(self.columns, self.rows));
        for row in 0..self.rows {
            for column in 0..self.columns {
                let value = self.get(&(row, column));
                println!("{:?} => {:?} => {:?}\n", (row, column), value, (column, row));
                rm.set(&(column, row), value);
            }
        }
        println!("{:?}", rm);
        rm
    }
}

#[test]
fn it_works() {
    let mut m: M = vec![1,2,3].into_matrix(&(2,3));
    assert_eq!(m.values, vec![1,2,3,0,0,0]);

    let p = (1,1);
    m.set(&p, 2);
    assert_eq!(m.values, vec![1,2,3,0,2,0]);
    let gv = m.get(&p);
    assert_eq!(gv, 2);

    let mt: M = m.transpose();
    assert_eq!(mt.values, vec![1,3,0,2,0,0]);

    // assert_eq!(m.sum(&mt).values, vec![2,5,5,2]);
}
