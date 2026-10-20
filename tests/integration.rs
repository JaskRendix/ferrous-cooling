use ferrous_cooling::*;

#[test]
fn lhp_design_has_positive_margin() {
    let fluid = Fluid::ammonia();

    let wick = Wick::typical_ammonia_wick();

    let capillary = wick.max_capillary_pressure(fluid.surface_tension);

    let result = evaluate(
        capillary,
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
fn lhp_design_meets_recommended_margin() {
    let fluid = Fluid::ammonia();

    let wick = Wick::typical_ammonia_wick();

    let capillary = wick.max_capillary_pressure(fluid.surface_tension);

    let result = evaluate(
        capillary,
        PressureBudget {
            groove_loss: 500.0,
            vapor_loss: 2_000.0,
            condenser_loss: 1_000.0,
            liquid_loss: 8_000.0,
            wick_loss: 300.0,
            gravity_loss: 500.0,
        },
    );

    assert!(recommended_pressure_margin(capillary, result.total_loss,));
}

#[test]
fn cold_case_inventory_is_positive() {
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
fn hot_case_inventory_is_positive() {
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
fn evaporation_heat_is_positive() {
    let fluid = Fluid::ammonia();

    let heat = evaporation_heat(&fluid, 0.001);

    assert!(heat > 0.0);
}
