fn main() {
    //let mut s = String::from("hello");

    //change(&mut s);
    //println!("{s}");
    //print!("mutables : ");
    //mutables();
    let s = String::from("hello world");
    let new_s = slice_str(&s);
    let second = second_word(&s);
    println!("first word: {new_s}");
    println!("second word: {second}");
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

fn slice_str(s: &String) -> &str {
    //let mut res = Vec::new();

    //for char in s.chars() {
    //    if char == ' ' {
    //        break;
    //    } else {
    //        res.push(char);
    //    }
    //}

    //res.iter().collect::<String>()
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }
    &s[..]
 
}
