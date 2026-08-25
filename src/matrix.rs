use std::{
    fmt,
    ops::{AddAssign, Mul},
    sync::mpsc,
    thread,
};

use crate::{
    Error::{self, MatrixShapeMismatch, WorkerGone},
    Result, Vector, dot_product,
};

const NUM_THREADS: usize = 4;

// 一个待算的任务
struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

// 算完的结果
struct MsgOutput<T> {
    idx: usize,
    value: Result<T>,
}

// 发给 worker 的完整消息：任务 + 一个「结果往哪送」的回邮信封
struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }
            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T: fmt::Display> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix(row={}, col={}, {self})", self.row, self.col)
    }
}

/// # Errors
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + AddAssign + Mul<Output = T> + Send + 'static,
{
    if a.col != b.row {
        return Err(MatrixShapeMismatch {
            a_col: a.col,
            b_row: b.row,
        });
    }
    let data_len = a.row * b.col;
    let mut data = vec![T::default(); data_len];

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let idx = msg.input.idx;
                    let value = dot_product(msg.input.row, msg.input.col);
                    msg.sender
                        .send(MsgOutput { idx, value })
                        .map_err(|_| WorkerGone)?;
                }
                Ok::<_, Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let mut receivers = Vec::new();
    for i in 0..a.row {
        let a_row = &a.data[i * a.col..(i + 1) * a.col];
        for j in 0..b.col {
            let b_col = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let row = Vector::new(a_row);
            let col = Vector::new(b_col);
            let idx = i * b.col + j;
            let (otx, orx) = oneshot::channel();
            let input = MsgInput { idx, row, col };
            senders[idx % NUM_THREADS]
                .send(Msg { input, sender: otx })
                .map_err(|_| WorkerGone)?;

            receivers.push(orx);
        }
    }

    for rx in receivers {
        let output = rx.recv().map_err(|_| WorkerGone)?;
        data[output.idx] = output.value?;
    }

    let matrix = Matrix {
        data,
        row: a.row,
        col: b.col,
    };
    Ok(matrix)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_multiply_success() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b)?;

        assert_eq!(format!("{c}"), "{22 28, 49 64}");
        assert_eq!(format!("{c:?}"), "Matrix(row=2, col=2, {22 28, 49 64})");
        Ok(())
    }
}
