use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // unimplemented!(
    //     "For the '{word}' word find anagrams among the following words: {possible_anagrams:?}"
    // );
   let out_lay2=HashSet::from_iter(possible_anagrams.iter().copied().filter(|x| { 
      check_char(&low(word), &low(x))
   }));
   out_lay2
}
fn check_char(word :&str,input2 :&str)->bool{
    let mut result = false;
    if word.len()==input2.len() && word.ne(input2){
       let mut word_vec= word.chars().collect::<Vec<char>>();
       let mut vec_input2:Vec<char>=input2.chars().collect();
       word_vec.sort_unstable();
       vec_input2.sort_unstable();
       result=word_vec.eq(&vec_input2);
    }
    result
}
fn low(input:&str)->String{
    input.to_lowercase()
}
