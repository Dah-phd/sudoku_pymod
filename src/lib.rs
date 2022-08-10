#[macro_use]
extern crate cpython;

use cpython::{PyList, PyResult, Python, PythonObject, ToPyObject};

fn make_key(py: Python, base: Vec<Vec<i32>>) -> PyResult<PyList> {
    let mut key = base;
    solve(&mut key);
    let mut py_key = Vec::new();
    for row in key {
        for val in row {
            py_key.push(PythonObject::into_object(ToPyObject::to_py_object(
                &val, py,
            )))
        }
    }
    Ok(PyList::new(py, &py_key))
}
// backstrack algorithm
fn solve(mut board: &mut Vec<Vec<i32>>) -> bool {
    let location = find_location(&board);
    if location == None {
        return true;
    }
    let location = location.unwrap();
    for i in 1..10 {
        if works(&board, &location, &i) {
            board[location.0 as usize][location.1 as usize] = i;
            if solve(&mut board) {
                return true;
            }
        }
    }
    board[location.0 as usize][location.1 as usize] = 0 as i32;
    false
}

fn get_range_by_location(location:usize) -> std::ops::Range<usize> {
    match location {
        loc if loc < 3 => 0..3,
        loc if loc < 6 => 3..6,
        _ => 6..9
    }
}

// check if works
fn works(board: &Vec<Vec<i32>>, location: &(usize, usize), value: &i32) -> bool {
    if board[location.0].contains(value) {
        return false;
    };
    for row in board {
        if row[location.1] == *value {
            return false;
        };
    };
    for rr in get_range_by_location(location.0) {
        for rc in get_range_by_location(location.1) {
            if *value == board[rr][*rc] {
                return false;
            }
        }
    }
    true
}
// points to zero
fn find_location(board: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
    for (row, vec) in board.iter().enumerate() {
        for (cell, i) in vec.iter().enumerate() {
            if *i == 0 {
                return Some((row, cell));
            }
        }
    }
    return None;
}

py_module_initializer!(solver, initsolver, PyInit_solver, |py, m| {
    m.add(py, "__doc__", "This is sudoku solver with good speed.")?;
    m.add(py, "make_key", py_fn!(py, make_key(base: Vec<Vec<i32>>)))?;
    Ok(())
});
