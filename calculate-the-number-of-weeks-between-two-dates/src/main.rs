/*
    Calculate the number of weeks between two dates
*/

fn weeks_between(a: &str, b: &str) -> i32 {
    let differences: Vec<i32> = b
        .split("-")
        .zip(a.split("-"))
        .map(|(a, b)| a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap())
        .collect();

    differences
        .iter()
        .zip(vec![365, 30, 1].iter())
        .map(|x| x.0 * x.1)
        .sum::<i32>()
        / 7

    /*
    use chrono::NaiveDate;

    let t1 = NaiveDate::parse_from_str(a, "%Y-%m-%d").unwrap();
    let t2 = NaiveDate::parse_from_str(b, "%Y-%m-%d").unwrap();

    let n_weeks = (t2 - t1).num_days() / 7;
    n_weeks as i32
        */
}

fn main() {
    let n_weeks = weeks_between("2010-01-21", "2010-10-21");

    println!("hello: {}", n_weeks);
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10");
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18");
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");
    assert_eq!(n_weeks, -1);
}
