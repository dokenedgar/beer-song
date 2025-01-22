pub fn verse(n: u32) -> String {
    let verse_string: &str;
    let formatted: String;
    if n <= 0 {
        verse_string = "
\nNo more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.";
    } else if n == 1 {
        verse_string = "
\n1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall."
    } else if n == 2 {
        verse_string = "
\n2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall."
    } else {
        formatted = format!(
            "\n\n{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.",
            n,
            n,
            n - 1
        );
        verse_string = formatted.as_str();
    }
    verse_string.to_string()
}

pub fn sing(start: u32, end: u32) -> String {
    // todo!("sing verses {start} to {end}, inclusive")
    let mut resultant_verses: String = String::from("");
    let mut start_mut = start;
    loop {
        // println!("IN LOOP: {}, {}", start_mut, end);
        if end <= start_mut {
            resultant_verses += &verse(start_mut);
            if start_mut > 0 {
                start_mut -= 1;
            } else {
                break;
            }

            println!("{start_mut}");
        } else {
            break;
        }
    }

    resultant_verses
}
