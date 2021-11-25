use driver::Driver;
use std::collections::VecDeque;

pub struct Cscan {}

impl Driver for Cscan {
    fn process_queue(&self, mut queue: Vec<u32>, buf_size: u32) -> f32 {
        let mut total_time: f32 = 0.0;
        let queue_length = queue.len() as f32;

        // FIXME
        // technically don't check to see if we even have buf_size elements
        let mut buf = VecDeque::with_capacity(buf_size as usize);
        for x in 0..buf_size {
            buf.push_back(queue[x as usize]);
        }
        let mut current_cylinder: u32 = 0;
        while !buf.len() > 1 {
            if !queue.is_empty() {
                buf.push_back(queue.pop().unwrap());
                buf.make_contiguous().sort();
            } else {
                break;
            }
            if current_cylinder > buf[buf.len() - 1] {
                total_time += crate::calc_seek_time(current_cylinder - buf[0]);
                current_cylinder = buf[0];
                buf.remove(0);
            } else {
                for i in 0..(buf.len() - 1) {
                    if buf[i] == current_cylinder {
                        total_time += crate::LATENCY_MS;
                        buf.remove(i);
                        break;
                    } else if buf[i] > current_cylinder {
                        total_time += crate::calc_seek_time(buf[i] - current_cylinder);
                        current_cylinder = buf[i];
                        buf.remove(i);
                        break;
                    }
                }
            }
        }
        return total_time / queue_length;
    }
}
