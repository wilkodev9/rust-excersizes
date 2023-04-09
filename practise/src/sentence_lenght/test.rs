

#[cfg(test)]
mod test {
    use crate::sentence_lenght::sentence_lenght::test;
    #[test]
    fn lenght(){
        assert_eq!("even", test("4"));    
        assert_eq!("odd", test("3"));    
        assert_eq!("linux", test("5"));    
        assert_eq!("windows", test("7"));    
        assert_eq!("reallyLongWord", test("14"));   
    }
}