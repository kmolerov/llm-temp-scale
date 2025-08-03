#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}

impl Range {
    /// Maps value from self to the target range
    /// mapped = target_min + ((value - base_min) / (base_max - base_min)) * (target_max - target_min)
    pub fn map_to(&self, target: Range, value: f32) -> f32 {
        let clamped = value.clamp(self.min, self.max);
        let ratio = if (self.max - self.min).abs() < f32::EPSILON {
            0.0
        } else {
            (clamped - self.min) / (self.max - self.min)
        };
        target.min + ratio * (target.max - target.min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_mapping() {
        let range = Range { min: 0.0, max: 1.0 };
        assert_eq!(range.map_to(range, 0.0), 0.0);
        assert_eq!(range.map_to(range, 0.5), 0.5);
        assert_eq!(range.map_to(range, 1.0), 1.0);
    }

    #[test]
    fn test_simple_mapping() {
        let base = Range { min: 0.0, max: 1.0 };
        let target = Range { min: 0.0, max: 2.0 };
        assert_eq!(base.map_to(target, 0.0), 0.0);
        assert_eq!(base.map_to(target, 0.5), 1.0);
        assert_eq!(base.map_to(target, 1.0), 2.0);
    }

    #[test]
    fn test_clamping_out_of_bounds_low() {
        let base = Range { min: 0.0, max: 1.0 };
        let target = Range { min: 0.0, max: 10.0 };
        let mapped = base.map_to(target, -1.0);
        assert_eq!(mapped, 0.0); // should clamp to base.min
    }

    #[test]
    fn test_clamping_out_of_bounds_high() {
        let base = Range { min: 0.0, max: 1.0 };
        let target = Range { min: 0.0, max: 10.0 };
        let mapped = base.map_to(target, 2.0);
        assert_eq!(mapped, 10.0); // should clamp to base.max
    }

    #[test]
    fn test_nonzero_base_min() {
        let base = Range { min: 0.3, max: 1.0 };
        let target = Range { min: 0.0, max: 2.0 };
        let mapped = base.map_to(target, 0.3);
        assert!((mapped - 0.0).abs() < 1e-6);

        let mapped = base.map_to(target, 1.0);
        assert!((mapped - 2.0).abs() < 1e-6);

        let mapped = base.map_to(target, 0.65); // halfway
        assert!((mapped - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_zero_range_base() {
        let base = Range { min: 0.5, max: 0.5 };
        let target = Range { min: 0.0, max: 1.0 };
        let mapped = base.map_to(target, 0.5);
        assert_eq!(mapped, 0.0); // defined fallback to target.min
    }
}
