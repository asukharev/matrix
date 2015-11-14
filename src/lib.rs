mod position;
use position::Position;
mod size;
use size::Size;
mod math;

#[derive(Debug, Default)]
pub struct Matrix<T> {
    pub rows: usize,
    pub columns: usize,
    pub values: Vec<T>
}

impl<T> Clone for Matrix<T> where T: Clone {
    fn clone(&self) -> Matrix<T> {
        let v: Vec<T> = self.values.iter().map(|x| x.clone()).collect();
        Matrix { rows: self.rows, columns: self.columns, values: v }
    }
}

impl<T> From<(usize, usize, Vec<T>)> for Matrix<T> where T: Default + Clone {
    fn from(v: (usize, usize, Vec<T>)) -> Matrix<T> {
        let (rows, columns, data) = v; // Decompose
        let mut dv: Vec<T> = Vec::new();
        let count = rows * columns;
        let mut it = data.iter();
        let mut idx = 0;
        while idx < count {
            let value: T = match it.next() {
                Some(v) => v.clone(),
                None => Default::default()
            };
            dv.push(value);
            idx += 1;
        }
        Matrix { rows: rows, columns: columns, values: dv }
    }
}

pub trait MatrixMod<T> {
    fn set(&mut self, p: &Position, v: T);
    fn get(&self, p: &Position) -> T;
    fn get_row(&self, row: usize) -> Vec<T>;
    fn transpose(&self) -> Matrix<T>;
}

impl<T> MatrixMod<T> for Matrix<T> where T: Default + Clone {
    fn set(&mut self, p: &Position, v: T) {
        self.values[p.column() + (p.row() * self.columns)] = v.clone();
    }
    fn get(&self, p: &Position) -> T {
        let v = &self.values[p.column() + (p.row() * self.columns)];
        v.clone()
    }
    fn get_row(&self, row: usize) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();
        for column in 0..self.columns {
            let value = self.get(&(row, column));
            v.push(value.clone());
        }
        v
    }
    fn transpose(&self) -> Matrix<T> {
        let mut v: Vec<T> = Vec::new();
        for column in 0..self.columns {
            for row in 0..self.rows {
                let value = self.get(&(row, column));
                v.push(value.clone());
            }
        }
        Matrix::from((self.columns, self.rows, v))
    }
}

#[test]
fn it_works() {
    {
        let v: Vec<u16> = vec![1,2,3];
        let mut m = Matrix::from((3, 3, v));
        assert_eq!(m.values, vec![1, 2, 3, 0, 0, 0, 0, 0, 0]);

        let p = &(1,1);
        m.set(p, 2);
        assert_eq!(m.values, vec![1, 2, 3, 0, 2, 0, 0, 0, 0]);

        let gv = m.get(p);
        assert_eq!(gv, 2);

        let mt: Matrix<u16> = m.transpose();
        assert_eq!(mt.values, vec![1, 0, 0, 2, 2, 0, 3, 0, 0]);

        let mtt: Matrix<u16> = mt.transpose();
        assert_eq!(m.values, mtt.values);

        match m.clone() + mt.clone() {
            Ok(r) => {
                println!("{:?}", r);
                assert_eq!(r.values, vec![2, 2, 3, 2, 4, 0, 3, 0, 0]);
            },
            Err(why) => println!("{:?}", why),
        }

        match m.clone() * mt.clone() {
            Ok(r) => {
                println!("{:?}", r);
                assert_eq!(r.values, vec![14, 4, 0, 4, 4, 0, 0, 0, 0]);
            },
            Err(why) => println!("{:?}", why),
        }

        match 2 as u16 * m.clone() {
            Ok(r) => {
                println!("{:?}", r);
                assert_eq!(r.values, vec![2, 4, 6, 0, 4, 0, 0, 0, 0]);
            },
            Err(why) => println!("{:?}", why),
        }
    }

    {
        let v0: Vec<u16> = vec![1,2,3,4,5,6];
        let m0 = Matrix::from((3, 2, v0));

        let v1: Vec<u16> = vec![1,1];
        let m1 = Matrix::from((2, 1, v1));

        match m0.clone() * m1.clone() {
            Ok(r) => {
                println!("{:?}", r);
                assert_eq!(r.values, vec![3, 7, 11]);
            },
            Err(why) => println!("{:?}", why),
        }
    }

    { // Strings
        let v: Vec<String> = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        // let m: M<String> = v.into_matrix(&(3,3));
        let m = Matrix::from((3, 3, v));
        assert_eq!(m.values, vec!["A", "B", "C", "", "", "", "", "", ""]);
        let mt: Matrix<String> = m.transpose();
        assert_eq!(mt.values, vec!["A", "", "", "B", "", "", "C", "", ""]);
        match m + mt {
            Ok(mmt) => {
                println!("{:?}", mmt);
                assert_eq!(mmt.values, vec!["AA", "B", "C", "B", "", "", "C", "", ""]);
            },
            Err(why) => println!("{:?}", why),
        }
    }

    // assert!(false);
}
