use std::thread;
use std::time::Duration;

fn main (){

  
  thread::spawn(||{
    for i in 1..=10{
    println!("the no. is {:?}",i);
      thread::sleep(Duration::millis(1))
    };
    }
  );
  

}


// Using Join Handle

use std::thread;
use std::time::Duration;
fn main (){

  let handle =  thread::spawn(||{

    for mut i in 0..=10{
        println!("{} for thread",i  );
        thread::sleep(Duration::from_millis(5))
    };
});
    for mut j in 0..=6{
        println!("{} is printed for main",j  );
        thread::sleep(Duration::from_millis(5))

    };

    handle.join().unwrap();

    
   
}
