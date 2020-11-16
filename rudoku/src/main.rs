mod sudoku;

fn main() {
    // let su = [
    //     7, 0, 2, 0, 4, 8, 0, 0, 0,
    //     0, 8, 5, 0, 0, 0, 7, 0, 0,
    //     0, 0, 0, 0, 0, 2, 6, 0, 3,
    //     0, 0, 0, 6, 0, 1, 4, 0, 2,
    //     1, 0, 8, 2, 0, 0, 0, 0, 7,
    //     9, 0, 0, 7, 0, 0, 8, 0, 0,
    //     0, 0, 0, 0, 0, 3, 0, 4, 1,
    //     8, 3, 0, 0, 9, 0, 0, 7, 0,
    //     0, 9, 0, 4, 2, 0, 0, 3, 0
    // ];

    let su = [
        0,4,0,0,0,0,0,0,0,
        0,0,1,0,3,4,6,2,0,
        6,0,3,0,0,0,0,7,0,
        0,0,0,4,8,3,5,0,7,
        0,0,0,0,5,0,0,6,0,
        0,0,0,0,0,9,0,4,0,
        0,0,5,0,0,0,0,0,1,
        8,0,0,5,4,7,3,9,6,
        0,0,0,0,8,1,0,0,0
    ];

    let mut s = sudoku::Sudoku::new(&su);
    s.draw_to_display_q();
    
    s.solve_sudoku();

    println!();

    s.draw_to_display_s();
}





/*
fn solve_sudoku(sudoku: &[i32; 81]) -> [i32; 81]
{
    let mut temp = sudoku.clone();

}


fn get_my_square(sudoku: &[i32; 81], num: i32, option: char) -> Result<[i32; 9], &'static str>
{
    let mut ret = [0,0,0,0,0,0,0,0,0];
    match option
    {
        'H' =>
        {
            let h = get_my_first_addr(num, 'H').unwrap() as usize;
            for n in 0..ret.len()
            {
                ret[n] = sudoku[n + h];
            }
            Ok(ret)
        },
        'V' => {
            let v = get_my_first_addr(num, 'V').unwrap() as usize;
            for n in 0..ret.len()
            {
                ret[n] = sudoku[n * 9 + v];
            }
            Ok(ret)
        },
        'M' => {
            let z = get_my_first_addr(num, 'M').unwrap() as usize;
            for n in 0..3
            {
                for m in 0..3 {
                    ret[m + 3 * n] = sudoku[n * 9 + z + m];
                }
                
            }
            Ok(ret)
        },
        _ =>
        {
            Err("option err")
        }
    }
}

fn get_my_addr(num: i32) -> (i32, i32)
{

    (num%9, (num - num%9) / 9)
}

fn get_my_first_addr(num: i32, option: char) -> Result<i32, &'static str>
{
    match option
    {
        'H' =>
        {
            Ok(get_my_addr(num).1 * 9)
        },
        'V' =>
        {
            Ok(get_my_addr(num).0)
        },
        'M' =>
        {
            let mut x = num;
            while x % 3 != 0
            {
                x = x - 1;
            }
            while !((0 <= x && x <= 8) || (27 <= x && x <= 35) || (54 <= x && x <= 62))
            {
                x = x - 9;
            }
            Ok(
                x
            )
        },
        _ =>
        {
            Err("Oprion Err")
        }
    }
}
*/