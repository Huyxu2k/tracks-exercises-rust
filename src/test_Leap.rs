use  crate::Leap::{is_leap_year};

fn process_leapyear_case(year: u64, expected: bool) {
    assert_eq!(is_leap_year(year), expected);
}
#[test]
fn test_year_not_divisible_by_4_common_year() {
    process_leapyear_case(2015, false);
}
#[test]
fn test_year_divisible_by_2_not_divisible_by_4_in_common_year() {
    process_leapyear_case(1970, false);
}
#[test]
fn test_year_divisible_by_4_not_divisible_by_100_leap_year() {
    process_leapyear_case(1996, true);
}
#[test]
fn test_year_divisible_by_4_and_5_is_still_a_leap_year() {
    process_leapyear_case(1960, true);
}
#[test]
fn test_year_divisible_by_100_not_divisible_by_400_common_year() {
    process_leapyear_case(2100, false);
}