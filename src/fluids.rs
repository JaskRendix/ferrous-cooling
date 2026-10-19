/// Approximate surface tension of ammonia at room temperature [N/m].
pub const AMMONIA_SURFACE_TENSION: f64 = 0.023;

/// Thermophysical properties of a working fluid.
///
/// Units:
/// - surface_tension: N/m
#[derive(Debug, Clone, Copy)]
pub struct Fluid {
    pub name: &'static str,
    pub surface_tension: f64,

    pub latent_heat: f64,
    pub liquid_density: f64,
    pub vapor_density: f64,
    pub liquid_specific_heat: f64,
}

impl Fluid {
    /// Creates an ammonia working fluid.
    ///
    /// The surface tension value is a representative
    /// room-temperature approximation suitable for
    /// initial LHP calculations.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ferrous_cooling::fluids::Fluid;
    ///
    /// let fluid = Fluid::ammonia();
    ///
    /// assert_eq!(fluid.name, "Ammonia");
    /// ```
    pub fn ammonia() -> Self {
        Self {
            name: "Ammonia",
            surface_tension: AMMONIA_SURFACE_TENSION,
            latent_heat: 1_370_000.0,
            liquid_density: 682.0,
            vapor_density: 7.9,
            liquid_specific_heat: 4700.0,
        }
    }

    /// Saturation-line slope (dP/dT).
    ///
    /// Equation (7) from Ku (1999).
    /// # Example
    ///
    /// ```rust
    /// use ferrous_cooling::fluids::Fluid;
    ///
    /// let ammonia = Fluid::ammonia();
    ///
    /// let slope =
    ///     ammonia.saturation_pressure_slope(
    ///         300.0,
    ///     );
    ///
    /// assert!(slope > 0.0);
    /// ```
    pub fn saturation_pressure_slope(&self, temperature: f64) -> f64 {
        let liquid_specific_volume = 1.0 / self.liquid_density;

        let vapor_specific_volume = 1.0 / self.vapor_density;

        let delta_specific_volume = vapor_specific_volume - liquid_specific_volume;

        self.latent_heat / (temperature * delta_specific_volume)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ammonia_has_expected_name() {
        let fluid = Fluid::ammonia();

        assert_eq!(fluid.name, "Ammonia");
    }

    #[test]
    fn ammonia_surface_tension_is_positive() {
        let fluid = Fluid::ammonia();

        assert!(fluid.surface_tension > 0.0);
    }

    #[test]
    fn ammonia_latent_heat_is_positive() {
        let fluid = Fluid::ammonia();

        assert!(fluid.latent_heat > 0.0);
    }

    #[test]
    fn ammonia_densities_are_positive() {
        let fluid = Fluid::ammonia();

        assert!(fluid.liquid_density > 0.0);
        assert!(fluid.vapor_density > 0.0);
    }

    #[test]
    fn liquid_density_exceeds_vapor_density() {
        let fluid = Fluid::ammonia();

        assert!(fluid.liquid_density > fluid.vapor_density);
    }

    #[test]
    fn ammonia_specific_heat_is_positive() {
        let fluid = Fluid::ammonia();

        assert!(fluid.liquid_specific_heat > 0.0);
    }

    #[test]
    fn saturation_pressure_slope_is_positive() {
        let fluid = Fluid::ammonia();

        let slope = fluid.saturation_pressure_slope(300.0);

        assert!(slope > 0.0);
    }

    #[test]
    fn saturation_pressure_slope_matches_expected_value() {
        let fluid = Fluid::ammonia();

        let slope = fluid.saturation_pressure_slope(300.0);

        let expected =
            fluid.latent_heat / (300.0 * (1.0 / fluid.vapor_density - 1.0 / fluid.liquid_density));

        assert!((slope - expected).abs() < 1e-6);
    }
}
