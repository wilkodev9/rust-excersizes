

#[cfg(test)]
mod test {
    use crate::word_splitting::word_splitting::test_word_splitting;
    #[test]
    fn charcter_array(){
        assert_eq!(vec!["even", "odd"], test_word_splitting("even odd"));    
    }
}