//Single module
mod movie{
  pub fn mov_name(){
     println!("RRR");
      }
}
fn main(){
movie::mov_name();

}

//Sub Module
mod movie{
  pub fn mov_name(){
     println!("RRR");
      }
  pub fn mov_name1(){
     println!("Bramhastra");
      }
}
fn main(){
movie::mov_name();
  movie::mov_name1();

}



