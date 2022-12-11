use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // 获取切片长度
    let len = slice.len();
    // 使用 as_mut_ptr 方法来访问切片包含的裸指针
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        // 函数接收一个裸指针和长度来创建一个切片
        (slice::from_raw_parts_mut(ptr, len),
        // 创建另一个起始于 mid 处且拥有剩余所有元素的切片
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
fn main() {
    let mut arr = vec![1,2,3,4,5,6];
    println!("{:?}",split_at_mut(&mut arr, 4));
}
