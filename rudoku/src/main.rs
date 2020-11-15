fn main() {
    let su = [
        0, 1, 2, 3, 4, 5, 6, 7, 8 ,
        9, 10,11,12,13,14,15,16,17,
        18,19,20,21,22,23,24,25,26,
        27,28,29,30,31,32,33,34,35,
        36,37,38,39,40,41,42,43,44,
        45,46,47,48,49,50,51,52,53,
        54,55,56,57,58,59,60,61,62,
        63,64,65,66,67,68,69,70,71,
        72,73,74,75,76,77,78,79,80
    ];
    let n =33;
    println!("{:?}", get_my_addr(n));
    println!("{}", get_my_first_addr(n, 'H').unwrap());
    println!("{}", get_my_first_addr(n, 'V').unwrap());
    println!("{}", get_my_first_addr(n, 'M').unwrap());
    println!("{:?}", get_my_square(&su, n, 'H'));
    println!("{:?}", get_my_square(&su, n, 'V'));
    println!("{:?}", get_my_square(&su, n, 'M'));
}
/*
fn solve_sudoku(sudoku: &[i32; 81]) -> [i32; 81]
{
    let mut temp = sudoku.clone();

}

*/
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