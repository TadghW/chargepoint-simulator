use rand::prelude::*;
use rand::distributions::{Distribution, Uniform, WeightedIndex};

fn main() {
    
    let ticks_per_hour: usize = 4;
    let ticks_per_day: usize = ticks_per_hour * 24;
    let ticks_per_year: usize = ticks_per_day * 365; // 15-minute intervals in a non-leap year 
    let num_chargepoints: usize = 20;
    let charging_power: f32 = 11.0; // kW
    let vehicle_efficiency: f32 = 18.0; // kWh per 100 km
    let arrival_probabilities: [f32; 24] = [0.0094, 0.0094, 0.0094, 0.0094, 0.0094, 0.0094, 0.0094, 0.0094, 0.0283, 0.0283, 0.0566, 0.0566, 0.0556, 0.0755, 0.0755, 0.0755, 0.1038, 0.1038, 0.1038, 0.0472, 0.0472, 0.0472, 0.0094, 0.0094];    
    let demand_probabilities: [f32; 9] = [0.3431, 0.049, 0.098, 0.1176, 0.0882, 0.1176, 0.1078, 0.0490, 0.0294];
    let demand_values_dist: [i16; 9] = [0, 5, 10, 20, 30, 50, 100, 200, 300];

    simulate_chargepoints(ticks_per_hour, ticks_per_day, ticks_per_year, num_chargepoints, charging_power, vehicle_efficiency, arrival_probabilities, demand_probabilities, demand_values_dist);

}

fn simulate_chargepoints(ticks_per_hour: usize, ticks_per_day: usize, ticks_per_year: usize, num_chargepoints: usize, charging_power: f32, vehicle_efficiency: f32, arrival_probabilities: [f32; 24], demand_probabilities: [f32; 9], demand_values: [i16; 9]) -> (f32, f32, f32, f32){

    let mut total_energy_consumed: f32 = 0.0;
    let max_power_demand: f32 = charging_power * num_chargepoints as f32;
    let mut power_demand_by_tick: Vec<f32> = vec![0.0; ticks_per_year.try_into().unwrap()]; // Return these for nice power/time visualisations on FE. If mem-use a concern (higher resolution use case i.e.) replace with HashSet and only store power demand change between ticks
    let mut chargepoints: Vec<Option<(usize, f32)>> = vec![None; num_chargepoints as usize]; // Chargepoints envisioned as Vec of Option<(usize, f32)> where tuple.1 is tick when charging completed, and tuple.2 is total consumption over charge duration. More extensive v of sim should have own struct or enum.
    let mut rng: ThreadRng = thread_rng();

    for tick in 0..ticks_per_year {

        for cp in 0..num_chargepoints {

            // Check if any vehicles in occupied chargepoints have finished charging. If they have add their power draw to yearly demand metric and free the chargepoint
            if chargepoints[cp].is_some() {
                if chargepoints[cp].unwrap().0 <= tick {
                    total_energy_consumed += chargepoints[cp].unwrap().1 as f32;
                    chargepoints[cp] = None;
                } 
            }

            // Roll for new arrivals at empty chargepoints
            if chargepoints[cp].is_none() {
                let time_val: f32 = (tick as f32 % ticks_per_day as f32) / ticks_per_hour as f32;
                let hour_val: i32 = time_val.trunc() as i32;
                let arrival_prob_val: f32 = arrival_probabilities[hour_val as usize];
                let arrival_dist: Uniform<f32> = Uniform::new(0.0, 1.0);
                // Roll for demand for charging
                if arrival_dist.sample(&mut rng) < arrival_prob_val {
                    let demand_dist: WeightedIndex<f32> = WeightedIndex::new(&demand_probabilities).unwrap();
                    let km_demand: i16 = demand_values[demand_dist.sample(&mut rng)];
                    if km_demand > 0 {
                        //If vehicle desires charging, update chargepoint to engaged with tick of completion and total power demand
                        let kwh_demand_per_km: f32 = vehicle_efficiency / 100.0;
                        let kwh_demand: f32 = km_demand as f32 * kwh_demand_per_km;
                        let time_demand: f32 = kwh_demand / charging_power;
                        let tick_demand: i32 = (time_demand * ticks_per_hour as f32).trunc() as i32;
                        chargepoints[cp] = Some((tick + tick_demand as usize, kwh_demand.round()));
                    }
                }
            }

        }

        //Power draw = occupied chargepoints * charging power 
        let occupied_chargepoints: usize = chargepoints.iter().filter(|cp| cp.is_some()).count(); 
        let power_demand_of_tick: f32 =  occupied_chargepoints as f32 * charging_power;
        power_demand_by_tick[tick] = power_demand_of_tick;

    }

    // Find maximum power demand of single tick
    let peak_power_demand: &f32 = power_demand_by_tick
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .unwrap()
        .1;

    let results: (f32, f32, f32, f32) = (total_energy_consumed, max_power_demand, *peak_power_demand, (peak_power_demand / max_power_demand));
    println!("Total Energy Consumed (kWh): {:?} Maximum Theoretical Demand (kW): {:?} Maximum Simulated Demand (kW): {:?} Concurrency Factor: {:?}", results.0, results.1, results.2, results.3);
    println!("Demand by Tick: {:?}", [power_demand_by_tick]);
    return results;

}
