use std::thread;
#[derive(Debug)]
enum IpAddr {
 V4(u8, u8, u8, u8),
 V6(String),
}

fn main() {
    // Example usage of the IpAddr struct
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
       
       
    println!("Home IP: {:?} ", home) ;
    println!("Loopback IP: {:?}", loopback );


    let mut data = vec![1, 2, 3, 4, 5];

    thread::spawn(move || {
        println!("{:?}", data);
    }).join().unwrap();


    let value_1 = Some(5);
    let value_2 = None;
    let value_3 = value_1.unwrap();
    let value_4 = value_2.unwrap();
}