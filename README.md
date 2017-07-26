# rust-bitmap
A very simple bitmap font generator for rust.

Contains a simple 8x8 bitmap font containing ASCII character codes from 0x20 - 7F (Space, 0-9, A-Z, a-z, punctuation)

It contains a few functions that you feed a string, and you'll get a Result in return :
 - a length at which a new row should start, and
 - a vector containing the bitmap.

If the string contains characters outside this range, an error is returned.

 Functions currently implemented are:
 - A luma function that takes foreground/background brightness values and returns a vector of those, useful for greyscale.
 - A boolean function that simply returns a vector of true/false values that you can do what you want with. 
 
 Example code:
 
 ```rust
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
```
 
 Running the example code above should generate the following from the grayscale and mono functions:

```
        ███               ██     ████    ████      ███  ██████    ███   
         ██              ███    ██  ██  ██  ██    ████  ██       ██     
 ████    ██      ████     ██        ██      ██   ██ ██  █████   ██      
    ██   █████  ██  ██    ██      ███     ███   ██  ██      ██  █████   
 █████   ██  ██ ██        ██     ██         ██  ███████     ██  ██  ██  
██  ██   ██  ██ ██  ██    ██    ██  ██  ██  ██      ██  ██  ██  ██  ██  
 ███ ██ ██ ███   ████   ██████  ██████   ████      ████  ████    ████   
                                                                        
        ███               ██     ████    ████      ███  ██████    ███   
         ██              ███    ██  ██  ██  ██    ████  ██       ██     
 ████    ██      ████     ██        ██      ██   ██ ██  █████   ██      
    ██   █████  ██  ██    ██      ███     ███   ██  ██      ██  █████   
 █████   ██  ██ ██        ██     ██         ██  ███████     ██  ██  ██  
██  ██   ██  ██ ██  ██    ██    ██  ██  ██  ██      ██  ██  ██  ██  ██  
 ███ ██ ██ ███   ████   ██████  ██████   ████      ████  ████    ████   
```
