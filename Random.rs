
use rand::random;

fn main(){
let mut number  = rand::random::<f64>(); // let mut number:f64  = rand::random();
  println!("{:?}",number);

}
  
  in Cargo.toml insert rand = "0.8.4"

------------------------------------------------------

for a range in Random number

use rand::{thread_rng,Rng};
fn main(){

    let mut number = rand::thread_rng().gen_range(0..11);
    println!("{:?}", number);
}
