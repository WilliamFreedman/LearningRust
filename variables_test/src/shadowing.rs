fn doesnt_compile() {  //assigning to a variable not marked at mut fails
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn this_one_does() { //mut makes mutable, who would have thunk
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing() { //you can make an immutable variable then redeclare it, called shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    let x = String::new(); //shadowing can change variable type. this seems dangerous if the change is conditional

    println!("The value of x is: {x}");
}
