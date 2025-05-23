
use rustpl::{twosum, Button, Draw, Screen };
use rustpl::ispalintrome::is_palintrome;
use rustpl::rustb::*;
struct SelectBox {
    width:u32,
    height:u32,
    options:Vec<String>,

}

impl Draw for SelectBox {
    fn draw(&self){
        //code for draw
    }
}


fn main(){

    let screen = Screen{
        components: vec![
            Box::new(SelectBox{
                width:10,
                height:30,
                options:vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe")
                ]
            }),
            Box::new(Button{
                width:20,
                height:10,
                label:String::from("Ok"),
            }),
        ]
    };
   screen.run();

//twosum   
let nums = vec![5 ,4, 5 ,2,7];

let result = twosum::two_sum(nums,10);

print!("that two number sum are : {:?}",result);

//is_palindrome
let data = String::from("abccba");
let result = is_palintrome(&data);

println!("{} is a palindrome ? {}", data,result);

let black = Color(0, 0, 0);
 let origin = Point(0, 0, 0);
let subject = AlwaysEqual;



}

pub struct Color(i32, i32, i32);
pub struct Point(i32, i32, i32);

struct AlwaysEqual;