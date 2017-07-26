# rust-bitmap
A very simple bitmap font generator for rust.

Contains a simple 8x8 bitmap font containing ASCII character codes from 0x20 - 7F (Space, 0-9, A-Z, a-z, punctuation)
If the string contains characters outside the 0x20 - 0x7F ASCII range, an error is returned.

It contains a few functions that you feed a string, and you'll get a Result in return that has a vector of 8 vectors, with each vector containing the value for a single row of the generated bitmap font.
This can then be easily looped through in your program to plug pixels into whatever imaging library you're using, or print to the console as in the example below.

 Functions currently implemented are:
 - A luma function that takes foreground/background brightness values and returns a vector of those, useful for greyscale.
 - A boolean function that simply returns a vector of true/false values that you can do what you want with. 
 
 Example code:
 
 ```rust
    extern crate bitfont;
    
    fn main() {
        use bitfont::bitmap_luma;
        let my_vec = bitmap_luma("abc123456", 8, 0).unwrap();
        for my_row in my_vec {
            for my_char in my_row {
                if my_char > 0 {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        
        use bitfont::bitmap_bool;
        let my_vec = bitmap_bool("abc123456").unwrap();
        for my_row in my_vec {
            for my_char in my_row {
                if *my_char {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
```
 
 Running the example code above should generate the following from the luma and bool functions:

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
