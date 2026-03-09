fn main() {
    //let mut s = String::from("hello");

    //change(&mut s);
    //println!("{s}");
    //print!("mutables : ");
    //mutables();
    let mut s = String::from("hello");
    let new_s = slice_str(s);
    println!("{new_s}");
}


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn mutables() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    //let r2 = &mut s;
    let r2 = &r1;

    println!("{r1}, {r2}");
}

fn slice_str(s: String) -> String {
    let mut res = Vec::new();

    for char in s.chars() {
        if char == ' ' {
            break;
        } else {
            res.push(char);
        }
    }

    res.iter().collect::<String>()
}
