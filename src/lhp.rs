use crate::fluids::Fluid;

/// Parameters for the cold-case fluid inventory calculation.
#[derive(Debug, Clone, Copy)]
pub struct ColdCase {
    /// Liquid density [kg/m³].
    pub rho_liquid: f64,

    /// Vapor density [kg/m³].
    pub rho_vapor: f64,

    /// Total loop volume excluding the compensation chamber [m³].
    pub loop_volume: f64,

    /// Compensation chamber volume [m³].
    pub cc_volume: f64,

    /// Fraction of compensation chamber occupied by liquid.
    pub liquid_fraction: f64,
}

/// Computes the required fluid inventory for the cold case.
///
/// Equation (8) from Ku (1999).
///
/// # Example
///
/// ```rust
/// use ferrous_cooling::lhp::{
///     ColdCase,
///     cold_case_inventory,
/// };
///
/// let inventory = cold_case_inventory(
///     ColdCase {
///         rho_liquid: 600.0,
///         rho_vapor: 5.0,
///         loop_volume: 1.0e-4,
///         cc_volume: 2.0e-5,
///         liquid_fraction: 0.8,
///     },
/// );
///
/// assert!(inventory > 0.0);
/// ```
pub fn cold_case_inventory(cold_case: ColdCase) -> f64 {
    cold_case.rho_liquid * (cold_case.loop_volume + cold_case.liquid_fraction * cold_case.cc_volume)
        + cold_case.rho_vapor * (1.0 - cold_case.liquid_fraction) * cold_case.cc_volume
}

/// Recommended minimum capillary-pressure margin.
///
/// NASA guidance suggests maintaining roughly
/// 50% margin between available capillary pressure
/// and calculated system pressure losses.
pub const RECOMMENDED_MARGIN_RATIO: f64 = 0.5;

/// Pressure losses throughout the Loop Heat Pipe.
///
/// All values are expressed in Pascals (Pa).
#[derive(Debug, Clone, Copy)]
pub struct PressureBudget {
    /// Pressure drop in the evaporator grooves.
    pub groove_loss: f64,

    /// Pressure drop in the vapor transport line.
    pub vapor_loss: f64,

    /// Pressure drop through the condenser.
    pub condenser_loss: f64,

    /// Pressure drop in the liquid transport line.
    pub liquid_loss: f64,

    /// Pressure drop across the wick.
    pub wick_loss: f64,

    /// Hydrostatic (gravity/elevation) pressure loss.
    pub gravity_loss: f64,
}

/// Computes the total pressure loss in the loop.
///
/// Based on Equation (1) from Ku (1999):
///
/// ΔP_total = ΔP_groove
///          + ΔP_vapor
///          + ΔP_condenser
///          + ΔP_liquid
///          + ΔP_wick
///          + ΔP_gravity
pub fn total_pressure_loss(budget: PressureBudget) -> f64 {
    budget.groove_loss
        + budget.vapor_loss
        + budget.condenser_loss
        + budget.liquid_loss
        + budget.wick_loss
        + budget.gravity_loss
}

/// Result of a capillary-pressure budget evaluation.
#[derive(Debug, Clone, Copy)]
pub struct Performance {
    /// Sum of all pressure losses [Pa].
    pub total_loss: f64,

    /// Remaining capillary margin [%].
    ///
    /// Values above 0% indicate the wick can theoretically
    /// overcome the calculated pressure losses.
    pub margin_percent: f64,
}

