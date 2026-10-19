/// Coefficient used in the capillary-pressure relation:
///
/// ΔP = 2σ / r
const CAPILLARY_PRESSURE_FACTOR: f64 = 2.0;

/// Wick properties used for capillary pumping.
#[derive(Debug, Clone, Copy)]
pub struct Wick {
    /// Effective pore radius [m].
    pub pore_radius: f64,
}

impl Wick {
    /// Calculates capillary pressure.
    ///
    /// ΔP = 2σ cos(θ) / r
    pub fn capillary_pressure(&self, surface_tension: f64, contact_angle_rad: f64) -> f64 {
        CAPILLARY_PRESSURE_FACTOR * surface_tension * contact_angle_rad.cos() / self.pore_radius
    }

    /// Calculates the maximum capillary pressure.
    pub fn max_capillary_pressure(&self, surface_tension: f64, contact_angle_rad: f64) -> f64 {
        CAPILLARY_PRESSURE_FACTOR * surface_tension * contact_angle_rad.cos() / self.pore_radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capillary_pressure_is_positive() {
        let wick = Wick {
            pore_radius: 1.0e-6,
        };

        let pressure = wick.capillary_pressure(0.023, 0.0);

        assert!(pressure > 0.0);
    }

    #[test]
    fn capillary_pressure_matches_expected_value() {
        let wick = Wick {
            pore_radius: 1.0e-6,
        };

        let pressure = wick.capillary_pressure(0.023, 0.0);

        let expected = 46_000.0;

        assert!((pressure - expected).abs() < 1e-6);
    }

    #[test]
    fn max_capillary_pressure_matches_expected_value() {
        let wick = Wick {
            pore_radius: 1.0e-6,
        };

        let pressure = wick.max_capillary_pressure(0.023, 0.0);

        let expected = 46_000.0;

        assert!((pressure - expected).abs() < 1e-6);
    }

    #[test]
    fn smaller_pore_produces_higher_pressure() {
        let small_wick = Wick {
            pore_radius: 1.0e-6,
        };

        let large_wick = Wick {
            pore_radius: 2.0e-6,
        };

        let small_pressure = small_wick.max_capillary_pressure(0.023, 0.0);

        let large_pressure = large_wick.max_capillary_pressure(0.023, 0.0);

        assert!(small_pressure > large_pressure);
    }

    #[test]
    fn ninety_degree_contact_angle_gives_zero_pressure() {
        let wick = Wick {
            pore_radius: 1.0e-6,
        };

        let pressure = wick.capillary_pressure(0.023, std::f64::consts::FRAC_PI_2);

        assert!(pressure.abs() < 1e-10);
    }
}
