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



///
///pass by author 
/// 
/// 
pub fn anagrams_for1<'a>(word: &str, candidates: &[&'a str]) -> HashSet<&'a str> {
    let nword = normalize(word);
    let mut nword_sorted = nword.clone();
    nword_sorted.sort_unstable();
    candidates.iter()
        .filter(|c| is_anagram(&nword, &normalize(c), &nword_sorted))
        .copied()
        .collect()
}
fn is_anagram(word1: &[char], word2: &[char], word1_sorted: &[char]) -> bool {
    let mut result = false;
    if word1.len() == word2.len() && word1.ne(word2) {
        let mut list2 = word2.to_owned();
        list2.sort_unstable();
        result = word1_sorted.eq(&list2);
    }
    result
}
fn normalize(word: &str) -> Vec<char> {
    word.to_lowercase().chars().collect()
}