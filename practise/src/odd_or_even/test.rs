

#[cfg(test)]
mod test {
    use crate::odd_or_even::odd_or_even::test;
    #[test]
    fn even_test(){
        assert_eq!("even", test("2"));    
        assert_eq!("even", test("4"));    
        assert_eq!("even", test("6"));    
        assert_eq!("even", test("8"));   
        assert_eq!("even", test("-2"));    
        assert_eq!("even", test("-4"));    
        assert_eq!("even", test("-6"));    
        assert_eq!("even", test("-8"));    
        
    }
    #[test]
    fn odd_test(){
        assert_eq!("odd", test("1"));    
        assert_eq!("odd", test("3")); 
        assert_eq!("odd", test("5")); 
        assert_eq!("odd", test("7")); 
        assert_eq!("odd", test("-1"));    
        assert_eq!("odd", test("-3")); 
        assert_eq!("odd", test("-5")); 
        assert_eq!("odd", test("-7")); 
    }

    #[test]
    #[should_panic]
    fn text_odd_even_test() {
        test("blep");
    }
}