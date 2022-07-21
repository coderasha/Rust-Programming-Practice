pub fn run(){

    println!("I am a good boy ");
    println!("Number= {} ",1);
    println!("{} and {} are good boys ","Ram","Shyam");
    //Positional Arguements
    println!("Among {0} and {1} , {0} is a better boy ","Ram","Shyam");

    println!(
        "Among {name} and {activity}  ",name="Ram",activity = "Play"
    );

    println!(
        "Binary: {:b} and Hex: {:x} Octal: {:o}",10,10,10 
    );

    
     println!(
    "{:?}",(12 ,true ,"hello") 
);



}
