// board_module is for entire board.
// 1. structure of board. 9 * 9 fixed size array consist of data type char
// 2. init() : initialize Board structure.
// 3. show() : printing the current Board structure.
// 4. put_possible() : block, coordinate given, check it is possible to put block in the place.
// 5. put_block() : putting block into given coordinate.
// 6. check_lose() : checking if there is possible place to put block.  (Iterator, Closure used)
pub mod board_module{
    use super::block_module;
    pub struct Board{
        pub data: [[char;9];9],
    }
    impl Board{
        pub fn init() -> Self{
            Self{
                data: [['_';9];9],
            }
        }
        pub fn show(&self){
            println!("   1 2 3 4 5 6 7 8 9");
            println!("   _ _ _ _ _ _ _ _ _");
            for i in 0..9{
                print!("{} ",i+1);
                for j in 0..9{
                    print!("|{}",self.data[i][j]);
                }
                println!("|");
            }
            print!("\n");
        }
        fn put_possible(&self,block: &block_module::Block, coordinate:(i8,i8)) -> bool{
            let row = coordinate.0 as usize;
            let col = coordinate.1 as usize;
            if row > 7 || col > 7 || row  < 1 || col < 1{
                return false
            }
            let put = block.data;
            //let target = &self.data[row .. row +3][col .. col +3];
            let mut res: bool = true;
            'outer_for: for i in 0..3{
                for j in 0..3{
                    if put[i][j] != '_' && self.data[row+i-1][col+j-1] != '_' {
                        res = false;
                        break 'outer_for;
                    }
                }
            }
            res
        }
        pub fn put_block(&mut self, block: &block_module::Block, coordinate:(i8,i8)) -> Option<()>{
            let poss = self.put_possible(block,coordinate);
            if !poss {
                None
            }
            else{
                let row = coordinate.0 as usize;
                let col = coordinate.1 as usize;
                for i in 0..3{
                    for j in 0..3{
                        if block.data[i][j] != '_'{
                            self.data[row +i -1][col + j-1] = block.data[i][j];
                        }
                    }
                }
                Some(())
            }
        }
        pub fn check_lose(&self,block: &block_module::Block) -> bool{
            // if player lose 100%(checkmate), return true.
            // inital value is true, that means, the base assumption is lose.
            // if we find the possiblity to put block to board, make res = false.
            let mut res = true;
            let mut search_coordinate_iter = (1..=7).flat_map(|x| (1..=7).map(move |y| (x,y)));
            let _rot_block = &(block.rotate());
            'outer: for _i in 1..5{
                loop{
                    res = match search_coordinate_iter.next() {
                        Some(coor) => !self.put_possible(_rot_block, coor), //because put_possible is true when we 'can' put.
                        None => break,
                    };
                    if res == false{
                        break 'outer;
                    }
                }
                let _rot_block = &(_rot_block.rotate());
            }
            res
        }
    }
}

// block_module is for block structure of block, functions
// 1. Player enum is used to generate the proper block for player.
// 2. structure of block.
// 3. get_block() : by using player enum, block_num(random number in range 0~13 is given), returns proper block.
// 4. rotate() : roating block, returns rotated block by 90 degree.
// 5. show() : printing block structure.
mod block_module{
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum Player{
        P1,
        P2,
    }

    #[derive(PartialEq)]
    #[derive(Debug)]
    pub struct Block{
        // Possible block is total 14. Variation by rotation is 4 per block.
        pub data: [[char;3];3],
    }
    impl Block{
        pub fn get_block(player: &Player, block_num: i8) -> Self{
            let element:char = match player {
                Player::P1 => '0',
                Player::P2 => '@',
            };
            let block: Self = match block_num {
                0 => Block{data: [ [element,element,element],['_','_','_'],['_','_','_'] ]}, // 1,2,3
                1 => Block{data: [ [element,element,element],[element,'_','_'],['_','_','_']]}, //1,2,3,4
                2 => Block{data: [ [element,element,element],['_','_',element],['_','_','_']]}, //1,2,3,6
                3 => Block{data: [ [element,element,element],['_',element,'_'],['_','_','_']]}, //1,2,3,5
                4 => Block{data: [ ['_',element,element],[element,element,'_'],['_','_','_']]}, //2,3,4,5
                5 => Block{data: [ [element,element,'_'],['_',element,element],['_','_','_']]}, //1,2,5,6
                6 => Block{data: [ [element,element,'_'],[element,element,'_'],['_','_','_']]}, //1,2,4,5
                7 => Block{data: [ [element,'_','_'],[element,element,'_'],[element,element,'_']]}, // 1,4,5,7,8
                8 => Block{data: [ ['_',element,element],[element,element,'_'],[element,'_','_']]}, // 2,3,4,5,7
                9 => Block{data: [ [element,element,'_'],['_',element,element],['_',element,'_']]}, // 1,2,5,6,8
                10 => Block{data: [ ['_',element,element],[element,element,'_'],['_',element,'_']]},// 2,3,4,5,8
                11 => Block{data: [ ['_','_',element],[element,element,element],[element,'_','_']]},// 3,4,5,6,7
                12 => Block{data: [ [element,'_','_'],[element,element,element],['_','_',element]]},// 1,4,5,6,9
                13 => Block{data: [ ['_',element,'_'],[element,element,element],['_',element,'_']]},// 2,4,5,6,8
                _ => panic!("invalid block_num..."),
            };
            block
        }
        pub fn rotate(&self) -> Self{
            let rotated_block: Self = Block{data: [ [self.data[0][2],self.data[1][2],self.data[2][2]], [self.data[0][1],self.data[1][1],self.data[2][1]], [self.data[0][0],self.data[1][0],self.data[2][0]]]};
            rotated_block
        }
        pub fn show(&self){
            println!(" _ _ _ ");
            for i in 0..3{
                for j in 0..3{
                    print!("|{}",self.data[i][j]);
                }
                println!("|");
            }
            print!("\n");
        }
    }
}

