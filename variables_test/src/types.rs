fn compound_types() {
    let tup = (1,1.5,'a');

    let (x,y,z) = tup; //assigns variables to tuple elements

    let a = tup.0;

    println!("{}",a);

    //arrays

    let a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = [1, 2, 3, 4, 5]; //first brackets are [type,size]

    let a = [3; 5]; //brackets are [initial value, size]
}

fn main() {
    compound_types();
}