use std::io;
use regex::Regex;

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
    
    //split the string into Coefficients, and operators

    let re = Regex::new(r"(\d+|[+\-*/()])").unwrap();
    let mut equation_vec = Vec::new();

    for mat in re.find_iter(equation.as_str()) {
        equation_vec.push(&equation[mat.start()..mat.end()]);
    }

    println!("{:?}", equation_vec);

    false
}
