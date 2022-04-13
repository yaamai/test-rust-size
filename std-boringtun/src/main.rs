use boringtun::device::{Device, DeviceConfig};

fn main() {
    let config = DeviceConfig {
        n_threads: 1,
        use_connected_socket: true,
        use_multi_queue: true,
        uapi_fd: 0,
    };
    let dev = Device::new("wg1", config).unwrap();
    println!("Hello, world! ");
}
