/*

features:
   word length checker
   percentage of sentence thats words and digits and amount
   upper and lowercase case charcter amount + percentage
   word counter
   charcter counter
   sort longest word to shortest word
   checks if the numbers are even or odd
   checks sentence lenght
   create a text document to save this data to
   
to-do:
   sepperate input out of main
   put the unit tests into the files of original
   write out the the pseudo code for all the things that need to get done
   create a flow chart for all teh interactions
*/

use crate::odd_or_even::odd_or_even::odd_or_even;
use crate::sentence_lenght::sentence_lenght::sentence_lenght;
use crate::word_splitting::word_splitting::word_splitting;
pub mod word_splitting;
pub mod odd_or_even;
pub mod sentence_lenght;

fn main() {
   x = 
}

fn input() -> String {
   let mut numb = String::new();
   std::io::stdin().read_line(&mut numb);
   // let filter = number_filter(numb);
   return numb;
}