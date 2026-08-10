#[derive(Debug)]
struct X<'a>(&'a i32);

impl Drop for X<'_> {
    fn drop(&mut self) {}
}

fn lifetime_drop() {
    let mut data = vec![1, 2, 3];
    let x = X(&data[0]);
    println!("{:?}", x);
    drop(x); // 手动实现的析构 drop，默认会在函数结束时执行，不提前 drop， 将不可再出现可变引用
    data.push(4);
}