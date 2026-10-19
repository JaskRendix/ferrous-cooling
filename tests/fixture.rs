use std::fs;

use ferrous_cooling::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    fluid: FluidConfig,
    wick: WickConfig,
    pressure_budget: PressureBudgetConfig,
    cold_case: ColdCaseConfig,
    hot_case: HotCaseConfig,
    thermal: ThermalConfig,
    saturation: SaturationConfig,
}

#[derive(Debug, Deserialize)]
struct FluidConfig {
    name: String,
}

#[derive(Debug, Deserialize)]
struct WickConfig {
    pore_radius: f64,
}

#[derive(Debug, Deserialize)]
struct PressureBudgetConfig {
    groove_loss: f64,
    vapor_loss: f64,
    condenser_loss: f64,
    liquid_loss: f64,
    wick_loss: f64,
    gravity_loss: f64,
}

#[derive(Debug, Deserialize)]
struct ColdCaseConfig {
    rho_liquid: f64,
    rho_vapor: f64,
    loop_volume: f64,
    cc_volume: f64,
    liquid_fraction: f64,
}

#[derive(Debug, Deserialize)]
struct HotCaseConfig {
    liquid_density: f64,
    vapor_density: f64,
    liquid_line_volume: f64,
    primary_wick_volume: f64,
    secondary_wick_volume: f64,
    groove_volume: f64,
    vapor_line_volume: f64,
    condenser_volume: f64,
    compensation_chamber_volume: f64,
    vapor_fraction: f64,
}

#[derive(Debug, Deserialize)]
struct ThermalConfig {
    mass_flow_rate: f64,
    specific_heat: f64,
    compensation_temperature: f64,
    inlet_temperature: f64,
    conductance: f64,
    evaporator_temperature: f64,
}

#[derive(Debug, Deserialize)]
struct SaturationConfig {
    temperature: f64,
}

#[test]
fn fixture_runs_complete_analysis() {
    let toml_text = fs::read_to_string("tests/data/basic_lhp.toml").unwrap();

    let fixture: Fixture = toml::from_str(&toml_text).unwrap();

    let fluid = match fixture.fluid.name.as_str() {
        "ammonia" => Fluid::ammonia(),
        _ => panic!("unsupported fluid"),
    };

    //
    // Capillary analysis
    //
    let wick = Wick {
        pore_radius: fixture.wick.pore_radius,
    };

    let capillary = wick.max_capillary_pressure(fluid.surface_tension, 0.0);

    assert!(capillary > 0.0);

    //
    // Pressure budget
    //
    let performance = evaluate(
        capillary,
        PressureBudget {
            groove_loss: fixture.pressure_budget.groove_loss,

            vapor_loss: fixture.pressure_budget.vapor_loss,

            condenser_loss: fixture.pressure_budget.condenser_loss,

            liquid_loss: fixture.pressure_budget.liquid_loss,

            wick_loss: fixture.pressure_budget.wick_loss,

            gravity_loss: fixture.pressure_budget.gravity_loss,
        },
    );

    assert!(performance.total_loss > 0.0);
    assert!(performance.margin_percent > 0.0);

    assert!(recommended_pressure_margin(
        capillary,
        performance.total_loss,
    ));

    //
    // Cold-case inventory
    //
    let cold_inventory = cold_case_inventory(ColdCase {
        rho_liquid: fixture.cold_case.rho_liquid,

        rho_vapor: fixture.cold_case.rho_vapor,

        loop_volume: fixture.cold_case.loop_volume,

        cc_volume: fixture.cold_case.cc_volume,

        liquid_fraction: fixture.cold_case.liquid_fraction,
    });

    assert!(cold_inventory > 0.0);

    //
    // Hot-case inventory
    //
    let hot_inventory = hot_case_inventory(HotCase {
        liquid_density: fixture.hot_case.liquid_density,

        vapor_density: fixture.hot_case.vapor_density,

        liquid_line_volume: fixture.hot_case.liquid_line_volume,

        primary_wick_volume: fixture.hot_case.primary_wick_volume,

        secondary_wick_volume: fixture.hot_case.secondary_wick_volume,

        groove_volume: fixture.hot_case.groove_volume,

        vapor_line_volume: fixture.hot_case.vapor_line_volume,

        condenser_volume: fixture.hot_case.condenser_volume,

        compensation_chamber_volume: fixture.hot_case.compensation_chamber_volume,

        vapor_fraction: fixture.hot_case.vapor_fraction,
    });

    assert!(hot_inventory > 0.0);

    //
    // Saturation slope
    //
    let slope = fluid.saturation_pressure_slope(fixture.saturation.temperature);

    assert!(slope > 0.0);

    //
    // Thermal calculations
    //
    let cc_heat = compensation_chamber_heat(
        fixture.thermal.mass_flow_rate,
        fixture.thermal.specific_heat,
        fixture.thermal.compensation_temperature,
        fixture.thermal.inlet_temperature,
    );

    assert!(cc_heat > 0.0);

    let leak = heat_leak(
        fixture.thermal.conductance,
        fixture.thermal.evaporator_temperature,
        fixture.thermal.compensation_temperature,
    );

    assert!(leak > 0.0);

    let inlet_temp = inlet_temperature(
        fixture.thermal.inlet_temperature,
        leak,
        fixture.thermal.mass_flow_rate,
        fixture.thermal.specific_heat,
    );

    assert!(inlet_temp > fixture.thermal.inlet_temperature);

    //
    // Saturation temperature difference
    //
    let delta_t = saturation_temperature_difference(
        performance.total_loss,
        fixture.pressure_budget.wick_loss,
        slope,
    );

    assert!(delta_t > 0.0);

    //
    // Maximum inventory
    //
    let max_inventory = maximum_inventory(
        fixture.cold_case.rho_liquid,
        fixture.cold_case.loop_volume,
        fixture.cold_case.cc_volume,
    );

    assert!(max_inventory > 0.0);

    //
    // General sanity checks
    //
    assert!(max_inventory > cold_inventory);
    assert!(slope.is_finite());
    assert!(delta_t.is_finite());
    assert!(inlet_temp.is_finite());
}
