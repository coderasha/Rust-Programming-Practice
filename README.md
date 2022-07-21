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
its a module in rust that provides the container space to store values.In simple terms its a container that stores values like an array 
