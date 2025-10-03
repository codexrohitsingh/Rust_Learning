// fn main() {
//     println!("Hello, this is my first rust project ");
//     let x :i8= 5; //i means signed and it can both postive and negative 
//     let y :u8 =10;  //unsigned mean you can only use positive 
//     let z :u32= 1000;
//     let a :f32=1000.001; //this is for float 
//     println!("float is  : {}",a);
//     println!("{}",x);
//     println!("{}",y);
//     println!("{}",z);
//     let mut x :i8 = 5;

//     for i : i32 in 0..1000 {
//         x= x+10;
//     }
//     for i: i32 in 0..1000{
//         x=x+10;
//     }
//     for _i in 0..1000{
//         x=x+100;
//     }
//     print!("{}",x)

//     let is_male:bool=false;

//     let is_above_18:bool = true;
//     if is_male{
//         println!("Yes it a male")

//     }
//     else {
//         print!("It is not a male ")
//     }
//     if is_male && is_above_18 {
//         print!("It is a real male ")
//     }
//     let greeting =String::from ("hello World");
//     println!(" {}",greeting);
//     let char1 =greeting.chars().nth(10);
//     print!("{}",char1.unwrap());

//     match char1 {
//         Some(c)=>println!("{}",c),
//         None=>println!("No character at 1000th")
//     }
//     print!("{}",greeting.chars().nth(10));

//     Conditional and loops
//     let is_even :bool = false;
//     if is_even{
//         println!{"This is even "}
//     }else{
//         println!("This is Odd")
//     }
//     for i in 0..10{
//         println!("{}",i)
//     }
//     let first_sentence = String::from("This is a sentence ");
//     let find_first_word=get_first_word(first_sentence);
//     fn get_first_word(first_sentence: String)->String{
//         let mut word = String::from("");
//         for char in first_sentence.chars(){
//             word.push_str(char.to_string().as_str());
//             if char== ' '{
//                 break;
//         }
//     }
//     return word;
// }
// println!("{}",find_first_word)
// }

// Find the first word of the sentence and the sentence is -> THis is a sentence
// fn main(){
// let sentence = String::from("THis is a sentence");

//     for i in sentence.chars(){
//     print!("{}",i);
//     if i==' '{
//         break;
//     }


// }

// }
// fn main (){
// println!("{}",do_sum(14,23 ));

// }
// fn do_sum(a:i32,b:i32)->i32 {
//     return a+b;
// }

// fn main(){
//     setup_stack();
//     setup_heap();
//     upgrade_string();
// }
// fn setup_stack(){
//     let a :u8=5;
//     let b :u8=10;
//      let c=a+b;
//     println!("The sum of a: {} and b: {} is {}",a,b,c);
// }
// fn setup_heap(){
//     let s1=String::from("This is sentence one");
//     let s2=String::from("this is sentence two");
//     let combined = format!("{},{}",s1,s2);
//     println!("{}",combined);
// }
// fn upgrade_string(){
//     let mut s1 =String::from("This is the second sentence");
//     println!("Capacity :{}, length: {}, pointer: {:p}",s1.capacity(),s1.len(),s1.as_ptr());
//     println!("This is before the update ");
//     s1.push_str(" This is the upgrade");
//     println!("Capacity :{}, length: {}, pointer: {:p}",s1.capacity(),s1.len(),s1.as_ptr());

//     print!("{s1}");

// }


// Write a basic program in rust to get the length of 2 first word of a string 
// fn main(){
//     let a =String::from("this is the string from the sentence");
// print!("{}",get_length(a));
// }

// fn get_length(a:String)->String{
//     let mut st=String::from(" ");
//     for i in a.chars(){
//         st.push_str(i.to_string().as_str());
//         if i==' '{
//             break;
//     }

// }
// return st;
// }

// Write a program to find the reverse of a number 123; 
// fn main (){
//     let a = 10;
//     find_reverse(a);
// }

// fn find_reverse(a:f32){
//     println!("{}",a%10);
    
// }
// Write a program to divide 2 numbers 
// fn main(){
//     print!("{}",10/2);
// }
// Make a function to do the same above division

// fn main(){
//     let s1 = String::from("this is the s1");
//     let s2 =s1;
//     let s3 = s2;
//     let s4=s1;
//     println!("{}",s3);
//     print!("{}",s4);
// }

// fn main(){
//     let my_string = String::from("this is the string");
//     new_string(my_string.clone());
//     println!("{}",my_string)
// }
// fn new_string(my_new_string:String){
//     println!("{}",my_new_string)
// }

// fn main(){
//     let mut my_string = String::from("this is the string");
//     my_string=new_string(my_string);
//     println!("{}",my_string)
// }
// fn new_string(my_new_string:String)->String{
//     println!("{}",my_new_string);
//     return my_new_string;
// }

// fn main(){
//     let my_string = String::from("this is the string");
//     let my_string_3=new_string(my_string);
//     println!("{}",my_string_3)
// }
// fn new_string(my_new_string:String)->String{
//     println!("{}",my_new_string);
//     return my_new_string;
// }

// fn main(){
//     let s1 = String::from("this is the sentence one ");
//     let s2 = &s1;
//     println!("{}",s2);
//     println!("{}",s1);
// }

// fn main(){
//     let s1 = String::from("this is the sentence one ");

//      using_function(&s1);
//      println!("{}",s1);
// }
// fn using_function(s1:&String){
//     println!("{}",s1);

// }
// fn main(){
//     let mut s1 =String::from("Hello");
//     s1.push_str("World");
//     println!("{}",s1);
// }
// fn main(){
//     let s1 = String::from("Hello");
//     print!("{}",format!("{}{}",s1,"World"));
//     println!("{}{}","hello","World")
// }
// fn main(){
//     let mut s1=String::from("Hello");
//     println!("This is before the hanky panky {}",s1);
//     update_string(&mut s1);
//     println!("This is after the hanky panky {}",s1)
// }
// fn update_string(s1:&mut String){
//     s1.push_str("World");
// }

// fn main(){
//     let mut s1 = String::from("Hello");
//     let s2 = &mut s1;
//     let s3 = &s1;
//     println!("{}",s2);
//     println!("{}",s3)
// }


// struct User{
//     name:String,
//     age:u32,

// }
// fn main(){
//     let user =User{
//         name:String::from("Rohit"),
//         age:12
//     };

//     println!("My name is {} and my age is {}",user.name,user.age)
// }

// struct Rect {
//      width:u32,
//      height:u32,
// }

// impl Rect {
//     fn area(&self)->u32{
//         return self.width*self.height;
//     }
// }
// fn main(){
//     let a = Rect{
//         width:32,
//         height:34,
//     };
//     print!("{}",a.area())

// // }
// enum Direction {
//     North,
//     South,
// }

// fn main(){
//     let a= Direction::North;
//     print!("{:?}",a)
// }