/// Evaluates whether the available capillary pressure is
/// sufficient to overcome all loop pressure losses.
///
/// # Arguments
///
/// * `capillary_pressure` - Available capillary pumping pressure [Pa]
/// * `budget` - Pressure-loss budget for the loop
///
/// # Returns
///
/// A [`Performance`] summary containing:
///
/// * total pressure loss
/// * remaining capillary margin
///
/// # Formula
///
/// ```text
/// margin = (ΔP_cap - ΔP_total) / ΔP_cap × 100
/// ```
///
/// # Example
///
/// ```rust
/// use ferrous_cooling::lhp::{
///     evaluate,
///     PressureBudget,
/// };
///
/// let result = evaluate(
///     46_000.0,
///     PressureBudget {
///         groove_loss: 500.0,
///         vapor_loss: 2_000.0,
///         condenser_loss: 1_000.0,
///         liquid_loss: 8_000.0,
///         wick_loss: 300.0,
///         gravity_loss: 500.0,
///     },
/// );
///
/// assert!(result.margin_percent > 0.0);
/// ```
pub fn evaluate(capillary_pressure: f64, budget: PressureBudget) -> Performance {
    let total_loss = total_pressure_loss(budget);

    let margin = (capillary_pressure - total_loss) / capillary_pressure * 100.0;

    Performance {
        total_loss,
        margin_percent: margin,
    }
}

/// Returns true if the design satisfies the
/// recommended 50% pressure margin.
///
/// The NASA paper recommends maintaining a
/// substantial margin between capillary pumping
/// capability and total system pressure losses.
pub fn recommended_pressure_margin(capillary_pressure: f64, total_loss: f64) -> bool {
    total_loss <= capillary_pressure * RECOMMENDED_MARGIN_RATIO
}

/// Vaporization heat transfer.
///
/// Equation (12) from Ku (1999):
///
/// Q = m λ
///
/// # Example
///
/// ```rust
/// use ferrous_cooling::fluids::Fluid;
/// use ferrous_cooling::lhp::evaporation_heat;
///
/// let fluid = Fluid::ammonia();
///
/// let heat = evaporation_heat(
///     &fluid,
///     0.001,
/// );
///
/// assert!(heat > 0.0);
/// ```
pub fn evaporation_heat(fluid: &Fluid, mass_flow_rate: f64) -> f64 {
    mass_flow_rate * fluid.latent_heat
}

/// Compensation-chamber heat balance.
///
/// Equation (14) from Ku (1999):
///
/// Q = m Cp (Tcc - Tin)
///
/// # Example
///
/// ```rust
/// use ferrous_cooling::lhp::compensation_chamber_heat;
///
/// let heat = compensation_chamber_heat(
///     0.001,
///     4700.0,
///     300.0,
///     295.0,
/// );
///
/// assert!(heat > 0.0);
/// ```
pub fn compensation_chamber_heat(
    mass_flow_rate: f64,
    specific_heat: f64,
    compensation_temperature: f64,
    inlet_temperature: f64,
) -> f64 {
    mass_flow_rate * specific_heat * (compensation_temperature - inlet_temperature)
}

/// Computes the liquid temperature entering the
/// compensation chamber.
///
/// Rearranged Equation (15) from Ku (1999).
///
/// # Example
///
/// ```rust
/// use ferrous_cooling::lhp::inlet_temperature;
///
/// let t_in = inlet_temperature(
///     280.0,
///     50.0,
///     0.001,
///     4700.0,
/// );
///
/// assert!(t_in > 280.0);
/// ```
pub fn inlet_temperature(
    subcooler_exit_temperature: f64,
    liquid_line_heat_gain: f64,
    mass_flow_rate: f64,
    specific_heat: f64,
) -> f64 {
    subcooler_exit_temperature + liquid_line_heat_gain / (mass_flow_rate * specific_heat)
}

/// Heat leak from the evaporator to the
/// compensation chamber.
///
/// Equation (13) from Ku (1999):
///
/// Q = G (Te - Tcc)
///
/// # Example
///
/// ```rust
/// use ferrous_cooling::lhp::heat_leak;
///
/// let leak = heat_leak(
///     0.25,
///     310.0,
///     300.0,
/// );
///
/// assert!(leak > 0.0);
/// ```
pub fn heat_leak(
    conductance: f64,
    evaporator_temperature: f64,
    compensation_temperature: f64,
) -> f64 {
    conductance * (evaporator_temperature - compensation_temperature)
}

