use alloc::vec::Vec;
use fast_math::exp;
use stylus_sdk::prelude::*;

sol_storage! {
    pub struct MatrixOp{}
}

impl MatrixOp {
    pub fn transpose(ori: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let m = ori.len();
        let n = ori[0].len();
        let mut trans: Vec<Vec<f64>> = Vec::new();

        for i in 0..m {
            for j in 0..n {
                let ori_j_i = ori[j][i];
                trans[i][j] = ori_j_i;
            }
        }
        trans
    }

    pub fn softmax(z: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut v: Vec<Vec<f64>> = Vec::new();
        let mut sum: f32 = 0.0;
        for i in 0..z.len() {
            sum += exp(z[i][0] as f32);
        }
        for i in 0..z.len() {
            v[i][0] = (exp(z[i][0] as f32) / sum) as f64;
        }
        v
    }

    pub fn relu(z: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let len = z.len();
        let len2 = z[0].len();
        let mut v: Vec<Vec<f64>> = Vec::new();
        for i in 0..len {
            for j in 0..len2 {
                if z[i][j] > 0.0 {
                    v[i][j] = z[i][j];
                } else {
                    v[i][j] = 0.0;
                }
            }
        }
        v
    }

    pub fn dot_product(m1: &Vec<Vec<f64>>, m2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        assert_eq!(
            m1[0].len(),
            m2.len(),
            "Number of columns in first matrix must be equal to number of rows in second matrix"
        );
        let m = m1.len();
        let n = m2.len();
        let p = m2[0].len();
        let mut result = vec![vec![0.0; p]; m];

        for i in 0..m {
            for j in 0..p {
                for k in 0..n {
                    result[i][j] += m1[i][k] * m2[k][j];
                }
            }
        }

        result
    }
    pub fn sum(m1: &Vec<Vec<f64>>, m2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        assert_eq!(m1.len(), m2.len(), "Number of rows donot match");
        let n = m1.len();
        let m = m1[0].len();
        let mut result = vec![vec![0.0; m]; n];
        for i in 0..n {
            for k in 0..m {
                result[i][k] = m1[i][k] + m2[k][0];
            }
        }
        result
    }

    pub fn one_hot(y: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let m = y.len();
        let n = y[0].len();
        let size = m * n;
        let mut max_val = y[0][0];
        for i in 0..m {
            for j in 0..n {
                if y[i][j] > max_val {
                    max_val = y[i][j];
                }
            }
        }

        max_val += 1.0;
        let final_val = max_val as usize;
        let mut one_hot_y = vec![vec![0.0; final_val]; size]; // np.zeroes--done
        for row in 0..m {
            let col = y[row][0] as usize;
            one_hot_y[row][col] = 1.0;
        }
        one_hot_y
    }

    pub fn scalar_mul(mat: &Vec<Vec<f64>>, scalar: f64) -> Vec<Vec<f64>> {
        let mut result: Vec<Vec<f64>> = Vec::new();
        let m = mat.len();
        let n = mat[0].len();
        for i in 0..m {
            for j in 0..n {
                result[i][j] = mat[i][j] * scalar;
            }
        }
        result
    }
}