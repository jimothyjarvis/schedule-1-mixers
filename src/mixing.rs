use crate::effects::*;
use crate::mixers::*;
use crate::my_set::*;
use Effect::*;
use Mixer::*;

fn mixer_default(m: Mixer) -> Effect {
    match m {
        Cuke => Energizing,
        FluMedicine => Sedating,
        Gasoline => Toxic,
        Donut => CalorieDense,
        EnergyDrink => Athletic,
        MouthWash => Balding,
        MotorOil => Slippery,
        Banana => Gingeritis,
        Chili => Spicy,
        Iodine => Jennerising,
        Paracetamol => Sneaky,
        Viagra => TropicThunder,
        HorseSemen => LongFaced,
        MegaBean => Foggy,
        Addy => ThoughtProvoking,
        Battery => BrightEyed,
    }
}

fn mixer_transform<const N: usize, S>(m: Mixer, out: &mut S) -> ()
where
    S: MySet<Effect, N>,
{
    match m {
        Cuke => {
            out.replace(Euphoric, Laxative);
            out.replace(Toxic, Euphoric);
            out.replace(Munchies, Athletic);
            out.replace(Slippery, Munchies);
            out.replace(Foggy, Cyclopean);
            out.replace(Gingeritis, ThoughtProvoking);
            out.replace(Sneaky, Paranoia);
        }
        FluMedicine => {
            out.replace(Munchies, Slippery);
            out.replace(Athletic, Munchies);
            out.replace(Calming, BrightEyed);
            out.replace(Focused, Calming);
            out.replace(Euphoric, Toxic);
            out.replace(Laxative, Euphoric);
            out.replace(Cyclopean, Foggy);
            out.replace(Electrifying, Refreshing);
            out.replace(Shrinking, Paranoia);
            out.replace(ThoughtProvoking, Gingeritis);
        }
        Gasoline => {
            out.replace(Disorienting, Glowing);
            out.replace(Electrifying, Disorienting);
            out.replace(Euphoric, Spicy);
            out.replace(Energizing, Euphoric);
            out.replace(Sneaky, TropicThunder);
            out.replace(Jennerising, Sneaky);
            out.replace(Gingeritis, Smelly);
            out.replace(Laxative, Foggy);
            out.replace(Munchies, Sedating);
            out.replace(Paranoia, Calming);
            out.replace(Shrinking, Focused);
        }
        Donut => {
            out.replace(AntiGravity, Slippery);
            out.replace(Balding, Sneaky);
            out.replace(CalorieDense, Explosive);
            out.replace(Focused, Euphoric);
            out.replace(Jennerising, Gingeritis);
            out.replace(Munchies, Calming);
            out.replace(Shrinking, Energizing);
        }
        EnergyDrink => {
            out.replace(Disorienting, Electrifying);
            out.replace(Glowing, Disorienting);
            out.replace(Euphoric, Energizing);
            out.replace(Spicy, Euphoric);
            out.replace(Focused, Shrinking);
            out.replace(Foggy, Laxative);
            out.replace(Schizophrenia, Balding);
            out.replace(Sedating, Munchies);
            out.replace(TropicThunder, Sneaky);
        }
        MouthWash => {
            out.replace(Calming, AntiGravity);
            out.replace(CalorieDense, Sneaky);
            out.replace(Explosive, Sedating);
            out.replace(Focused, Jennerising);
        }
        MotorOil => {
            out.replace(Munchies, Schizophrenia);
            out.replace(Energizing, Munchies);
            out.replace(Euphoric, Sedating);
            out.replace(Foggy, Toxic);
            out.replace(Paranoia, AntiGravity);
        }
        Banana => {
            out.replace(Calming, Sneaky);
            out.replace(Energizing, ThoughtProvoking);
            out.replace(Cyclopean, Energizing);
            out.replace(Focused, SeizureInducing);
            out.replace(Disorienting, Focused);
            out.replace(LongFaced, Refreshing);
            out.replace(Paranoia, Jennerising);
            out.replace(Smelly, AntiGravity);
            out.replace(Toxic, Smelly);
        }
        Chili => {
            out.replace(AntiGravity, TropicThunder);
            out.replace(Athletic, Euphoric);
            out.replace(Laxative, LongFaced);
            out.replace(Munchies, Toxic);
            out.replace(Shrinking, Refreshing);
            out.replace(Sneaky, BrightEyed);
            out.replace(ThoughtProvoking, Focused);
        }
        Iodine => {
            out.replace(Calming, Balding);
            out.replace(CalorieDense, Gingeritis);
            out.replace(Euphoric, SeizureInducing);
            out.replace(Foggy, Paranoia);
            out.replace(Refreshing, ThoughtProvoking);
            out.replace(Toxic, Sneaky);
        }
        Paracetamol => {
            out.replace(Calming, Slippery);
            out.replace(Foggy, Calming);
            out.replace(Paranoia, Balding);
            out.replace(Energizing, Paranoia);
            out.replace(Electrifying, Athletic);
            out.replace(Focused, Gingeritis);
            out.replace(Glowing, Toxic);
            out.replace(Munchies, AntiGravity);
            out.replace(Spicy, BrightEyed);
            out.replace(Toxic, TropicThunder);
        }
        Viagra => {
            out.replace(Athletic, Sneaky);
            out.replace(Disorienting, Toxic);
            out.replace(Euphoric, BrightEyed);
            out.replace(Laxative, Calming);
            out.replace(Shrinking, Gingeritis);
        }
        HorseSemen => {
            out.replace(AntiGravity, Calming);
            out.replace(Gingeritis, Refreshing);
            out.replace(SeizureInducing, Energizing);
            out.replace(ThoughtProvoking, Electrifying);
        }
        MegaBean => {
            out.replace(Calming, Glowing);
            out.replace(Athletic, Laxative);
            out.replace(Sneaky, Calming);
            out.replace(Energizing, Cyclopean);
            out.replace(Focused, Disorienting);
            out.replace(Jennerising, Paranoia);
            out.replace(SeizureInducing, Focused);
            out.replace(Shrinking, Electrifying);
            out.replace(Slippery, Toxic);
            out.replace(ThoughtProvoking, Energizing);
        }
        Addy => {
            out.replace(Explosive, Euphoric);
            out.replace(Foggy, Energizing);
            out.replace(Glowing, Refreshing);
            out.replace(LongFaced, Electrifying);
            out.replace(Sedating, Gingeritis);
        }
        Battery => {
            out.replace(Cyclopean, Glowing);
            out.replace(Euphoric, Zombifying);
            out.replace(Electrifying, Euphoric);
            out.replace(Laxative, CalorieDense);
            out.replace(Munchies, TropicThunder);
            out.replace(Shrinking, Munchies);
        }
    }
}

pub fn mix<S, const N: usize>(m: Mixer, base: &S) -> S
where
    S: MySet<Effect, N>,
{
    let mut mixed = base.clone();
    mixer_transform(m, &mut mixed);
    if mixed.len() < N {
        mixed.insert(mixer_default(m));
    }
    return mixed;
}

pub fn get_price(base: u32, mult: u32) -> u32 {
    let n = base * (100 + mult);
    let d = 100;
    let r = if n % d < (d / 2) { 0 } else { 1 };
    (n / d) + r
}

pub fn get_profit<'a, S, const N: usize, const M: usize>(
    effects: &'a S,
    mixers: &[Option<Mixer>; M],
    base_value: u32,
) -> u32
where
    S: MySet<Effect, N>,
{
    let mult = get_effects_multiplier(effects);
    let cost = get_mixers_cost_arr(mixers);
    let value = get_price(base_value, mult);
    if value < cost { return 0 } else { value - cost }
}
