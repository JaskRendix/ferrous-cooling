/// Coefficient used in the capillary-pressure relation:
///
/// ΔP = 2σ / r
const CAPILLARY_PRESSURE_FACTOR: f64 = 2.0;

/// Wick properties used for capillary pumping.
#[derive(Debug, Clone, Copy)]
pub struct Wick {
    /// Effective pore radius [m].
    pub pore_radius: f64,

    pub permeability: f64,

    pub porosity: f64,

    pub thermal_conductivity: f64,

    pub thickness: f64,
}

impl Wick {
    /// Typical ammonia LHP primary wick properties.
    ///
    /// Values are representative engineering estimates
    /// suitable for preliminary calculations.
    pub fn typical_ammonia_wick() -> Self {
        Self {
            pore_radius: 1.0e-6,
            permeability: 1.0e-13,
            porosity: 0.6,
            thermal_conductivity: 10.0,
            thickness: 1.0e-3,
        }
    }
    /// Calculates capillary pressure.
    ///
    /// ΔP = 2σ cos(θ) / r
    pub fn capillary_pressure(&self, surface_tension: f64, contact_angle_rad: f64) -> f64 {
        CAPILLARY_PRESSURE_FACTOR * surface_tension * contact_angle_rad.cos() / self.pore_radius
    }

    /// Calculates the maximum capillary pressure.
    pub fn max_capillary_pressure(&self, surface_tension: f64) -> f64 {
        CAPILLARY_PRESSURE_FACTOR * surface_tension / self.pore_radius
    }

    /// Calculates pressure drop through the wick using
    /// Darcy's law.
    ///
    /// ΔP = μ L ṁ / (K ρ A)
    ///
    /// where:
    /// - μ = dynamic viscosity [Pa·s]
    /// - L = wick thickness [m]
    /// - ṁ = mass flow rate [kg/s]
    /// - K = permeability [m²]
    /// - ρ = liquid density [kg/m³]
    /// - A = flow area [m²]
    pub fn wick_pressure_drop(
        &self,
        viscosity: f64,
        mass_flow_rate: f64,
        density: f64,
        area: f64,
    ) -> f64 {
        viscosity * self.thickness * mass_flow_rate / (self.permeability * density * area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capillary_pressure_is_positive() {
        let wick = Wick::typical_ammonia_wick();

        let pressure = wick.capillary_pressure(0.023, 0.0);

        assert!(pressure > 0.0);
    }

    #[test]
    fn capillary_pressure_matches_expected_value() {
        let wick = Wick::typical_ammonia_wick();

        let pressure = wick.capillary_pressure(0.023, 0.0);

        let expected = 46_000.0;

        assert!((pressure - expected).abs() < 1e-6);
    }

    #[test]
    fn max_capillary_pressure_matches_expected_value() {
        let wick = Wick::typical_ammonia_wick();

        let pressure = wick.max_capillary_pressure(0.023);

        let expected = 46_000.0;

        assert!((pressure - expected).abs() < 1e-6);
    }

    #[test]
    fn smaller_pore_produces_higher_pressure() {
        let small_wick = Wick {
            pore_radius: 1.0e-6,
            ..Wick::typical_ammonia_wick()
        };

        let large_wick = Wick {
            pore_radius: 2.0e-6,
            ..Wick::typical_ammonia_wick()
        };

        let small_pressure = small_wick.max_capillary_pressure(0.023);

        let large_pressure = large_wick.max_capillary_pressure(0.023);

        assert!(small_pressure > large_pressure);
    }

    #[test]
    fn ninety_degree_contact_angle_gives_zero_pressure() {
        let wick = Wick::typical_ammonia_wick();

        let pressure = wick.capillary_pressure(0.023, std::f64::consts::FRAC_PI_2);

        assert!(pressure.abs() < 1e-10);
    }

    #[test]
    fn wick_pressure_drop_is_positive() {
        let wick = Wick::typical_ammonia_wick();

        let dp = wick.wick_pressure_drop(2.5e-4, 0.001, 682.0, 1.0e-4);

        assert!(dp > 0.0);
    }

    #[test]
    fn lower_permeability_increases_pressure_drop() {
        let low_k = Wick {
            permeability: 1.0e-14,
            ..Wick::typical_ammonia_wick()
        };

        let high_k = Wick {
            permeability: 1.0e-12,
            ..Wick::typical_ammonia_wick()
        };

        let low = low_k.wick_pressure_drop(2.5e-4, 0.001, 682.0, 1.0e-4);

        let high = high_k.wick_pressure_drop(2.5e-4, 0.001, 682.0, 1.0e-4);

        assert!(low > high);
    }

    #[test]
    fn thicker_wick_increases_pressure_drop() {
        let thin = Wick {
            thickness: 1.0e-3,
            ..Wick::typical_ammonia_wick()
        };

        let thick = Wick {
            thickness: 2.0e-3,
            ..Wick::typical_ammonia_wick()
        };

        let dp_thin = thin.wick_pressure_drop(2.5e-4, 0.001, 682.0, 1.0e-4);

        let dp_thick = thick.wick_pressure_drop(2.5e-4, 0.001, 682.0, 1.0e-4);

        assert!(dp_thick > dp_thin);
    }

    #[test]
    fn higher_mass_flow_increases_pressure_drop() {
        let wick = Wick::typical_ammonia_wick();

        let low = wick.wick_pressure_drop(2.5e-4, 0.001, 682.0, 1.0e-4);

        let high = wick.wick_pressure_drop(2.5e-4, 0.002, 682.0, 1.0e-4);

        assert!(high > low);
    }

    #[test]
    fn higher_density_reduces_pressure_drop() {
        let wick = Wick::typical_ammonia_wick();

        let low_density = wick.wick_pressure_drop(2.5e-4, 0.001, 500.0, 1.0e-4);

        let high_density = wick.wick_pressure_drop(2.5e-4, 0.001, 1000.0, 1.0e-4);

        assert!(low_density > high_density);
    }
}
