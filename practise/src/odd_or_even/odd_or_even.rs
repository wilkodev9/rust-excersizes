#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn even_test(){
        assert_eq!("even", odd_or_even("2"));    
        assert_eq!("even", odd_or_even("4"));    
        assert_eq!("even", odd_or_even("6"));    
        assert_eq!("even", odd_or_even("8"));   
        assert_eq!("even", odd_or_even("-2"));    
        assert_eq!("even", odd_or_even("-4"));    
        assert_eq!("even", odd_or_even("-6"));    
        assert_eq!("even", odd_or_even("-8"));    
        
    }
    #[test]
    fn odd_test(){
        assert_eq!("odd", odd_or_even("1"));    
        assert_eq!("odd", odd_or_even("3")); 
        assert_eq!("odd", odd_or_even("5")); 
        assert_eq!("odd", odd_or_even("7")); 
        assert_eq!("odd", odd_or_even("-1"));    
        assert_eq!("odd", odd_or_even("-3")); 
        assert_eq!("odd", odd_or_even("-5")); 
        assert_eq!("odd", odd_or_even("-7")); 
    }

    #[test]
    #[should_panic]
    fn text_odd_even_test() {
        odd_or_even("blep");
    }
}
pub fn odd_or_even(x: &str)-> &str{
    return "a string"
}


// /*
//     with this exersice im i will check if a number is odd or even
// */

// pub fn odd_or_even(x: &str)-> String{
//     let numb = x.to_string();
//     // let filter = number_filter(numb);
//     return numb;
// }

// // fn number_filter(mut number: String) -> String{
// //     for i in no_space(number.trim().to_string()).chars(){
// //         if i == '-' {
// //             number = number.replace("-","");
// //             let number = number_filter(number);
// //             return number.to_string();
// //         }
// //         if i.is_numeric() == false {
// //             println!("please only use numbers");
// //             let new_input = input();
// //             return new_input.to_string();
// //         }
// //     }
// //     return even_or_odd(number.to_string());
// // }

// fn even_or_odd(x: String)-> String{
//     let blep: i32 = x.trim().parse().unwrap();
//     if blep % 2 == 0 {
//         return "even".to_string();
//     } else {
//         return "odd".to_string();
//     }
// }

// //this makes sure that i dont have any form of white space
// fn no_space(x : String) -> String{
//     x.replace(" ", "").trim().to_string()
//   }