use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct ProgressReporter {
    enabled: bool,
    bar: Option<ProgressBar>,
}

impl ProgressReporter {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            bar: None,
        }
    }

    pub fn start_scan(&mut self, total_files: usize) {
        if !self.enabled {
            return;
        }

        let bar = ProgressBar::new(total_files as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        bar.enable_steady_tick(Duration::from_millis(100));
        
        self.bar = Some(bar);
    }

    pub fn update(&self, message: &str) {
        if let Some(bar) = &self.bar {
            bar.set_message(message.to_string());
            bar.inc(1);
        }
    }

    pub fn finish(&self, message: &str) {
        if let Some(bar) = &self.bar {
            bar.finish_with_message(message.to_string());
        }
    }
}