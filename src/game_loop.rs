use std::io;
//use rand::prelude::*;

use crate::bloco::Bloco;

pub fn start_game()
{

    let cubo_magico = Bloco::new();
    println!("Start Game?:");
    let input = read_input();
    let converted_input = input.trim().to_lowercase(); 
    if converted_input == "start"
    {
        game_loop(cubo_magico);
    }
 
}


fn game_loop(mut cubo_magico: Bloco)
{
    let possible_inputs: [&str; 3 ] = ["QUIT", "FLIP","SCRAMBLE"];
    cubo_magico.display_cube();
       
    loop
    {
        let mut error: &str = "n";
        println!("FACES: TOP,LEFT,FRONT,RIGHT,DOWN,BACK");
        println!("DIRECTIONS: UP,DOWN,LEFT,RIGHT");
        println!("COLUMNS: 0 -> 2");
        println!("quit: exit game");
        println!("flip colors:flip face direction col/row");


        let input = read_input();
        let converted_input = input.trim().to_uppercase(); 
        
        if converted_input.trim() == possible_inputs[0]
        {
            break;
        }
        else if converted_input.contains(possible_inputs[1])
        {
            let actions: [&str; 4] = ["UP","DOWN","LEFT","RIGHT"];
            
            let faces: [&str; 6] = ["TOP","LEFT","FRONT","RIGHT","DOWN","BACK"];
            let separate_input:Vec<&str> = converted_input.split(' ').collect();
            if faces.contains(&separate_input[1]) && actions.contains(&separate_input[2])
            {
                let face = match separate_input[1]
                {
                    "TOP" => {0},
                    "LEFT" => {1},
                    "FRONT" => {2},
                    "RIGHT" => {3},
                    "DOWN" => {4},
                    "BACK" => {5}
                    &_ => {-1}
                };

                if !["0","1","2"].contains(&separate_input[3])
                {
                    error = "COLUMN DOES NOT EXIST"
                }
                else if face != -1
                {
                    cubo_magico.flip_cube(face, separate_input[2], 
                        if separate_input[3] == "0" {0} 
                        else if separate_input[3] == "1" {1} else {2})
                }
                
            }
            else
            {
                error = "FACE OR DIRECTION DOES NOT EXIST"
            }
        }
        else
        {
            error = "COMMAND DOES NOT EXIST"
        }


        println!("-------------------------------------------");
        cubo_magico.display_cube();
        println!("-------------------------------------------");

        if error != "n"
        {
            println!("{}",error);
        }

       
    }
    println!("good game");
}


fn read_input() -> String {

    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("can't read line");
    
    return input;
}