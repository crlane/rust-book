fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        println!("i = {:?}, P = {:?}", i, padovan);
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}

fn main() {
    print_padovan()
}
