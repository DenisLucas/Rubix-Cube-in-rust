pub struct Lado{
    pub lado: [[char; 3];3]
}

impl Lado
{
    pub fn new(color:char) -> Self
    {
        let lado:[[char; 3];3] =
        [
            [color,color,color],
            [color,color,color],
            [color,color,color]
        ];
        Self {
            lado: lado
        }
    }
    

    pub fn display_side(&self,col:i8, invert:bool)
    {
        if invert
        {
            print!("[ {} , {} , {} ]  ",
        self.lado[col as usize][2],
        self.lado[col as usize][1],
        self.lado[col as usize][0]);
        }
        else
        {
            print!("[ {} , {} , {} ]  ",
            self.lado[col as usize][0],
            self.lado[col as usize][1],
            self.lado[col as usize][2]);
        }
        
    }

    pub fn get_side_col(&self, col:i8, get_vertical:bool) -> [char;3]
    {
        return 
        if get_vertical 
        {
            [
            self.lado[0][col as usize],
            self.lado[1][col as usize],
            self.lado[2][col as usize]
            ]
        }
        else 
        {
            [
            self.lado[col as usize][0],
            self.lado[col as usize][1],
            self.lado[col as usize][2]
            ]
        }
    }

    pub fn flip_lados(&mut self, vertical:bool, col:i8, to_change:[char;3]) -> [char;3]
    {
        
        let mut original: [char;3] = ['t','t','t'];
        for x in 0..3
        {
            if vertical
            {
                original[x] = self.lado[x][col as usize]; 
                self.lado[x][col as usize] = to_change[x];
            }
            else
            {
                original[x] = self.lado[col as usize][x]; 
                self.lado[col as usize][x] = to_change[x];
            }
            
        }
        

        return original
    }


    pub fn rotate_lado(&mut self, move_up:bool)
    {
        let copy = self.lado;
        if move_up
        {
            self.lado[0] = [copy[0][2],copy[1][2],copy[2][2]];
            self.lado[1] = [copy[0][1],copy[1][1],copy[2][1]];
            self.lado[2] = [copy[0][0],copy[1][0],copy[2][0]];
        }
        else
        {
            self.lado[2] =[copy[2][2],copy[1][2],copy[0][2]];
            self.lado[1] = [copy[2][1],copy[1][1],copy[0][1]];
            self.lado[0] = [copy[2][0],copy[1][0],copy[0][0]];
        }
    }
}
