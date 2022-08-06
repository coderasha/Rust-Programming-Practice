variable- snake-case
Constants - UpperCase...Constants can't be shadowed like variables.
Struct- Camelcase


**##Rust Theories**
1. By default all declared Rustvariables are immutable i.e their values can't be changed.
   In order to change the value and make it mutable we should use "mut" keyword for the variable 
   Ex- Let mut x = 30;
   
**Infinite Loop**
   
fn main(){
let mut n = 0;

loop{
n+=1;

if n==7{

    return();
    // break;
}
if  n==3{
    continue;
}
println!("the number is {}",n);

}


}


**Tuple**
tuple is a way in rust to represent different data types in as single compound type.
for ex -
fn main(){
 let tup1 = (20,21,22);
 println!("the value at 2 is {}",tup1.2);
}

**vector**
its a module in rust that provides the container space to store values.In simple terms its a container that stores values like an array .
Syntax- Vec<T>  ---T--> type of Data
Creating a vector in Rust:
To create Vector simply follow the below-listed methods.

**1. Using Vec::new() Method:**
let v : Vec<i64> = Vec::new();  
Here v is the initialized vector that will contain the 64-bit integer datatype. It is initialized with help of the Vec::new() method.

fn main() {
 
    let v : Vec<i64> = Vec::new(); 
 
    // printing the size of vector
    println!("{ }",v.len());
}
** **2. Using macro in Rust: ****
let v = vec!['G','E','E','K','S'];   
Here this vector created using the macro vec!.  And it stores the value that we provide here which is char type.

fn main() {
 
    let v = vec!['G','E','E','K','S'];
 
    // printing the size of vector
    println!("{ }",v.len());
}
The iter() method returns an iterator object of the collection. Values in an iterator object are called items. The next() method of the iterator can be used to traverse through the items.
   
   fn main() {

let v = vec!["cat" ,"dog", "mew","bark"];
// let mut v1:Vec<T> = Vec::new();
// v1.push(27);
// v1.push(28);
// v1.push(29);

// println!("{}", v[1]);
// println!("{}", v1);

for (index,c) in v.iter().enumerate(){
println!("{} {} ", index,c);

}
}
   
**Enum**
   
  ** COnstants**
   Declared in Uppercase.and type declaration is mandatory.
 const MAXIMUM_NUM: u8 = 50;
   fn main(){
   for i in 1..MAXIMUM_NUM{
     if i%5==0{
     println!("{}", i)
   }
   }

   }
**function**
   // const MAXIMUM_NUM: u8 = 51;
// fn main(){
// for i in 1..MAXIMUM_NUM{
//   if i%5==0{
//   println!("{}", i)
// }
// }



// }

fn main() 
 {

 print_num(10)
 }

 fn print_num(n: u32){

    for i in 1..n
    {
      print!("{}",i);

    }
 }
   
 **  Shadowing**
    variable shadowing occurs when a variable declared within a certain scope  has the same name as a variable declared in an outer scope. 
   This outer variable is said to be shadowed by the inner variable, while the inner identifier is said to mask the outer identifier.
   
**  Reference**
   A reference to a variable can be used the same way as the actual variable.
   Ex if variable is X then its reference is created as &X
   Let x =10
   lwt xr= &x (reference to x)
   
   if we want to change the value of variable x, we can use mutable reference.
   let x=10;
   let dom = &mut x;
   dom+=1;
   println!({},dom);
   
  ** Struct**
   
   struct Color {
      red:u8,
     blue:u8,
   green:u8,
   }
   
   fn main(){
   let bg = Color{red:255 , blue:10 , green:50};
   println!("{}{}{}", bg.red,bg.blue, bg.green);
   
   
   }
  ** Tuple Structs**
   struct Color(u8,u8,u8)
   fn main()
   {
   let red = Color(255,0,0);
   println!("{} {} {}",red.0,red.1,red.2);
   
   }
   
 // ** Pass By Reference**
 struct Color{
   red:u8,
   green:u8,
   blue:u8,

 }

