#[cfg(test)]
use std::time::*;

mod my_set;
use crate::my_set::*;
mod find_max;
use crate::find_max::*;
mod effects;
use crate::effects::*;
mod mixers;
use crate::mixers::*;
mod mixing;
use crate::mixing::*;

use Effect::*;
use csv::WriterBuilder;

fn _print_max<S, const N: usize, const P: usize>(
    maxes: Vec<
        [(
            &str,                        // Name of the product
            u32,                         // Profit
            u32,                         // Base value
            Vec<mixers::Mixer>,  // Max mixers
            S,                           // Max effects
            Option<std::time::Duration>, // Time to compute the max, if test attr
        ); P],
    >,
) where
    S: MySet<Effect, N>,
{
    for m in maxes {
        for (name, max_profit, value, max_mixers, max_effects, time) in m {
            println!("Max Value {} with {} ingredients:", name, max_mixers.len());
            println!("  Value: {:?}", value);
            println!("  Cost: {:?}", get_mixers_cost_vec(&max_mixers));
            println!("  Profit: {:?}", max_profit);
            println!("  Effects:");
            for e in max_effects.iter() {
                println!("  - {:?}", e)
            }
            println!("");
            println!("  Ingredients:");
            for m in max_mixers.iter() {
                println!("  - {:?}", m);
            }
            println!("");
            println!("");
            time.map(|t| {
                println!("Running took {} seconds.", t.as_secs());
                println!("");
                println!("");
            });
        }
    }
}

fn _csv_max<S, const N: usize, const P: usize>(
    maxes: Vec<
        [(
            &str,                        // Name of the product
            u32,                         // Profit
            u32,                         // Base value
            Vec<mixers::Mixer>,          // Max mixers
            S,                           // Max effects
            Option<std::time::Duration>, // Time to compute the max, if test attr
        ); P],
    >,
) where
    S: MySet<Effect, N>,
{
    // The closure is just to use ?, since this function as a whole returns unit, not Result
    let res = || {
        let mut wtr = WriterBuilder::new().from_path("results.csv")?;
        wtr.write_record(&[
            "Product",
            "Profit",
            "Sell Value",
            "Cost",
            "Mixers",
            "Effects",
            "Runtime",
        ])?;
        for m in maxes {
            let rows = m.map(|(name, max_profit, value, max_mixers, max_effects, time)| {
                (
                    name,
                    max_profit,
                    value,
                    get_mixers_cost_vec(&max_mixers),
                    max_mixers,
                    max_effects,
                    time,
                )
            });
            for row in rows.iter() {
                wtr.write_record(&[
                    row.0,
                    &row.1.to_string(),
                    &row.2.to_string(),
                    &row.3.to_string(),
                    &format!("{:?}", row.4),
                    &format!("{:?}", row.5.iter().collect::<Vec<Effect>>()),
                    &row.6
                        .map_or(String::default(), |t| format!("{:?}", t.as_secs())),
                ])?;
            }
        }
        Ok::<(), Box<dyn std::error::Error>>(())
    };
    match res() {
        Ok(_) => println!("CSV OK"),
        Err(e) => println!("Error while creating csv: {}", e),
    }
}

const MAX_ES: usize = 8;

fn run_max<S, const P: usize, const M: usize>(
    products: [(&str, S, u32); P],
) -> [(
    &str,                        // Name of the product
    u32,                         // Profit
    u32,                         // Base value
    Vec<mixers::Mixer>,          // Max mixers
    S,                           // Max effects
    Option<std::time::Duration>, // Time to compute the max, if test attr
); P]
where
    S: MySet<Effect, MAX_ES>,
    S: Sync,
    S: Send,
{
    products.map(|(name, init_effects, base_value)| {
        #[cfg(test)]
        let now = Instant::now();
        let (max_effects, max_mixers, max_profit) =
            find_max::<S, MAX_ES, M>(init_effects.clone(), base_value);
        // Underscore to suppress unused when test
        let _elapsed_time: Option<std::time::Duration> = None;
        #[cfg(test)]
        let _elapsed_time = Some(now.elapsed());
        let value = get_price(base_value, get_effects_multiplier(&max_effects));
        (
            name,
            max_profit,
            value,
            max_mixers.iter().flatten().map(|m| { *m }).collect::<Vec<Mixer>>(),
            max_effects,
            _elapsed_time,
        )
    })
}

macro_rules! repeat_run_max {
    ($v:expr, $s:ty, $plen:expr, $p:expr, $($n:expr),*) => {
        $(
            $v.push(run_max::<$s, $plen, $n>($p));
        )*
    };
}

fn main() {
    let mut og_effects = ArraySet::<Effect, MAX_ES>::new();
    og_effects.insert(Calming);
    let mut sd_effects = ArraySet::<Effect, MAX_ES>::new();
    sd_effects.insert(Refreshing);
    let mut gc_effects = ArraySet::<Effect, MAX_ES>::new();
    gc_effects.insert(Energizing);
    let mut gd_effects = ArraySet::<Effect, MAX_ES>::new();
    gd_effects.insert(Sedating);
    let meth_effects = ArraySet::<Effect, MAX_ES>::new();
    let coke_effects = ArraySet::<Effect, MAX_ES>::new();

    let products = [
        ("OG Kush", og_effects, 35),
        ("Sour Diesel", sd_effects, 35),
        ("Green Crack", gc_effects, 35),
        ("Grandaddy Purple", gd_effects, 35),
        ("Meth", meth_effects, 70),
        ("Coke", coke_effects, 150),
    ];

    let mut maxes = Vec::new();
    repeat_run_max!(maxes, ArraySet<Effect, MAX_ES>, 6, products, 1, 2, 3, 4, 5, 6, 7, 8);
    // repeat_run_max!(maxes, ArraySet<Effect, MAX_ES>, 6, products, 1, 2, 3, 4, 5, 6, 7);
    // repeat_run_max!(maxes, ArraySet<Effect, MAX_ES>, 6, products, 1, 2, 3, 4, 5, 6);
    // TODO: cli or separate executables
    // _print_max(maxes);
    _csv_max(maxes);
}

// Need to run with --nocapture to see the output
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main()
    }
}
