use std::io;

struct board{
    data: [[char;9];9]
}
impl for board{
    fn create() -> Self{
        mut board{
            data: [['_';9];9],
        }
    }
    fn show(&self){
        println!("   1 2 3 4 5 6 7 8 9");
        println!("   _ _ _ _ _ _ _ _ _");
        for i in 1..10{
            print!("i ");
            for j in 1..10{
                print!("|{self[i][j]}");
            }
            println!("|");
        }
    }
    fn put_block(){

    }
}

enum blocks{
    b1: [[char;3];3] = [['@']]
}

struct block{
    // Possible plock is total 14. Variation by rotation is 4 per block.
    fn init(){

    }
    fn rotate(&self){

    }
}

impl for block{
    fn get_block(player: char) -> Self{

    }
}



fn get_instruction() -> Result<(i8,i8), Err>{
    print!("Put your block (r c) or Rotate(0):");
    let mut instruction = String::new();
    io::stdin()
        .read_line(&mut instruction)
        .expect("Failed to get instruction");
    let ins_iter = instruction.trim().split_whitespace();   //variable shadowing, trim returns &str
    val = ins_iter.next().unwrap();
    if val == '0'{
        Ok(0,-1)
    }
    else if val > 0 & val < 10{
        row = val.parse().unwrap();
        col = ins_iter.next().unwrap().parse().unwrap;
        Ok(row,col)
    }
    else{
        Err
    }

    for inst in instruction{
        val = inst.parse::<i8>.expect("Not converatable")
        if val == 0{
            Ok(0,-1)
        }
        else if val < 10 & val > 0 {

        } 
    }


}