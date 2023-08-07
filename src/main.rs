fn main() {
    // Reference
    let x = 5;
    let y = &x;
    println!("Simple Ref: {}", y);
    // Immutable Ref
    let w = 6;
    let q = &w;
    println!("Simple Immutable Ref: {}", q);

    // Iterators

    let makis: Vec<_> = vec![1, 2, 3]
        .iter() // Create the Iterator to go over the elements in the array
        .map(|x| x * 3) // Map through and multiply by 3
        .collect(); // Take the Iterator you produced and put it somewhere that you can use

    println!("Makis: {:?}", makis);

    // Complex  way the collect() works
    let data = vec![1, 2, 3];
    let mut takis = data.iter().map(|x| x + 1);

    let mut new_takis_vector = vec![];

    while let Some(x) = takis.next() {
        new_takis_vector.push(x);
    }

    println!("Takis: {:?}", new_takis_vector);
}
