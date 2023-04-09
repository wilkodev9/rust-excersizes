/*
    with this exersice im i will check if a number is odd or even
*/

pub fn odd_or_even(){
    println!("enter a number and i will check if its odd or even");
    let inp = input();
    output(inp);
}   

pub fn test(x: &str)-> String{
    let numb = x.to_string();
    let filter = number_filter(numb);
    return filter;
}

fn input() -> String {
    let mut numb = String::new();
    std::io::stdin().read_line(&mut numb);
    let filter = number_filter(numb);
    return filter;
}

fn output(imp: String) {
    print!("your number is {}", imp)
}

fn number_filter(mut number: String) -> String{
    for i in no_space(number.trim().to_string()).chars(){
        if i == '-' {
            number = number.replace("-","");
            let number = number_filter(number);
            return number.to_string();
        }
        if i.is_numeric() == false {
            println!("please only use numbers");
            let new_input = input();
            return new_input.to_string();
        }
    }
    return even_or_odd(number.to_string());
}

fn even_or_odd(x: String)-> String{
    let blep: i32 = x.trim().parse().unwrap();
    if blep % 2 == 0 {
        return "even".to_string();
    } else {
        return "odd".to_string();
    }
}

//this makes sure that i dont have any form of white space
fn no_space(x : String) -> String{
    x.replace(" ", "").trim().to_string()
  }