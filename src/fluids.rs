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

    pub fn water() -> Self {
        Self {
            name: "Water",
            surface_tension: 0.072,
            latent_heat: 2_257_000.0,
            liquid_density: 997.0,
            vapor_density: 0.026,
            liquid_specific_heat: 4184.0,
        }
    }

    pub fn methanol() -> Self {
        Self {
            name: "Methanol",
            surface_tension: 0.022,
            latent_heat: 1_100_000.0,
            liquid_density: 792.0,
            vapor_density: 1.17,
            liquid_specific_heat: 2510.0,
        }
    }

    pub fn acetone() -> Self {
        Self {
            name: "Acetone",
            surface_tension: 0.023,
            latent_heat: 518_000.0,
            liquid_density: 784.0,
            vapor_density: 2.4,
            liquid_specific_heat: 2160.0,
        }
    }

    pub fn propane() -> Self {
        Self {
            name: "Propane",
            surface_tension: 0.008,
            latent_heat: 356_000.0,
            liquid_density: 493.0,
            vapor_density: 1.9,
            liquid_specific_heat: 2500.0,
        }
    }

    pub fn propylene() -> Self {
        Self {
            name: "Propylene",
            surface_tension: 0.007,
            latent_heat: 330_000.0,
            liquid_density: 520.0,
            vapor_density: 2.0,
            liquid_specific_heat: 2400.0,
        }
    }

    pub fn r134a() -> Self {
        Self {
            name: "R134a",
            surface_tension: 0.008,
            latent_heat: 217_000.0,
            liquid_density: 1207.0,
            vapor_density: 5.25,
            liquid_specific_heat: 1420.0,
        }
    }

    pub fn r245fa() -> Self {
        Self {
            name: "R245fa",
            surface_tension: 0.013,
            latent_heat: 190_000.0,
            liquid_density: 1330.0,
            vapor_density: 8.0,
            liquid_specific_heat: 1320.0,
        }
    }

    pub fn co2() -> Self {
        Self {
            name: "CO2",
            surface_tension: 0.003,
            latent_heat: 200_000.0,
            liquid_density: 770.0,
            vapor_density: 65.0,
            liquid_specific_heat: 2200.0,
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
    fn ammonia_slope_is_reasonable() {
        let fluid = Fluid::ammonia();

        let slope = fluid.saturation_pressure_slope(300.0);

        assert!(slope > 30_000.0);
        assert!(slope < 40_000.0);
    }

    fn assert_fluid_is_valid(fluid: Fluid) {
        assert!(!fluid.name.is_empty());

        assert!(fluid.surface_tension > 0.0);
        assert!(fluid.latent_heat > 0.0);

        assert!(fluid.liquid_density > 0.0);
        assert!(fluid.vapor_density > 0.0);

        assert!(fluid.liquid_density > fluid.vapor_density);

        assert!(fluid.liquid_specific_heat > 0.0);

        let slope = fluid.saturation_pressure_slope(300.0);

        assert!(slope > 0.0);
        assert!(slope.is_finite());
    }

    #[test]
    fn ammonia_properties_are_valid() {
        assert_fluid_is_valid(Fluid::ammonia());
    }

    #[test]
    fn water_properties_are_valid() {
        assert_fluid_is_valid(Fluid::water());
    }

    #[test]
    fn methanol_properties_are_valid() {
        assert_fluid_is_valid(Fluid::methanol());
    }

    #[test]
    fn acetone_properties_are_valid() {
        assert_fluid_is_valid(Fluid::acetone());
    }

    #[test]
    fn propane_properties_are_valid() {
        assert_fluid_is_valid(Fluid::propane());
    }

    #[test]
    fn propylene_properties_are_valid() {
        assert_fluid_is_valid(Fluid::propylene());
    }

    #[test]
    fn r134a_properties_are_valid() {
        assert_fluid_is_valid(Fluid::r134a());
    }

    #[test]
    fn r245fa_properties_are_valid() {
        assert_fluid_is_valid(Fluid::r245fa());
    }

    #[test]
    fn co2_properties_are_valid() {
        assert_fluid_is_valid(Fluid::co2());
    }

    #[test]
    fn fluid_names_are_unique() {
        let fluids = [
            Fluid::ammonia(),
            Fluid::water(),
            Fluid::methanol(),
            Fluid::acetone(),
            Fluid::propane(),
            Fluid::propylene(),
            Fluid::r134a(),
            Fluid::r245fa(),
            Fluid::co2(),
        ];

        let mut names = std::collections::HashSet::new();

        for fluid in fluids {
            assert!(
                names.insert(fluid.name),
                "duplicate fluid name: {}",
                fluid.name
            );
        }
    }

    #[test]
    fn all_fluids_have_positive_saturation_slope() {
        let fluids = [
            Fluid::ammonia(),
            Fluid::water(),
            Fluid::methanol(),
            Fluid::acetone(),
            Fluid::propane(),
            Fluid::propylene(),
            Fluid::r134a(),
            Fluid::r245fa(),
            Fluid::co2(),
        ];

        for fluid in fluids {
            let slope = fluid.saturation_pressure_slope(300.0);

            assert!(slope > 0.0, "{} produced a non-positive slope", fluid.name);

            assert!(
                slope.is_finite(),
                "{} produced a non-finite slope",
                fluid.name
            );
        }
    }
}
