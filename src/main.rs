use core::num;
use std::thread;
use std::fs::File;
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
    //let value_2 = None;
    let value_3 = value_1.unwrap();
    //let value_4 = value_2.unwrap();

    let path = "text.txt";
    //let file = File::open(path).expect("Failed to open file");
    fn open_file(path: &str) -> std::io::Result<File> {
        // Attempt to open the file and return the result
    let file = File::open(path)?;
        Ok(file)
    }
    match open_file(path) {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(e) => println!("Error opening file: {}", e),
    }

    let numbers = vec![1, 2, 3, 4, 5];

    match largest_num_in_list(&numbers) {
        Some(largest) => println!("The largest number is: {}", largest),
        None => println!("The list is empty."),
    }

    let chara = vec!['a', 'e','i', 'o','u'];

    match largest_num_in_list(&chara) {
        Some(largest) => println!("The largest number is: {}", largest),
        None => println!("The list is empty."),
    }

    
}


fn largest_num_in_list<T: std::cmp::PartialOrd>(list: &[T]) -> Option<&T> {

    if list.is_empty() {
        return None;
    }

    let mut largest = &list[0];

    for num in list {

        if num > &largest {
            largest = num;
        }
    }

    Some(largest)
}