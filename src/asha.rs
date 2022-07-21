struct Foo{


    foo:fn(),

}

impl Foo{

    fn foo(&self){

        printf!("A");

    }
}

fn main(){

    let foo2 = || print!("B") ;
    Foo{ foo: foo2 }. foo();
}