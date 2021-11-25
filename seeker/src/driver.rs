pub trait Driver {
    fn process_queue(&self, data: Vec<u32>, queue_size: u32) -> f32;
}
