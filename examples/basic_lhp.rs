use ferrous_cooling::{fluids::Fluid, lhp::*, wick::Wick};

fn main() {
    let ammonia = Fluid::ammonia();

    let wick = Wick { pore_radius: 1e-6 };

    let capillary = wick.max_capillary_pressure(ammonia.surface_tension, 0.0);

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

    println!("Total loss: {:.1} Pa", result.total_loss);

    println!("Margin: {:.2}%", result.margin_percent);

    println!(
        "Recommended margin satisfied: {}",
        recommended_pressure_margin(capillary, result.total_loss,)
    );
}
