#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
pub fn sublist<T>(_first_list: &[T], _second_list: &[T]) -> Comparison
where
    T: PartialEq,
    T:Clone,
{
    //unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");

    if _first_list == _second_list {
        return Comparison::Equal;
    } else if is_sublist(_first_list, _second_list) {
        return Comparison::Sublist;
    } else if is_sublist(_second_list, _first_list) {
        return Comparison::Superlist;
    } else {
        return Comparison::Unequal;
    }
}

fn is_sublist<T>(f: &[T], s: &[T]) -> bool
where
    T: PartialEq,
    T:Clone,
{
    let vec_f=f.to_vec();
    let vec_s=s.to_vec();
    f.is_empty() || s.windows(f.len()).any(|w| w == f) //(f.iter().all(|item| s.contains(item)))

}
