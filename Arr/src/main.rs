struct Person{
    name:String,
   age:u8
   }

   impl ToString for Person{
       fn to_string(&self) -> String{

        return format!("The name is {} and age is {}", self.name ,self.age);
       }

   }
//
fn main(){
   let per_det = Person{name:String::from("Asha"), age:21};
   println!("{}", per_det.to_string());
   
   }

   
