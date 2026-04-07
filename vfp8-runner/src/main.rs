use vfp8_driver::Vfp8Accelerator;

fn main() {
    let device = Vfp8Accelerator::take().unwrap();
}