/// Computes the maximum allowable fluid inventory.
///
/// Equation (10) from Ku (1999):
///
/// M ≤ ρ (Vloop + Vcc)
///
/// # Example
///
/// ```rust
/// use ferrous_cooling::lhp::maximum_inventory;
///
/// let max_inventory = maximum_inventory(
///     600.0,
///     1.0e-4,
///     2.0e-5,
/// );
///
/// assert!(max_inventory > 0.0);
/// ```
pub fn maximum_inventory(liquid_density_max: f64, loop_volume: f64, cc_volume: f64) -> f64 {
    liquid_density_max * (loop_volume + cc_volume)
}

/// Parameters for the hot-case fluid inventory calculation.
///
/// Equation (9) from Ku (1999).
///
/// # Example
///
/// ```rust
/// use ferrous_cooling::lhp::{
///     HotCase,
///     hot_case_inventory,
/// };
///
/// let inventory = hot_case_inventory(
///     HotCase {
///         liquid_density: 550.0,
///         vapor_density: 8.0,
///         liquid_line_volume: 1.0e-5,
///         primary_wick_volume: 2.0e-6,
///         secondary_wick_volume: 2.0e-6,
///         groove_volume: 1.0e-6,
///         vapor_line_volume: 5.0e-6,
///         condenser_volume: 1.5e-5,
///         compensation_chamber_volume: 1.0e-5,
///         vapor_fraction: 0.3,
///     },
/// );
///
/// assert!(inventory > 0.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct HotCase {
    /// Liquid density in the hot case [kg/m³].
    pub liquid_density: f64,

    /// Vapor density in the hot case [kg/m³].
    pub vapor_density: f64,

    /// Liquid-line volume [m³].
    pub liquid_line_volume: f64,

    /// Primary-wick void volume [m³].
    pub primary_wick_volume: f64,

    /// Secondary-wick void volume [m³].
    pub secondary_wick_volume: f64,

    /// Evaporator groove volume [m³].
    pub groove_volume: f64,

    /// Vapor-line volume [m³].
    pub vapor_line_volume: f64,

    /// Condenser volume [m³].
    pub condenser_volume: f64,

    /// Compensation chamber volume [m³].
    pub compensation_chamber_volume: f64,

    /// Vapor fraction α within the compensation chamber.
    pub vapor_fraction: f64,
}

/// Computes the required fluid inventory for the hot case.
///
/// Equation (9) from Ku (1999).
///
/// # Example
///
/// ```rust
/// use ferrous_cooling::lhp::{
///     HotCase,
///     hot_case_inventory,
/// };
///
/// let inventory = hot_case_inventory(
///     HotCase {
///         liquid_density: 550.0,
///         vapor_density: 8.0,
///         liquid_line_volume: 1.0e-5,
///         primary_wick_volume: 2.0e-6,
///         secondary_wick_volume: 2.0e-6,
///         groove_volume: 1.0e-6,
///         vapor_line_volume: 5.0e-6,
///         condenser_volume: 1.5e-5,
///         compensation_chamber_volume: 1.0e-5,
///         vapor_fraction: 0.3,
///     },
/// );
///
/// assert!(inventory > 0.0);
/// ```
pub fn hot_case_inventory(hot_case: HotCase) -> f64 {
    hot_case.liquid_density
        * (hot_case.liquid_line_volume
            + hot_case.primary_wick_volume
            + hot_case.secondary_wick_volume
            + (1.0 - hot_case.vapor_fraction) * hot_case.compensation_chamber_volume)
        + hot_case.vapor_density
            * (hot_case.groove_volume
                + hot_case.vapor_line_volume
                + hot_case.condenser_volume
                + hot_case.vapor_fraction * hot_case.compensation_chamber_volume)
}

