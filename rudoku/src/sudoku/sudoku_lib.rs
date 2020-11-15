impl Sudoku {
    // static function

    // create sudoku
    pub fn new(su: &[i32; 81]) -> Sudoku
    {
        Sudoku{
            data: (*su).clone(),
            solve: [0; 81],
            marking: Vec::new()
        }
    }

    // get sudoku x,y location
    fn get_my_addr(num: i32) -> (i32, i32)
    {
        (num%9, (num - num%9) / 9)
    }

    // get sudoku's square's first address
    // option willb define the shape of the rectangle
    // option - V - Vertical rectangle
    //        - H - horizon  rectangle
    fn get_my_first_addr(num: i32, option: char) -> i32
    {
        match option
        {
            'H' =>
            {
                (Sudoku::get_my_addr(num).1 * 9)
            },
            'V' =>
            {
                (Sudoku::get_my_addr(num).0)
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
                x
            },
            _ =>
            {
                panic!("argument err");
            }

        }
    }

    // dyn function

    // draw sudoku data to std display
    pub fn draw_to_display_q(&self)
    {
        let mut i = 1;
        while i < 82
        {
            if self.data[i - 1] == 0
            {
                print!("- ");
            }
            else
            {
                print!("{} ", self.data[i - 1]);
            }
            if i % 27 == 0
            {  
                println!("\n---------------------");
            }
            else if i % 9 == 0
            {   
                println!();
            }
            else if i % 3 == 0
            {
                print!("| ");
            }
            i = i + 1;
        }
    }
    
   // draw sudoku data to std display
    pub fn draw_to_display_s(&self)
    {
        let mut i = 1;
        while i < 82
        {
            if self.solve[i - 1] == 0
            {
                print!("- ");
            }
            else
            {
                print!("{} ", self.solve[i - 1]);
            }
            if i % 27 == 0
            {  
                println!("\n---------------------");
            }
            else if i % 9 == 0
            {   
                println!();
            }
            else if i % 3 == 0
            {
                print!("| ");
            }
            i = i + 1;
        }
    }
    
    fn get_my_square(&self, num: i32, option: char) -> [i32; 9]
    {
        let mut ret = [0,0,0,0,0,0,0,0,0];
        match option
        {
            'H' =>
            {
                let h = Sudoku::get_my_first_addr(num, 'H') as usize;
                for n in 0..ret.len()
                {
                    ret[n] = self.data[n + h];
                }
                ret
            },
            'V' => {
                let v = Sudoku::get_my_first_addr(num, 'V') as usize;
                for n in 0..ret.len()
                {
                    ret[n] = self.data[n * 9 + v];
                }
                ret
            },
            'M' => {
                let z = Sudoku::get_my_first_addr(num, 'M') as usize;
                for n in 0..3
                {
                    for m in 0..3 {
                        ret[m + 3 * n] = self.data[n * 9 + z + m];
                    }
                    
                }
                ret
            },
            _ =>
            {
                panic!("option err");
            }
        }
    }
}