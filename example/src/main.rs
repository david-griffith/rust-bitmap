extern crate bitfont;

fn main() {
    use bitfont::bitmap_luma;
    let my_vec = bitmap_luma("abc123456", 8, 0).unwrap();
    for row_vec in my_vec {
        for my_char in row_vec {
            if my_char > 0 {
                print!("█");
            } else {
                print!(" ");
            }
        }
    }

    use bitfont::bitmap_bool;
    let my_vec = bitmap_bool("abc123456").unwrap();
    for row_vec in my_vec {
        for my_char in row_vec {
            if my_char {
                print!("█");
            } else {
                print!(" ");
            }
        }
    }

}