// module for interact with players (command line)
// 1. get_instruction(): preprocess the given plaintext to usable data.
// 2. interact() : by using get_instruction, processing proper action. (if invalid, goes recursively)
// 3. gaming() : overall function called by main.rs, generating actual game process.
// 4. change_player() : changing the turns of the players.
pub mod interact_module{
    use std::io;
    use std::io::Write;
    use rand::Rng;
    use super::block_module::*;
    use super::board_module::Board;
    fn get_instruction() -> Option<(i8,i8)>{
        let mut instruction = String::new();
        io::stdin()
            .read_line(&mut instruction)
            .expect("Failed to get instruction");
        let mut ins_iter = instruction.trim().split_whitespace();   //variable shadowing, trim returns &str
        let val = ins_iter.next()?.parse().ok()?;

        if val == 0{
            Some((0,-1)) 
        }
        else if val > 0 && val < 8{
            let row = val;
            let col = ins_iter.next()?.parse().ok()?;
            if col > 0 && col < 8{
                Some((row,col))
            }
            else{
                None
            }
        }
        else{
            None
        }
    }
    fn interact(cur_board: &mut Board, mut cur_block: Block, cur_player: &Player){
        let mut ins_res:(i8,i8);
        loop {
            print!("Put your block (r c) or Rotate (0): ");
            let _ = io::stdout().flush();
            // flushing buffer.
            ins_res = match get_instruction(){
                Some(res) => {
                    res
                }
                None => {
                    println!("Invalid Input!");
                    continue;
                }
            };
            break;
        }
        match ins_res.0 {
            // invalid instructions are already filtered before.
            0 => { // when player want's to rotate
                cur_block = cur_block.rotate();
                println!("{:#?}'s block:",*cur_player);
                cur_block.show();
                interact(cur_board,cur_block,cur_player);
            }
            _ => { // when player want's to put the block
                match cur_board.put_block(&cur_block,ins_res) {
                    None => {
                        println!("{:#?} is not able to put the block into ({},{}).\n",cur_player,ins_res.0,ins_res.1);
                        println!("{:#?}'s block:",*cur_player);
                        cur_block.show();
                        interact(cur_board,cur_block,cur_player);
                    },
                    Some(()) => (),
                }
            }
        }
    }
    pub fn gaming(){
        let mut game_board = Board::init();
        let mut cur_player = Player::P1;
        let mut end_condition:bool;
        loop{
            game_board.show();
            let block_num:i8 = rand::thread_rng().gen_range(0..=13);
            let cur_block = Block::get_block(&cur_player,block_num);
            println!("{:#?}'s block:",cur_player);
            cur_block.show();
            end_condition = game_board.check_lose(&cur_block);
            if end_condition{
                break;
            }
            interact(&mut game_board,cur_block, &cur_player);
            change_player(&mut cur_player);
        }
        let winner = match cur_player{
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        };
        println!("{:#?} fails to put the block. {:#?} wins!! congratulation!",cur_player,winner);
    }
    pub fn change_player(cur_player: &mut Player){
        if *cur_player == Player::P1{
            *cur_player = Player::P2
        }
        else{
            *cur_player = Player::P1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_block(){
        use block_module::Block;
        use block_module::Player;
        for i in 0..14{
            let ret = Block::get_block(&Player::P2,i);
            ret.show();
        }
    }

    #[test]
    fn test_rotate(){ // Test for rotating block!
        use block_module::Block;
        let input = Block{data: [ ['@','@','_'],['_','@','@'],['_','@','_']] };
        let ans =  Block{data: [ ['_','@','_'],['@','@','@'],['@','_','_']]};
        let ret = input.rotate();
        assert_eq!(ans,ret);
    }

    #[test]
    fn test_init_show(){
        use board_module::Board;
        let res = Board::init();
        res.show();
    }

    #[test]
    fn test_put_block(){
        use board_module::Board;
        use block_module::*;
        let mut game = Board::init();
        game.show();
        let input = Block::get_block(&Player::P2,9);
        let res1 = game.put_block(&input,(7,7)).unwrap();
        game.show();
        let input = Block::get_block(&Player::P1,1);
        let res2 = game.put_block(&input,(1,1)).unwrap();
        game.show();
        assert_eq!(res1,());
        assert_eq!(res2,());
    }

    #[test]
    fn test_check_lose(){
        use board_module::Board;
        use block_module::*;
        let game = Board{data: [['@';9];9]};
        let input = Block::get_block(&Player::P1,2);
        let res1 = game.check_lose(&input);
        let game = Board::init();
        let res2 = game.check_lose(&input);
        assert_eq!(res1,true);
        assert_eq!(res2,false);
    }

    #[test]
    fn test_change_player(){
        let mut cur_player = block_module::Player::P1;
        interact_module::change_player(&mut cur_player);
        assert_eq!(cur_player,block_module::Player::P2);
    }
}