use clap::{Args, Parser, Subcommand};

use ferrous_cooling::{
    ColdCase, Fluid, HotCase, PressureBudget, Wick, cold_case_inventory, compensation_chamber_heat,
    evaluate, heat_leak, hot_case_inventory, inlet_temperature, recommended_pressure_margin,
    saturation_temperature_difference,
};

#[derive(Parser)]
#[command(
    name = "ferrous-cooling",
    version,
    about = "Loop Heat Pipe analytical toolkit"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Capillary(CapillaryArgs),
    Budget(BudgetArgs),

    Inventory {
        #[command(subcommand)]
        command: InventoryCommand,
    },

    SaturationSlope(SaturationSlopeArgs),
    Thermal(ThermalArgs),
    DeltaT(DeltaTArgs),
}

#[derive(Args)]
struct CapillaryArgs {
    #[arg(long)]
    surface_tension: f64,

    #[arg(long)]
    contact_angle: f64,

    #[arg(long)]
    pore_radius: f64,
}

#[derive(Args)]
struct BudgetArgs {
    #[arg(long)]
    capillary_pressure: f64,

    #[arg(long)]
    groove: f64,

    #[arg(long)]
    vapor: f64,

    #[arg(long)]
    condenser: f64,

    #[arg(long)]
    liquid: f64,

    #[arg(long)]
    wick: f64,

    #[arg(long)]
    gravity: f64,
}

#[derive(Subcommand)]
enum InventoryCommand {
    Cold(ColdInventoryArgs),
    Hot(HotInventoryArgs),
}

#[derive(Args)]
struct ColdInventoryArgs {
    #[arg(long)]
    rho_liquid: f64,

    #[arg(long)]
    rho_vapor: f64,

    #[arg(long)]
    loop_volume: f64,

    #[arg(long)]
    cc_volume: f64,

    #[arg(long)]
    liquid_fraction: f64,
}

#[derive(Args)]
struct HotInventoryArgs {
    #[arg(long)]
    liquid_density: f64,

    #[arg(long)]
    vapor_density: f64,

    #[arg(long)]
    liquid_line_volume: f64,

    #[arg(long)]
    primary_wick_volume: f64,

    #[arg(long)]
    secondary_wick_volume: f64,

    #[arg(long)]
    groove_volume: f64,

    #[arg(long)]
    vapor_line_volume: f64,

    #[arg(long)]
    condenser_volume: f64,

    #[arg(long)]
    compensation_chamber_volume: f64,

    #[arg(long)]
    vapor_fraction: f64,
}

#[derive(Args)]
struct SaturationSlopeArgs {
    #[arg(long, default_value = "ammonia")]
    fluid: String,

    #[arg(long)]
    temperature: f64,
}

fn parse_fluid(name: &str) -> Option<Fluid> {
    match name.to_lowercase().as_str() {
        "ammonia" => Some(Fluid::ammonia()),
        "water" => Some(Fluid::water()),
        "methanol" => Some(Fluid::methanol()),
        "acetone" => Some(Fluid::acetone()),
        "propane" => Some(Fluid::propane()),
        "propylene" => Some(Fluid::propylene()),
        "r134a" => Some(Fluid::r134a()),
        "r245fa" => Some(Fluid::r245fa()),
        "co2" => Some(Fluid::co2()),
        _ => None,
    }
}

#[derive(Args)]
struct ThermalArgs {
    #[arg(long)]
    mass_flow_rate: f64,

    #[arg(long)]
    specific_heat: f64,

    #[arg(long)]
    compensation_temperature: f64,

    #[arg(long)]
    inlet_temperature: f64,

    #[arg(long)]
    conductance: f64,

    #[arg(long)]
    evaporator_temperature: f64,
}

#[derive(Args)]
struct DeltaTArgs {
    #[arg(long)]
    total_pressure_drop: f64,

    #[arg(long)]
    wick_pressure_drop: f64,

    #[arg(long)]
    pressure_slope: f64,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Capillary(args) => {
            let wick = Wick {
                pore_radius: args.pore_radius,
            };

            let pressure = wick.capillary_pressure(args.surface_tension, args.contact_angle);

            println!("Capillary pressure: {:.2} Pa", pressure);
        }

        Commands::Budget(args) => {
            let result = evaluate(
                args.capillary_pressure,
                PressureBudget {
                    groove_loss: args.groove,
                    vapor_loss: args.vapor,
                    condenser_loss: args.condenser,
                    liquid_loss: args.liquid,
                    wick_loss: args.wick,
                    gravity_loss: args.gravity,
                },
            );

            println!("Total pressure loss: {:.2} Pa", result.total_loss);

            println!("Margin: {:.2} %", result.margin_percent);

            println!(
                "Recommended margin satisfied: {}",
                recommended_pressure_margin(args.capillary_pressure, result.total_loss,)
            );
        }

        Commands::Inventory {
            command: InventoryCommand::Cold(args),
        } => {
            let inventory = cold_case_inventory(ColdCase {
                rho_liquid: args.rho_liquid,
                rho_vapor: args.rho_vapor,
                loop_volume: args.loop_volume,
                cc_volume: args.cc_volume,
                liquid_fraction: args.liquid_fraction,
            });

            println!("Required inventory: {:.6} kg", inventory);
        }

        Commands::Inventory {
            command: InventoryCommand::Hot(args),
        } => {
            let inventory = hot_case_inventory(HotCase {
                liquid_density: args.liquid_density,
                vapor_density: args.vapor_density,
                liquid_line_volume: args.liquid_line_volume,
                primary_wick_volume: args.primary_wick_volume,
                secondary_wick_volume: args.secondary_wick_volume,
                groove_volume: args.groove_volume,
                vapor_line_volume: args.vapor_line_volume,
                condenser_volume: args.condenser_volume,
                compensation_chamber_volume: args.compensation_chamber_volume,
                vapor_fraction: args.vapor_fraction,
            });

            println!("Required inventory: {:.6} kg", inventory);
        }

        Commands::SaturationSlope(args) => {
            let fluid = parse_fluid(&args.fluid).unwrap_or_else(|| {
                eprintln!("Unsupported fluid: {}", args.fluid);
                std::process::exit(1);
            });

            let slope = fluid.saturation_pressure_slope(args.temperature);

            println!("Fluid: {}", fluid.name);
            println!("Temperature: {:.2} K", args.temperature);
            println!("dP/dT: {:.3} Pa/K", slope);
        }

        Commands::Thermal(args) => {
            let cc_heat = compensation_chamber_heat(
                args.mass_flow_rate,
                args.specific_heat,
                args.compensation_temperature,
                args.inlet_temperature,
            );

            let leak = heat_leak(
                args.conductance,
                args.evaporator_temperature,
                args.compensation_temperature,
            );

            let t_in = inlet_temperature(
                args.inlet_temperature,
                leak,
                args.mass_flow_rate,
                args.specific_heat,
            );

            println!("Compensation chamber heat: {:.3} W", cc_heat);

            println!("Heat leak: {:.3} W", leak);

            println!("Resulting inlet temperature: {:.3} K", t_in);
        }

        Commands::DeltaT(args) => {
            let delta_t = saturation_temperature_difference(
                args.total_pressure_drop,
                args.wick_pressure_drop,
                args.pressure_slope,
            );

            println!("Saturation temperature difference: {:.6} K", delta_t);
        }
    }
}
