use crate::graphics_presets::Preset;
use crate::settings::Settings;
use deeply_render::Renderer;
use std::time::{Duration, Instant};

const SAMPLE_PERIOD: Duration = Duration::from_secs(2);
const COOLDOWN: Duration = Duration::from_secs(20);
const REQUIRED_SAMPLES: u8 = 3;

#[derive(Debug)]
pub(super) struct AdaptiveGraphics {
    sampled_at: Instant,
    last_change: Option<Instant>,
    pressure_samples: u8,
}

impl Default for AdaptiveGraphics {
    fn default() -> Self {
        Self {
            sampled_at: Instant::now(),
            last_change: None,
            pressure_samples: 0,
        }
    }
}

impl AdaptiveGraphics {
    pub(super) fn tick(&mut self, settings: &Settings, renderer: &Renderer, frame_ms: f32) -> Option<Preset> {
        let now = Instant::now();
        if now.duration_since(self.sampled_at) < SAMPLE_PERIOD {
            return None;
        }
        self.sampled_at = now;

        let memory_pressure = crate::sys_probe::process_memory().is_some_and(|m| m.private >= 1_500 * 1024 * 1024);
        let cpu_pressure = frame_ms >= 55.0;
        let gpu_pressure = renderer.stalled_for().is_some_and(|d| d >= Duration::from_millis(100));
        if memory_pressure || cpu_pressure || gpu_pressure {
            self.pressure_samples = self.pressure_samples.saturating_add(1);
        } else {
            self.pressure_samples = self.pressure_samples.saturating_sub(1);
        }

        let Some(current) = Preset::matching(settings) else {
            self.pressure_samples = 0;
            return None;
        };
        if current == Preset::Potato
            || self.pressure_samples < REQUIRED_SAMPLES
            || self.last_change.is_some_and(|at| now.duration_since(at) < COOLDOWN)
        {
            return None;
        }

        self.pressure_samples = 0;
        self.last_change = Some(now);
        Some(current.lower())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_without_pressure_samples() {
        let monitor = AdaptiveGraphics::default();
        assert_eq!(monitor.pressure_samples, 0);
    }
}
