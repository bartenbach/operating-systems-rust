use driver::Driver;

pub struct Fifo {}

impl Driver for Fifo {
    fn process_queue(&self, queue: Vec<u32>, queue_size: u32) -> f32 {
        println!("{} {}", queue.len(), queue_size);
        return 40.2;
    }
}
