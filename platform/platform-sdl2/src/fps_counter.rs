pub struct FpsCounter {
    start_time: std::time::SystemTime,
    counter: u32,
    update_duration: std::time::Duration,
}

impl FpsCounter {
    pub fn new(update_duration: std::time::Duration) -> Self {
        FpsCounter {
            start_time: std::time::SystemTime::now(),
            counter: 0,
            update_duration,
        }
    }

    pub fn update<T>(&mut self, mut callback: T)
        where T: FnMut(f32) {
        let now_time = std::time::SystemTime::now();
        if (now_time.duration_since(self.start_time)).unwrap() >= self.update_duration {
            match now_time.duration_since(self.start_time) {
                Ok(duration) => {
                    let fps = self.counter as f32 / duration.as_secs_f32();
                    callback(fps);
                    self.reset();
                }
                Err(msg) => panic!("fps_counter update error, Msg: {:?}", msg.duration()),
            }
        } else {
            self.counter += 1;
        }
    }

    fn reset(&mut self) {
        self.start_time = std::time::SystemTime::now();
        self.counter = 0;
    }
}
