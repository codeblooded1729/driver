use fpga::{Fpga, SendBuffer, F1};

fn encode(a: u64, b: u64, twiddle: u64) -> SendBuffer {
    let mut buffer = SendBuffer::default();
    buffer[..8].copy_from_slice(&a.to_le_bytes());
    buffer[8..16].copy_from_slice(&b.to_le_bytes());
    buffer[16..24].copy_from_slice(&twiddle.to_le_bytes());
    buffer
}

fn main() {
    let mut fpga = F1::new(0, 0).unwrap();
    println!("init");

    let index = 0;
    let send_buffer = encode(456, 123, 789);
    fpga.send(index, &send_buffer);
    println!("wrote");
    println!("{}", fpga);
}
