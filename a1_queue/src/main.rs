use a1_queue::Quenue;

fn main() {
    let mut q = Quenue::new(3);
    let _ = q.enqueue(1);
    let _ = q.enqueue(2);
    let _ = q.enqueue(3);

    println!("content: {:?}", q);
}
