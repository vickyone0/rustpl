
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
}