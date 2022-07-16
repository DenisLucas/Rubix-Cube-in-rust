use crate::lado::Lado;



pub struct Bloco {
    pub lados: [Lado;6]
}

impl Bloco
{
    pub fn new() -> Self
    {

        Self {
            lados: [
            Lado::new('W'), 
            Lado::new('Y'),Lado::new('R'),
            Lado::new('G'), Lado::new('B'), 
            Lado::new('O')
            ]
        }
    }
                    

    pub fn display_cube(&self)
    {
        print!("               ");

        self.lados[0].display_side(2,false);
        println!("");
        print!("               ");
        
        self.lados[0].display_side(1,false);
        println!("");
        print!("               ");

        self.lados[0].display_side(0,false);
        println!("");
        
        println!("");
        
        self.lados[1].display_side(0,true);
        self.lados[2].display_side(0,false);
        self.lados[3].display_side(0,false);
        self.lados[5].display_side(0,false);
        
        println!("");
        self.lados[1].display_side(1,true);
        self.lados[2].display_side(1,false);
        self.lados[3].display_side(1,false);
        self.lados[5].display_side(1,false);
        println!("");
        self.lados[1].display_side(2,true);
        self.lados[2].display_side(2,false);
        self.lados[3].display_side(2,false);
        self.lados[5].display_side(2,false);
        
        
        println!("");
        
        
        println!("");
        print!("               ");
        self.lados[4].display_side(0,false);
        println!("");
        
        print!("               ");
        self.lados[4].display_side(1,false);
        println!("");
        
        print!("               ");

        self.lados[4].display_side(2,false);
        println!("");
    }

    pub fn flip_cube(&mut self, face:i8, direction:&str,col:i8)
    {
        let front_flip_order_right: [i8; 4] = [2,3,5,1];
        let left_flip_order_upward: [i8; 4] = [1,0,3,4];
        let front_flip_order_upward: [i8; 4] = [2,0,5,4];
        #[allow(unused_assignments)]
        let mut flip_order: [i8;4] = [0;4]; 
        let mut rotate = false;
        let mut get_v = true;
        match direction 
        {
            "UP"|"DOWN" =>
            match face 
            {
                2|0|5|4 =>
                {
                    flip_order = front_flip_order_upward;
                    if direction == "DOWN"
                    {
                        flip_order.reverse();
                    }
                }
                1|3 =>
                {
                    flip_order = left_flip_order_upward;
                    if direction == "DOWN"
                    {
                        flip_order.reverse();
                    }
                    rotate = true;
                    
                }
                _ => todo!()
            },
            
            "LEFT"|"RIGHT" =>
            {
                match face
                {
                    2|3|0|5|1 =>
                    {
                        flip_order = front_flip_order_right;
                        if direction == "RIGHT"
                        {
                            flip_order.reverse();
                        }
                        get_v = false;
                    }
                    _ => todo!() 
                }
            }
            &_ => todo!()
        }
        let mut to_flip = self.lados[flip_order[0] as usize].get_side_col(col, 
            if direction == "UP"|| direction == "DOWN" 
                {true} 
            else 
                {false});

        
        for x in 0..flip_order.len()
        {
            if rotate
            {
                get_v = !get_v;
                
            }
            let indice = if x + 1 < flip_order.len() {x + 1} else {0};
            if flip_order[indice] == 0 {to_flip.reverse()};
            
            to_flip = self.lados[flip_order[indice] as usize].flip_lados(get_v,col, to_flip);
            
  
            
        }
        

        if col == 0 || col == 2
        {
            match direction
            {
                "UP" =>
                {
                    if face != 1 || face != 3 || face != 2
                    {
                        if col == 0
                        {
                            self.lados[1].rotate_lado(true);
                        }
                        else
                        {
                            self.lados[3].rotate_lado(true);
                        }
                    }
                    else
                    {
                        if col == 0
                        {
                            self.lados[2].rotate_lado(true);
                        }
                        else
                        {
                            self.lados[5].rotate_lado(true);
                        }
                    }
                }
                "DOWN" =>
                {
                    if face != 1 || face != 3 || face != 2
                    {
                        if col == 0
                        {
                            self.lados[1].rotate_lado(false);
                        }
                        else
                        {
                            self.lados[3].rotate_lado(false);

                        }
                    }
                    else
                    {
                        if col == 0
                        {
                            self.lados[2].rotate_lado(false);
                        }
                        else
                        {
                            self.lados[5].rotate_lado(false);
                        }
                    }
                    
                }
                "LEFT" =>
                {
                    if face != 0 || face != 4
                    {
                        if col == 0
                        {
                            self.lados[0].rotate_lado(false);
                        }
                        else
                        {
                            self.lados[4].rotate_lado(false);

                        }
                        
                    }
                    else
                    {
                        if col == 0
                        {
                            self.lados[1].rotate_lado(false);
                        }
                        else
                        {
                            self.lados[3].rotate_lado(false);
                        }
                    }
                }

                "RIGHT" =>
                {
                    if face != 0 || face != 4
                    {
                        if col == 0
                        {
                            self.lados[0].rotate_lado(true);
                        }
                        else
                        {
                            self.lados[4].rotate_lado(true);

                        }
                    }
                    else
                    {
                        if col == 0
                        {
                            self.lados[1].rotate_lado(true);
                        }
                        else
                        {
                            self.lados[3].rotate_lado(true);
                        }
                    }
                }
                &_ => todo!()
            }
        }
    }
}
