use vfp8_driver::Vfp8Accelerator;

fn main() {
    let _device = Vfp8Accelerator::take().unwrap();
}