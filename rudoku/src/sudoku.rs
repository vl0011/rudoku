mod sudoku_lib;

pub struct Sudoku {
    data: [i32; 81],
    solve: [i32; 81],
    marking: Vec<Mark>
}



struct Mark {
    num: i32,
    mark: Vec<i32>
}