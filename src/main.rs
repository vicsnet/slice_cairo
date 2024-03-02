fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello world");

    let word = first_world(&s); //word will get the value 5

    s.clear(); //this empty the sring making it equals to ''
    println!("The length is:{word}");

    // string slice
    // let _hello = &s[0..4];
    let _world = &s[6..11];

    // println!("hey, {hello}");

    let sec = second_word(&s);
    println!("my second {sec}");

    

}

fn first_world(s: &String) ->usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
           
        }
    }
    s.len()
    
}

fn second_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
          
            return &s[0..i];
        }
    }
   
    &s[..]
}