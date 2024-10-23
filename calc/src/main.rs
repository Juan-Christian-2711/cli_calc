use std::io;


fn main() {
    let mut quit = false;

    while quit == false{
        quit = calc();       
    }
}

fn calc() -> bool {
    let mut equation = String::new();
    io::stdin()
        .read_line(&mut equation)
        .expect("Failed to read line");
    if equation == String::from("q\n"){
        println!("exiting program");
        return true;
    }
    
    match operator {
                
    }

    false
}