fn main() {
let blue= Color{red:0, green:0 ,blue:255 };
bg_color(&blue);
}
fn bg_color(c:&Color){
 println!("background is R:{} B:{} G:{}", c.red, c.blue , c.green); 
}
   
  ** Array**
fn main() {

    let arr_ye =[1,2,3,4,5,6];
    for i in 0..arr_ye.len(){
        println!("Hello, world! {}",arr_ye[i]);
    }
    
}
  ** Impl Keyword**
   Used to add methods to a stuct to make it more useful.
**Implementin Traits in Rust**
   trait is just like interface. trait is basically what an object or a class or an object can do.
   
 struct Person{
    name:String,
   age:u8
   }

   impl ToString for Person{
       fn to_string(&self) -> String{

        return format!("The name is {} and age is {}", self.name ,self.age);
       }

   }

   fn main(){
   let per_det = Person{name:String::from("Asha"), age:21};
   println!("{}", per_det.to_string());
   
   }
   
**   Read a File in RUST**
   use std::fs::File;
use std::io::prelude::*;
fn main() {

    let mut file = File::open("info.txt").expect("The file is not opening");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("OOps!! It dodn't open");

    println!("Hello, world! {}",contents);
}
Output - Hello, world! Ashadipta is a good boy
   
  ** Command Line Arguements in RUST**
   Command-line arguments are given after the name of the program in command-line shell of Operating Systems.Command line arguments are nothing but simply arguments that are specified after the name of the program in the system's command line, and these argument values are passed on to your program during program execution.
  use std::env;
fn main() {

    let args:Vec<String> = env::args().collect(); 

    for arguements in args.iter(){

        println!("Hello, world!{}",arguements);
    }   
   
}
   
   **Writing to a File**
   use std::fs::File;
use std::io::prelude::*;
fn main() {

    let mut file = File::create("output.txt").expect("The file is not opening");

    file.write_all(b"How can I be successful").expect("error");
}
 ** Match Operator in RUST
**
Its like the RUST equivalent of a switch statement.its a conditional operator where u can do different things based on the value of a variable or an expression.
   
   fn main(){
   
   let x= 3;
   match x{
     1=>println!("This is one"),
    2=>println!("This is two"),
    3=>println!("This is three),
   }
   
   }
   
   Output- This is three
   
  ** Hash Maps**
   A collection of key-value pairs
   
   **Concurrency**
   
   In Concurrent programming, different parts of a program execute independently. On the other hand, in parallel programming, different parts of a program execute at the same time. Both the models are equally important as more computers take advantage of their multiple processors.

Threads
We can use threads to run codes simultaneously. In current operating systems, an executed program’s code is run in a process, and the operating system manages multiple processes at once. Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.

Creating a Thread
The thread::spawn function is used to create a new thread. The spawn function takes a closure as parameter. The closure defines code that should be executed by the thread. The following example prints some text from a main thread and other text from a new thread.
   
   The memory for a program can be allocated in the following −

Stack
Heap
Stack
A stack follows a last in first out order. Stack stores data values for which the size is known at compile time. For example, a variable of fixed size i32 is a candidate for stack allocation. Its size is known at compile time. All scalar types can be stored in stack as the size is fixed.

Consider an example of a string, which is assigned a value at runtime. The exact size of such a string cannot be determined at compile time. So it is not a candidate for stack allocation but for heap allocation.

Heap
The heap memory stores data values the size of which is unknown at compile time. It is used to store dynamic data. Simply put, a heap memory is allocated to data values that may change throughout the life cycle of the program. The heap is an area in the memory which is less organized when compared to stack.

Rocket API

import Rocket.
rocket::ignite().mount{"/", router![hello].launch()}
   
  ** Memory Management**
   
   Rust has its own memory management system and doesn't use a garbage collector. Golang,java,python etc use garbage collector.
   
  ** Reverse A String**
   fn main() {
    let name = "Ashadipta";
    let slic= name.chars().rev().collect::<String>();
   
  ** Slice**
   Subpart of a string
    println!("{:?}",slic);
}
