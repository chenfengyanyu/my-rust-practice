// https://docs.rs/sys-info/latest/sys_info/
extern crate sys_info;
fn main() {
    let cpus = sys_info::cpu_num().unwrap(); // 8
    let loadavgs = sys_info::loadavg().unwrap(); // LoadAvg { one: 3.20751953125, five: 4.00244140625, fifteen: 3.06640625 }
    let procs = sys_info::proc_total().unwrap(); // 获取当前进程数量: 418
    let ostype = sys_info::os_type().unwrap(); // "Darwin"
    let mems = sys_info::mem_info().unwrap(); // MemInfo { total: 16777216, free: 90336, avail: 3605552, buffers: 0, cached: 0, swap_total: 2097152, swap_free: 1575936 }
    // let times = sys_info::boottime().unwrap();

    println!(
        "{:?}, {:?}, {:?}, {:?},{:?}",
        cpus, loadavgs, procs, ostype, mems
    );
}
