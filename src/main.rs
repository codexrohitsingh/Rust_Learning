fn main() {
    // println!("Hello, this is my first rust project ");
    // let x :i8= 5; //i means signed and it can both postive and negative 
    // let y :u8 =10;  //unsigned mean you can only use positive 
    // let z :u32= 1000;
    // let a :f32=1000.001; //this is for float 
    // println!("float is  : {}",a);
    // println!("{}",x);
    // println!("{}",y);
    // println!("{}",z);
    // let mut x :i8 = 5;

    // for i : i32 in 0..1000 {
    //     x= x+10;
    // }
    // for i: i32 in 0..1000{
    //     x=x+10;
    // }
    // for _i in 0..1000{
    //     x=x+100;
    // }
    // print!("{}",x)

    // let is_male:bool=false;

    // let is_above_18:bool = true;
    // if is_male{
    //     println!("Yes it a male")

    // }
    // else {
    //     print!("It is not a male ")
    // }
    // if is_male && is_above_18 {
    //     print!("It is a real male ")
    // }
    // let greeting =String::from ("hello World");
    // println!(" {}",greeting);
    // let char1 =greeting.chars().nth(10);
    // print!("{}",char1.unwrap());

    // match char1 {
    //     Some(c)=>println!("{}",c),
    //     None=>println!("No character at 1000th")
    // }
    // print!("{}",greeting.chars().nth(10));

    // Conditional and loops
    // let is_even :bool = false;
    // if is_even{
    //     println!{"This is even "}
    // }else{
    //     println!("This is Odd")
    // }
    // for i in 0..10{
    //     println!("{}",i)
    // }
    let first_sentence = String::from("This is a sentence ");
    let find_first_word=get_first_word(first_sentence);
    fn get_first_word(first_sentence: String)->String{
        let mut word = String::from("");
        for char in first_sentence.chars(){
            word.push_str(char.to_string().as_str());
            if char== ' '{
                break;
        }
    }
    return word;
}
println!("{}",find_first_word)
}