/// Computes the saturation-temperature difference
/// between the evaporator and compensation chamber.
///
/// Rearranged Equation (5) from Ku (1999):
///
/// ΔT = (ΔPtotal - ΔPwick) / (dP/dT)
///
/// # Example
///
/// ```rust
/// use ferrous_cooling::lhp::saturation_temperature_difference;
///
/// let delta_t = saturation_temperature_difference(
///     15_000.0,
///     2_000.0,
///     40_000.0,
/// );
///
/// assert!(delta_t > 0.0);
/// ```
pub fn saturation_temperature_difference(
    total_pressure_drop: f64,
    wick_pressure_drop: f64,
    saturation_pressure_slope: f64,
) -> f64 {
    (total_pressure_drop - wick_pressure_drop) / saturation_pressure_slope
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fluids::Fluid;

    #[test]
    fn computes_total_pressure_loss() {
        let budget = PressureBudget {
            groove_loss: 500.0,
            vapor_loss: 2_000.0,
            condenser_loss: 1_000.0,
            liquid_loss: 8_000.0,
            wick_loss: 300.0,
            gravity_loss: 500.0,
        };

        assert_eq!(total_pressure_loss(budget), 12_300.0);
    }

    #[test]
    fn evaluates_margin() {
        let result = evaluate(
            46_000.0,
            PressureBudget {
                groove_loss: 500.0,
                vapor_loss: 2_000.0,
                condenser_loss: 1_000.0,
                liquid_loss: 8_000.0,
                wick_loss: 300.0,
                gravity_loss: 500.0,
            },
        );

        assert!(result.margin_percent > 0.0);
    }

    #[test]
    fn recommended_margin_passes() {
        assert!(recommended_pressure_margin(100_000.0, 40_000.0,));
    }

    #[test]
    fn recommended_margin_fails() {
        assert!(!recommended_pressure_margin(100_000.0, 60_000.0,));
    }

    #[test]
    fn computes_cold_case_inventory() {
        let inventory = cold_case_inventory(ColdCase {
            rho_liquid: 600.0,
            rho_vapor: 5.0,
            loop_volume: 1.0e-4,
            cc_volume: 2.0e-5,
            liquid_fraction: 0.8,
        });

        assert!(inventory > 0.0);
    }

    #[test]
    fn computes_hot_case_inventory() {
        let inventory = hot_case_inventory(HotCase {
            liquid_density: 550.0,
            vapor_density: 8.0,
            liquid_line_volume: 1.0e-5,
            primary_wick_volume: 2.0e-6,
            secondary_wick_volume: 2.0e-6,
            groove_volume: 1.0e-6,
            vapor_line_volume: 5.0e-6,
            condenser_volume: 1.5e-5,
            compensation_chamber_volume: 1.0e-5,
            vapor_fraction: 0.3,
        });

        assert!(inventory > 0.0);
    }

    #[test]
    fn computes_evaporation_heat() {
        let fluid = Fluid::ammonia();

        let heat = evaporation_heat(&fluid, 0.001);

        assert!(heat > 0.0);
    }

    #[test]
    fn computes_heat_leak() {
        let heat = heat_leak(0.25, 310.0, 300.0);

        assert_eq!(heat, 2.5);
    }

    #[test]
    fn computes_compensation_chamber_heat() {
        let heat = compensation_chamber_heat(0.001, 4700.0, 300.0, 295.0);

        assert_eq!(heat, 23.5);
    }

    #[test]
    fn computes_inlet_temperature() {
        let t_in = inlet_temperature(280.0, 50.0, 0.001, 4700.0);

        assert!(t_in > 280.0);
    }

    #[test]
    fn computes_saturation_temperature_difference() {
        let dt = saturation_temperature_difference(15_000.0, 2_000.0, 40_000.0);

        assert!((dt - 0.325).abs() < 1e-6);
    }

    #[test]
    fn computes_maximum_inventory() {
        let inv = maximum_inventory(600.0, 1.0e-4, 2.0e-5);

        assert!((inv - 0.072).abs() < 1e-12);
    }
}
