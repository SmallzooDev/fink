use std::time::{Duration, Instant};

/// A spinning indicator that cycles through frames
pub struct SpinnerIndicator {
    frames: Vec<&'static str>,
    current_frame: usize,
    last_update: Instant,
    update_interval: Duration,
}

impl SpinnerIndicator {
    /// Create a new spinner with default ASCII frames
    pub fn new() -> Self {
        Self {
            frames: vec!["|", "/", "-", "\\"],
            current_frame: 0,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(100),
        }
    }
    
    /// Create a spinner with custom frames
    pub fn with_frames(frames: Vec<&'static str>) -> Self {
        Self {
            frames,
            current_frame: 0,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(100),
        }
    }
    
    /// Get the current frame and advance if enough time has passed
    pub fn tick(&mut self) -> &str {
        if self.last_update.elapsed() >= self.update_interval {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.last_update = Instant::now();
        }
        self.frames[self.current_frame]
    }
    
    /// Get the current frame without advancing
    pub fn current(&self) -> &str {
        self.frames[self.current_frame]
    }
}

/// A sparkling/twinkling indicator that cycles through sparkle effects
pub struct SparkleIndicator {
    frames: Vec<&'static str>,
    current_frame: usize,
    last_update: Instant,
    update_interval: Duration,
}

impl SparkleIndicator {
    /// Create a new sparkle indicator with ASCII-compatible frames
    pub fn new() -> Self {
        Self {
            frames: vec!["*", "+", "o", "+"],
            current_frame: 0,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(200),
        }
    }
    
    /// Get the current frame and advance if enough time has passed
    pub fn tick(&mut self) -> &str {
        if self.last_update.elapsed() >= self.update_interval {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.last_update = Instant::now();
        }
        self.frames[self.current_frame]
    }
}

/// Loading dots indicator
pub struct LoadingDots {
    dots: Vec<&'static str>,
    current_frame: usize,
    last_update: Instant,
    update_interval: Duration,
}

impl LoadingDots {
    pub fn new() -> Self {
        Self {
            dots: vec!["   ", ".  ", ".. ", "..."],
            current_frame: 0,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(300),
        }
    }
    
    pub fn tick(&mut self) -> &str {
        if self.last_update.elapsed() >= self.update_interval {
            self.current_frame = (self.current_frame + 1) % self.dots.len();
            self.last_update = Instant::now();
        }
        self.dots[self.current_frame]
    }
}

/// Progress indicator with moving arrow
pub struct ProgressArrow {
    frames: Vec<&'static str>,
    current_frame: usize,
    last_update: Instant,
    update_interval: Duration,
}

impl ProgressArrow {
    pub fn new() -> Self {
        Self {
            frames: vec![
                "[>    ]",
                "[=>   ]",
                "[==>  ]",
                "[===> ]",
                "[====>]",
                "[ <===]",
                "[  <==]",
                "[   <=]",
                "[    <]",
            ],
            current_frame: 0,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(150),
        }
    }
    
    pub fn tick(&mut self) -> &str {
        if self.last_update.elapsed() >= self.update_interval {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.last_update = Instant::now();
        }
        self.frames[self.current_frame]
    }
}