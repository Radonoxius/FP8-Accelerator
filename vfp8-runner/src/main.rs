use vfp8_driver::Vfp8Accelerator;

fn main() {
    let mut device = Vfp8Accelerator::take().unwrap();

    let mut dummy = [1; 16];
    dummy[0] = 5;

    let _ = device.write_reg_at(0, dummy);
    let store = device.read_reg_at(0);

    println!("Sent {:?}", dummy);
    println!("Got {:?}!", store.unwrap());
}