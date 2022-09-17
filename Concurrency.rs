use std::thread;
use std::time::Duration;

fn main (){

  
  thread::spawn(||{
    for i in 1..=10{
    println!("the no. is {:?}",i);
      thread::sleep(Duration::millis(1)
    };
    }
  );
  

}
