use crate::my_set::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum Effect {
    #[default]
    AntiGravity,
    Athletic,
    Balding,
    BrightEyed,
    Calming,
    CalorieDense,
    Cyclopean,
    Disorienting,
    Electrifying,
    Energizing,
    Euphoric,
    Explosive,
    Focused,
    Foggy,
    Gingeritis,
    Glowing,
    Jennerising,
    Laxative,
    LongFaced,
    Munchies,
    Paranoia,
    Refreshing,
    Schizophrenia,
    Sedating,
    SeizureInducing,
    Shrinking,
    Slippery,
    Smelly,
    Sneaky,
    Spicy,
    ThoughtProvoking,
    Toxic,
    TropicThunder,
    Zombifying,
}

use Effect::*;

impl MySetItem for Effect {}

pub fn get_effect_multiplier(e: Effect) -> u32 {
    match e {
        AntiGravity => 54,
        Athletic => 32,
        Balding => 30,
        BrightEyed => 40,
        Calming => 10,
        CalorieDense => 28,
        Cyclopean => 56,
        Disorienting => 0,
        Electrifying => 50,
        Energizing => 22,
        Euphoric => 18,
        Explosive => 0,
        Focused => 16,
        Foggy => 36,
        Gingeritis => 20,
        Glowing => 48,
        Jennerising => 42,
        Laxative => 0,
        LongFaced => 52,
        Munchies => 12,
        Paranoia => 0,
        Refreshing => 14,
        Schizophrenia => 0,
        Sedating => 26,
        SeizureInducing => 0,
        Shrinking => 60,
        Slippery => 34,
        Smelly => 0,
        Sneaky => 24,
        Spicy => 38,
        ThoughtProvoking => 44,
        Toxic => 0,
        TropicThunder => 46,
        Zombifying => 58,
    }
}

pub fn get_effects_multiplier<'a, S, const N: usize>(es: &'a S) -> u32
where
    S: MySet<Effect, N>,
{
    let mut sum = 0;
    for e in es.iter() {
        sum += get_effect_multiplier(e)
    }
    return sum;
}
