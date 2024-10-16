fn main() {

    // print
    println!("Hello, world!");

    // work on variable
    let x: f32 = 32.0;
    println!("x is {0}", x);

    // work on add
    let a: i32 = 32;
    let b: i32 = 32;
    let c = a + b;
    println!("{0} + {1} = {2}", a, b, c);

    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    // All elements can be initialized to the same value.
    let ys: [i32; 5] = [0; 5];

    // Indexing starts at 0.
    println!("xs is {:?}", xs);
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);
    println!("ys is {:?}", ys);

    // take value from array
    let new_arrays: &[i32] = &xs[1..4];
    println!("new_arrays is {:?}", new_arrays);

}
