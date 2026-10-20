/// Thermophysical properties of a working fluid.
///
/// Units:
/// - surface_tension [N/m]
/// - latent_heat [J/kg]
/// - liquid_density [kg/m³]
/// - vapor_density [kg/m³]
/// - liquid_specific_heat [J/(kg·K)]
/// - liquid_viscosity [Pa·s]
/// - thermal_conductivity [W/(m·K)]
#[derive(Debug, Clone, Copy)]
pub struct Fluid {
    pub name: &'static str,
    pub surface_tension: f64,
    pub latent_heat: f64,
    pub liquid_density: f64,
    pub vapor_density: f64,
    pub liquid_specific_heat: f64,
    pub liquid_viscosity: f64,
    pub thermal_conductivity: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FluidKind {
    Ammonia,
    Water,
    Methanol,
    Acetone,
    Propane,
    Propylene,
    R134a,
    R245fa,
    Co2,
}

impl Fluid {
    pub fn from_kind(kind: FluidKind) -> Self {
        match kind {
            FluidKind::Ammonia => Self::ammonia(),
            FluidKind::Water => Self::water(),
            FluidKind::Methanol => Self::methanol(),
            FluidKind::Acetone => Self::acetone(),
            FluidKind::Propane => Self::propane(),
            FluidKind::Propylene => Self::propylene(),
            FluidKind::R134a => Self::r134a(),
            FluidKind::R245fa => Self::r245fa(),
            FluidKind::Co2 => Self::co2(),
        }
    }

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
            surface_tension: 0.023,
            latent_heat: 1_370_000.0,
            liquid_density: 682.0,
            vapor_density: 7.9,
            liquid_specific_heat: 4700.0,
            liquid_viscosity: 2.5e-4,
            thermal_conductivity: 0.52,
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
            liquid_viscosity: 8.9e-4,
            thermal_conductivity: 0.60,
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
            liquid_viscosity: 5.4e-4,
            thermal_conductivity: 0.20,
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
            liquid_viscosity: 3.1e-4,
            thermal_conductivity: 0.16,
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
            liquid_viscosity: 1.2e-4,
            thermal_conductivity: 0.11,
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
            liquid_viscosity: 1.1e-4,
            thermal_conductivity: 0.10,
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
            liquid_viscosity: 2.0e-4,
            thermal_conductivity: 0.08,
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
            liquid_viscosity: 4.0e-4,
            thermal_conductivity: 0.09,
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
            liquid_viscosity: 7.0e-5,
            thermal_conductivity: 0.10,
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

    pub fn reynolds_number(&self, velocity: f64, hydraulic_diameter: f64) -> f64 {
        self.liquid_density * velocity * hydraulic_diameter / self.liquid_viscosity
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

        assert!(fluid.liquid_viscosity > 0.0);
        assert!(fluid.thermal_conductivity > 0.0);
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

    #[test]
    fn reynolds_number_is_positive() {
        let fluid = Fluid::ammonia();

        let re = fluid.reynolds_number(1.0, 0.002);

        assert!(re > 0.0);
    }

    #[test]
    fn reynolds_number_increases_with_velocity() {
        let fluid = Fluid::ammonia();

        let low = fluid.reynolds_number(1.0, 0.002);

        let high = fluid.reynolds_number(2.0, 0.002);

        assert!(high > low);
    }

    #[test]
    fn ammonia_transport_properties_are_positive() {
        let fluid = Fluid::ammonia();

        assert!(fluid.liquid_viscosity > 0.0);

        assert!(fluid.thermal_conductivity > 0.0);
    }
}
