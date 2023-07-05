pub fn is_leap_year(year: u64) -> bool {
    //unimplemented!("true if {year} is a leap year")
    if (year % 400 == 0) && (year % 4 == 0) {
        return true;
    }
    if (year % 4 == 0) && (year % 100 != 0) {
        return true;
    } else if (year % 100 != 0) && (year % 4 != 0) {
        return false;
    } else {
        return false;
    }
}
