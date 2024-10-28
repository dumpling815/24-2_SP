use std::collections::HashMap;

#[derive(Debug)]
pub struct my_struct {
    my_string: String,
    my_num: i32,
    // We will assume that my_num should be in 0~99 !
}
pub trait structed {
    fn show_metadata(&self) -> &str;
}

impl structed for my_struct{
    fn show_metadata(&self)-> &str{
        let ret = &(self.my_string);
        ret
    }
}

impl my_struct {
    pub fn new(in1 : &str, in2: i32) -> my_struct{
        if in2 < 0 || in2 > 99 {
            panic!("Integer value should be in 0~99!");
        }
        else{
            my_struct{
                my_string : String::from(in1),
                my_num : in2,
            }
        }
    }
    pub fn int_update(&mut self, input: i32) {
        if input < 0 || input > 99 {
            panic!("Integer value should be in 0~99!");
        }
        else{
            self.my_num = input;
        }
    }
}

fn main() {

    let mut a = my_struct{
        my_string : String::from("Hello world"),
        my_num : 17,
    };
    let robber: String = a.my_string;
    println!("{robber}");
    println!("{}",a.my_num);
    a.my_string = String::from("Hello robber");
    println!("{a:?}");


    let mut my_vec : Vec<i32> = vec![1,2,3];
    let robber: i32 = my_vec[2];
    my_vec[2] = 5;
    println!("{0}",my_vec[2]);
    println!("------------------------");

    let que = String::from("Favorite color");
    let val = String::from("Crimson");
    let mut map = HashMap::new();
    map.insert(que,val);
    // println!("{}",map["Favorite color"]);
    println!("{map:?}");
    println!("------------------------");
    a = my_struct::new("New structure!",21);
    println!("{a:?}");
    a.int_update(23);
    println!("{a:?}");
    let trait_return = a.show_metadata();
    println!("{trait_return:?}");
}
