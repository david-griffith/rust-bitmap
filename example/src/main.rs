extern crate bitfont;

fn main() {
    use bitfont::bitmap_luma;
    let (my_len, my_vec) = bitmap_luma("abc123456", 8, 0).unwrap();
    for (my_count, my_char) in my_vec.iter().enumerate() {
        if my_char > &0 {
            print!("█");
        } else {
            print!(" ");
        }
        if (my_count + 1) % my_len == 0 {
            println!();
        }
    }

    use bitfont::bitmap_bool;
    let (my_len, my_vec) = bitmap_bool("abc123456").unwrap();
    for (my_count, my_char) in my_vec.iter().enumerate() {
        if *my_char {
            print!("█");
        } else {
            print!(" ");
        }
        if (my_count + 1) % my_len == 0 {
            println!();
        }
    }

}
