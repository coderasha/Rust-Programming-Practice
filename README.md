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
