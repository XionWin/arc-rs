#[allow(dead_code)]
pub struct FpsLimiter {
    pub fps: u32,
    _duration: std::time::Duration,
    _last_update_time: std::time::SystemTime,
}

impl FpsLimiter {
    pub fn new(fps: u32) -> Self {
        FpsLimiter {
            fps,
            _duration: std::time::Duration::from_secs(1) / fps - std::time::Duration::from_micros(350),
            _last_update_time: std::time::SystemTime::now(),
        }
    }

    pub fn delay(&mut self) {
        let now_time = std::time::SystemTime::now();
        match now_time.duration_since(self._last_update_time) {
            Ok(used_duration) => {
                if self._duration > used_duration {
                    let duration = self._duration - used_duration;
                    std::thread::sleep(duration)
                }
            },
            Err(msg) => println!("FpsLimiter delay error, Msg: {:?}", msg.duration()),
        }
        self.reset();
    }

    fn reset(&mut self) {
        self._last_update_time = std::time::SystemTime::now();
    }
}
