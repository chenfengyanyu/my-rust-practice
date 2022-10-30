fn main() {
    let a = [1, 2, 3];

    let mut iter = a.iter();

    // A call to next() returns the next value...
    println!("iter1 value: {:?}", iter.next());
    println!("iter2 value: {:?}", iter.next());
    println!("iter3 value: {:?}", iter.next());

    // ... and then None once it's over.
    println!("Over value: {:?}", iter.next());
    // assert_eq!(None, iter.next());

    // More calls may or may not return `None`. Here, they always will.
    // assert_eq!(None, iter.next());
    println!("Over value: {:?}", iter.next());
}
