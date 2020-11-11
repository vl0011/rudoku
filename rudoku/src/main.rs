fn main() {
    println!("{:?}", get_my_addr(80));

}
/*
fn solve_sudoku(sudoku: &[i32; 81]) -> [i32; 81]
{
    let mut temp = sudoku.clone();

}

fn get_my_square(sudoku: &[i32; 81], num: i32, option: char) -> Result<[i32; 9], &'static str>
{
    let mut n = 0;
    // let mut m = 0;
    let mut s: [i32; 9];
    if !(num > 0 && num < 82) {
        Err("num err");
    }
    match option {
        'H' => {
            if num % 9 != 0 {
                
            }
            for i in 0..9
            {
                
                s[i] = sudoku[n + i];
                n = n + 9;
            }
            Ok(s)
        },
        'V' => {
            Ok()
        },
        'M' => {
            Ok()
        },
        _ => {
            Err("Option Err")
        }
    }
}*/

fn get_my_addr(num: i32) -> (i32, i32)
{
    let mut h = 0;
    while (h + 1) * 9 < num 
    {
        h = h + 1;
    }
    
    (num - (h * 9), h)
}

fn get_my_first_addr(num: i32, option: char) -> Result<i32, &'static str>
{
    match option
    {
        'H' =>
        {
            Ok(get_my_addr(num).1)
        },
        'V' =>
        {
            Ok(get_my_addr(num).0 * 9)
        },
        'M' =>
        {
            Ok()
        },
        _ =>
        {
            Err("Oprion Err")
        }
    }
}
/*
fn get_horizn()
{

}

fn get_virtical()
{

}

fn get_33()
{

}
*/