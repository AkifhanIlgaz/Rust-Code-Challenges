/*
    Case-Insensitive Sort

    Write a function that sorts usernames, ignoring the case of any letters within them
*/

fn sort_usernames<T: AsRef<str> + PartialOrd>(usernames: &mut Vec<T>) {
    usernames.sort_by(|a, b| a.as_ref().to_lowercase().cmp(&b.as_ref().to_lowercase()))
}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted:   {:?}", &users);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}
