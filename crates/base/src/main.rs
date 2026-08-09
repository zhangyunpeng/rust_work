mod pin;

use pin::SelfRef;

fn main() {
    let mut sr1 = SelfRef::new("test1");
    let mut sr2 = SelfRef::new("test2");

    println!("a: {}, b: {}", sr1.as_ref().a(), sr1.as_ref().b());
    println!("a: {}, b: {}", sr2.as_ref().a(), sr2.as_ref().b());
    std::mem::swap(&mut sr1, &mut sr2);
    println!("a: {}, b: {}", sr1.as_ref().a(), sr1.as_ref().b());
    println!("a: {}, b: {}", sr2.as_ref().a(), sr2.as_ref().b());
}
