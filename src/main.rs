
fn read_string() -> String {
    let mut string = String::new();
    let result = std::io::stdin().read_line(&mut string);
    if result.is_err() {
        panic!();
    }
    return string;
}

struct TheNumber(i32);

fn add_number(the_one: &mut TheNumber, to_add: i32) {
    the_one.0 = the_one.0 + to_add;
}

fn multiply_number(the_one: &mut TheNumber, to_multiply: i32) {
    the_one.0 = the_one.0 * to_multiply;
}

fn result_number(the_one: TheNumber) {
    println!("The final number is: {}", the_one.0);
}

fn main() {
    println!("Insert start number:");
    let mut start_number = TheNumber(read_string().trim().parse::<i32>().unwrap());
    loop {
        println!("choose an operation: sum or mult or exit");
        let op = read_string();
        match op.trim() {
            "sum" => {
                println!("Type a number:");
                let first = read_string().trim().parse::<i32>().unwrap();
                add_number(&mut start_number, first);
            },
            "mult" => {
                println!("Type a number:");
                let first = read_string().trim().parse::<i32>().unwrap();
                multiply_number(&mut start_number, first);
            },
            "exit" => break,
            "" => continue,
            x => println!("Ivalid operation {:?}", x),
        }
    }
    result_number(start_number);
}
