
use rustpl::{twosum, Button, Draw, Screen };
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
let nums = vec![5 ,4, 5 ,2,7];

let result = twosum::two_sum(nums,10);

print!("that two number sum are : {:?}",result);

}
