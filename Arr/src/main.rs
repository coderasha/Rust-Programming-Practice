struct Person{
    name:String,
   age:u8
   }

   impl ToString for Person{
       fn to_string(&self) -> String{

        return format!("The name is {} and age is {}", self.name ,self.age);
       }

   }

   
