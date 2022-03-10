
fn read_string() -> String {
    let mut string = String::new();
    let result = std::io::stdin().read_line(&mut string);
    if result.is_err() {
        panic!();
    }
    return string;
}  

fn main() {
    println!("Insert start number:");
    let mut start_number = read_string().trim().parse::<i32>().unwrap();
    loop {
        println!("choose an operation: sum or mult or exit");
        let op = read_string();
        match op.trim() {
            "sum" => {
                println!("Type a number:");
                let first = read_string().trim().parse::<i32>().unwrap();
                start_number = start_number + first;
            },
            "mult" => {
                println!("Type a number:");
                let first = read_string().trim().parse::<i32>().unwrap();
                start_number = start_number * first;
            },
            "exit" => break,
            "" => continue,
            x => println!("Ivalid operation {:?}", x),
        }
    }
    println!("The final number is: {start_number}");
}
