/*
    with this exersice im i will check if a number is odd or even
*/

use std::num;

pub fn ex(){
    println!("enter a number and i will check if its odd or even");
    let inp = input();
    output(inp);
}   

fn input() -> String {
    let mut numb = String::new();
    std::io::stdin().read_line(&mut numb);
    return numb;
}

fn output(inp: String) {
    print!("you entered {}", inp)
}

fn number_filter(mut number: String){
    for i in no_space(number.trim().to_string()).chars(){
        if i == - {
            number = number.replace("-","");
            //resets the program
            let number = number_filter(number);
            return number;
        }
    }
}

fn odd_or_even(){

}

//this makes sure that i dont have any form of white space
fn no_space(x : String) -> String{
    x.replace(" ", "").trim().to_string()
  }
