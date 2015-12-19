extern crate matrix;
use matrix::Matrix;

#[test]
fn check_new() {
    let v: Vec<i32> = vec![1, 2, 3];
    let m = Matrix::from((3, 3, v));
    assert_eq!(m.values, vec![1, 0, 0, 2, 0, 0, 3, 0, 0]);
}

#[test]
fn check_set_value() {
    let v: Vec<i32> = vec![1, 2, 3, 0, 0, 0, 0, 0, 0];
    let mut m = Matrix::from((3, 3, v));
    // Set element of row and column
    m.set(1, 1, 2);
    assert_eq!(m.values, vec![1, 0, 0, 2, 2, 0, 3, 0, 0]);
}

#[test]
fn check_get_value() {
    let v: Vec<i32> = vec![1, 2, 3, 0, 0, 0, 0, 0, 0];
    let m = Matrix::from((3, 3, v.clone()));
    for column in 0..m.columns {
        for row in 0..m.rows {
            let mv = m.get(row, column);
            let idx = row + (column * m.columns);
            assert_eq!(*mv, v[idx]);
        }
    }
}

//
// ⎡1, 0, 0⎤'  ⎡1, 2, 3⎤
// ⎢2, 2, 2⎥ = ⎢0, 2, 0⎥
// ⎣3, 0, 0⎦   ⎣0, 2, 0⎦
//
#[test]
fn check_transpose() {
    let v: Vec<i32> = vec![1, 2, 3, 0, 2, 0, 0, 2, 0];
    let m = Matrix::from((3, 3, v));

    let mt: Matrix<i32> = m.transpose();
    assert_eq!(mt.values, vec![1, 2, 3, 0, 2, 0, 0, 2, 0]);

    let mtt: Matrix<i32> = mt.transpose();
    assert_eq!(m.values, mtt.values);
}

//
// ⎡1, 0, 0⎤       ⎡2, 0, 0⎤
// ⎢2, 0, 4⎥ * 2 = ⎢4, 0, 8⎥
// ⎣3, 0, 0⎦       ⎣6, 0, 0⎦
//
#[test]
fn check_multiplication_by_number() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let m = Matrix::from((3, 3, v));

    match 2 as i32 * m.clone() {
        Ok(r) => {
            assert_eq!(r.values, vec![2, 4, 6, 8, 10, 12, 14, 16, 18]);
        },
        Err(why) => println!("{:?}", why),
    }
}

//
// ⎡1, 4, 7⎤   ⎡1, 2, 3⎤   ⎡66,  78,  90⎤
// ⎢2, 5, 8⎥ * ⎢4, 5, 6⎥ = ⎢78,  93, 108⎥
// ⎣3, 6, 9⎦   ⎣7, 8, 9⎦   ⎣90, 108, 126⎦
//
#[test]
fn check_multiplication() {
    {
        let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let m = Matrix::from((3, 3, v));
        let mt: Matrix<i32> = m.transpose();

        match m * mt {
            Ok(r) => {
                assert_eq!(r.values, vec![66, 78, 90, 78, 93, 108, 90, 108, 126]);
            },
            Err(why) => println!("{:?}", why),
        }
    }
}

//
// ⎡1, 4⎤   ⎡1⎤   ⎡5⎤
// ⎢2, 5⎥ * ⎢ ⎥ = ⎢7⎥
// ⎣3, 6⎦   ⎣1⎦   ⎣9⎦
//
#[test]
fn check_multiplication_by_vector() {
    let v0: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let m0 = Matrix::from((3, 2, v0));

    let v1: Vec<i32> = vec![1, 1];
    let m1 = Matrix::from((2, 1, v1));

    match m0 * m1 {
        Ok(r) => {
            assert_eq!(r.values, vec![5, 7, 9]);
        },
        Err(why) => println!("{:?}", why),
    }
}

//
// ⎡1, 4, 7⎤   ⎡1, 2, 3⎤   ⎡ 2,  6, 10⎤
// ⎢2, 5, 8⎥ + ⎢4, 5, 6⎥ = ⎢ 6, 10, 14⎥
// ⎣3, 6, 9⎦   ⎣7, 8, 9⎦   ⎣10, 14, 18⎦
//
#[test]
fn check_sum() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let m = Matrix::from((3, 3, v));
    let mt: Matrix<i32> = m.transpose();

    match m.clone() + mt.clone() {
        Ok(r) => {
            assert_eq!(r.values, vec![2, 6, 10, 6, 10, 14, 10, 14, 18]);
        },
        Err(why) => println!("{:?}", why),
    }
}

//-------------------------------------------------------------------------------------------------
// #[test]
// fn it_works() {
//     { // Strings
//         let v: Vec<String> = vec!["A".to_string(), "B".to_string(), "C".to_string()];
//         // let m: M<String> = v.into_matrix(&(3,3));
//         let m = Matrix::from((3, 3, v));
//         assert_eq!(m.values, vec!["A", "B", "C", "", "", "", "", "", ""]);
//         let mt: Matrix<String> = m.transpose();
//         assert_eq!(mt.values, vec!["A", "", "", "B", "", "", "C", "", ""]);
//         match m + mt {
//             Ok(mmt) => {
//                 println!("{:?}", mmt);
//                 assert_eq!(mmt.values, vec!["AA", "B", "C", "B", "", "", "C", "", ""]);
//             },
//             Err(why) => println!("{:?}", why),
//         }
//     }
// }
