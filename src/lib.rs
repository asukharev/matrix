mod math;
use std::fmt;

#[derive(Default)]
pub struct Matrix<T> {
    pub rows: usize,
    pub columns: usize,
    pub values: Vec<T>,
}

impl<T> Matrix<T>
    where T: Default + Clone
{
    pub fn iter(&self) -> MatrixIter<T> {
        MatrixIter {
            matrix: self,
            index: 0,
        }
    }
    pub fn set(&mut self, row: usize, column: usize, v: T) {
        self.values[column + (row * self.columns)] = v;
    }
    pub fn get(&self, row: usize, column: usize) -> &T {
        &self.values[column + (row * self.columns)]
    }
    pub fn get_row(&self, row: usize) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();
        for column in 0..self.columns {
            let value = self.get(row, column);
            v.push(value.clone());
        }
        v
    }
    pub fn get_column(&self, column: usize) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();
        for row in 0..self.rows {
            let value = self.get(row, column);
            v.push(value.clone());
        }
        v
    }
    pub fn transpose(&self) -> Matrix<T> {
        let new_values : Vec<T> = (0..self.values.len())
            .enumerate()
            .map(|(idx, _)| {
                let r = idx % self.rows;
                let c = idx / self.rows;
                let t_idx = c + (r * self.columns);
                self.values[t_idx].clone()
            })    
            .collect();
        Matrix::from((self.rows, self.columns, new_values))
    }
}

pub struct MatrixIter<'a, T: 'a> {
    matrix: &'a Matrix<T>,
    index: usize,
}

impl<'a, T> Iterator for MatrixIter<'a, T>
    where T: Default + Clone
{
    type Item = (&'a T, (usize, usize));
    fn next(&mut self) -> Option<(&'a T, (usize, usize))> {
        if self.index < self.matrix.rows * self.matrix.columns {
            let row = self.index / self.matrix.columns;
            let column = self.index % self.matrix.columns;
            let position = (row, column);
            let v = self.matrix.get(row, column);
            self.index += 1;
            Some((v, position))
        } else {
            None
        }
    }
}

impl<T> fmt::Debug for Matrix<T>
    where T: Default + Clone + ToString
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: String = String::new();
        for row in 0..self.rows {
            output.push_str("\n[");
            for column in 0..self.columns {
                let value: &T = self.get(row, column);
                output.push_str(&value.to_string());
                if column < self.columns - 1 {
                    output.push(',');
                    output.push('\t');
                }
            }
            output.push_str("]");
        }
        write!(f, "{}", output)
    }
}

impl<T> Clone for Matrix<T>
    where T: Clone
{
    fn clone(&self) -> Matrix<T> {
        let v: Vec<T> = self.values.iter().map(|x| x.clone()).collect();
        Matrix {
            rows: self.rows,
            columns: self.columns,
            values: v,
        }
    }
}

impl<T> From<(usize, usize, Vec<T>)> for Matrix<T>
    where T: Default + Clone
{
    fn from(v: (usize, usize, Vec<T>)) -> Matrix<T> {
        let (columns, rows, data) = v;
        let m_data: Vec<T> = (0..(rows * columns))
            .enumerate()
            .map(|(idx, _)| if idx < data.len() {
                data[idx].clone()
            } else {
                T::default()
            })
            .collect();
        
        Matrix {
            rows: rows,
            columns: columns,
            values: m_data,
        }
    }
}
