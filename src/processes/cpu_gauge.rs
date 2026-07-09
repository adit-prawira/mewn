pub struct CpuGauge;

impl CpuGauge {
    pub fn render(percent: f64) -> String {
        let clamped = percent.clamp(0.0, 100.0);
        let filled = (clamped / 12.5).floor() as usize;
        let filled_string = "█".repeat(filled);
        let empty_string = "░".repeat(8usize.saturating_sub(filled));
        format!("{}{} {:>3.0}%", filled_string, empty_string, clamped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_zero_percent_then_renders_empty_bar() {
        assert_eq!(CpuGauge::render(0.0), "░░░░░░░░   0%");
    }

    #[test]
    fn given_fifty_percent_then_renders_half_bar() {
        assert_eq!(CpuGauge::render(50.0), "████░░░░  50%");
    }

    #[test]
    fn given_hundred_percent_then_renders_full_bar() {
        assert_eq!(CpuGauge::render(100.0), "████████ 100%");
    }

    #[test]
    fn given_negative_percent_then_clamps_to_zero() {
        assert_eq!(CpuGauge::render(-5.0), "░░░░░░░░   0%");
    }

    #[test]
    fn given_above_hundred_then_clamps_to_full() {
        assert_eq!(CpuGauge::render(150.0), "████████ 100%");
    }

    #[test]
    fn given_twelve_and_half_percent_then_one_block() {
        assert_eq!(CpuGauge::render(12.5), "█░░░░░░░  12%");
    }

    #[test]
    fn given_eighty_seven_and_half_percent_then_seven_blocks() {
        assert_eq!(CpuGauge::render(87.5), "███████░  88%");
    }

    #[test]
    fn given_twenty_five_percent_then_two_blocks() {
        assert_eq!(CpuGauge::render(25.0), "██░░░░░░  25%");
    }

    #[test]
    fn given_seventy_five_percent_then_six_blocks() {
        assert_eq!(CpuGauge::render(75.0), "██████░░  75%");
    }
}
