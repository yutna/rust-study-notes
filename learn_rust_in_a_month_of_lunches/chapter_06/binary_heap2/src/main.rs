// NOTE: A good way to use a BinaryHeap is for a collection of things to do.
// BinaryHead<(u8, &str)>

// NOTE: งง ในหนังสือบอกว่า มันจะ print ค่ามากออกมาก่อนเสมอ แต่ผลัพธ์ไม่ได้เป็นแบบนั้น

use std::collections::BinaryHeap;

fn main() {
    let mut jobs = BinaryHeap::new();

    jobs.push((100, "Reply to email from the CEO"));
    jobs.push((80, "Finish the report today"));
    jobs.push((5, "Watch some YouTube"));
    jobs.push((70, "Tell your team memebers thanks for always working hard"));
    jobs.push((30, "Plan who to hire next for the team"));

    for (n, job) in jobs {
        println!("You need to: {n} {job}");
    }
}
