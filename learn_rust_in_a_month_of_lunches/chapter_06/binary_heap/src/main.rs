// NOTE: A BinaryHeap is an interesting collection type because it is mostly
// underored but has a bit of order. It keeps the item with the greatest value
// in the front, but the other items are in any order. Some languages call this
// a priority queue.

use std::collections::BinaryHeap;

fn main() {
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];
    let mut heap = BinaryHeap::new();

    for num in many_numbers {
        heap.push(num);
    }

    println!("First item is largest, others are out of order: {heap:?}");

    while let Some(num) = heap.pop() {
        println!("Popped off {num}. Remaining numbers are: {heap:?}");
    }
}
