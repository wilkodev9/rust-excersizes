

#[cfg(test)]
mod test {
    use crate::odd::ex1::test;
    #[test]
    fn even(){
        assert_eq!("even", test("2"));    
        assert_eq!("even", test("4"));    
        assert_eq!("even", test("6"));    
        assert_eq!("even", test("8"));    
    }
    #[test]
    fn odd(){
        assert_eq!("odd", test("1"));    
        assert_eq!("odd", test("3")); 
        assert_eq!("odd", test("5")); 
        assert_eq!("odd", test("7")); 
    }
}