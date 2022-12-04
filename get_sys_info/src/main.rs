use machine_info::Machine;
use std::{thread, time};
fn main() {
    let mut m = Machine::new();
    // Please use a real PIDs!
    // m.track_process(46246).unwrap();
    // m.track_process(36759).unwrap();

    for _ in 1..10 {
        let processes = m.processes_status();
        let system = m.system_info();
        let graphics = m.graphics_status();
        println!("{:#?} {:#?} {:#?}", processes, system, graphics);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

// SystemInfo {
//     os_name: "Darwin",
//     kernel_version: "21.3.0",
//     os_version: "12.2.1",
//     hostname: "JarttodeiMac.lan",
//     distribution: "macos",
//     memory: 17179869184,
//     processor: Processor {
//         frequency: 0,
//         vendor: "Apple",
//         brand: "Apple M1",
//     },
//     total_processors: 8,
//     graphics: [],
//     disks: [
//         Disk {
//             name: "Macintosh HD",
//             fs: "apfs",
//             storage_type: "SSD",
//             mount_point: "/",
//             available: 494035906756,
//             size: 994662584320,
//         },
//         Disk {
//             name: "Macintosh HD",
//             fs: "apfs",
//             storage_type: "SSD",
//             mount_point: "/System/Volumes/Data",
//             available: 494035906756,
//             size: 994662584320,
//         },
//     ],
//     cameras: [],
//     nvidia: None,
//     vaapi: false,
//     model: None,
// }