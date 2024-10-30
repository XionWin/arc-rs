pub struct FpsLimiter {
    _fps: u32,
    _duration: std::time::Duration,
    _last_update_time: std::time::SystemTime,
}

impl FpsLimiter {
    pub fn new(fps: u32) -> Self {
        FpsLimiter {
            _fps: fps,
            _duration: std::time::Duration::from_secs(1) / fps,
            _last_update_time: std::time::SystemTime::now(),
        }
    }

    pub fn delay(&mut self) {
        let now_time = std::time::SystemTime::now();
        let duration = now_time
            .duration_since(self._last_update_time)
            .expect("ManualVSync delay error.");
        let wait_duration = if self._duration > duration {
            Some(self._duration - duration)
        } else {
            Option::None
        };

        match wait_duration {
            Some(dur) => std::thread::sleep(dur),
            None => {}
        }
        self.reset();
    }

    fn reset(&mut self) {
        self._last_update_time = std::time::SystemTime::now();
    }
}
