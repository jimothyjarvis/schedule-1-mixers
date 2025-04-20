use crate::my_set::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum Mixer {
    #[default]
    Cuke,
    FluMedicine,
    Gasoline,
    Donut,
    EnergyDrink,
    MouthWash,
    MotorOil,
    Banana,
    Chili,
    Iodine,
    Paracetamol,
    Viagra,
    HorseSemen,
    MegaBean,
    Addy,
    Battery,
}

use Mixer::*;

impl MySetItem for Mixer {}

pub const MIXERS: [Mixer; 16] = [
    Cuke,
    FluMedicine,
    Gasoline,
    Donut,
    EnergyDrink,
    MouthWash,
    MotorOil,
    Banana,
    Chili,
    Iodine,
    Paracetamol,
    Viagra,
    HorseSemen,
    MegaBean,
    Addy,
    Battery,
];


pub fn get_cost(m: Mixer) -> u32 {
    match m {
        Cuke => 2,
        FluMedicine => 5,
        Gasoline => 5,
        Donut => 3,
        EnergyDrink => 6,
        MouthWash => 4,
        MotorOil => 6,
        Banana => 2,
        Chili => 7,
        Iodine => 8,
        Paracetamol => 3,
        Viagra => 4,
        HorseSemen => 9,
        MegaBean => 7,
        Addy => 9,
        Battery => 8,
    }
}

pub fn get_mixers_cost_vec(mixers: &Vec<Mixer>) -> u32 {
    let mut cost = 0;
    for m in mixers {
        cost += get_cost(*m);
    }
    return cost;
}

pub fn get_mixers_cost_arr<const M: usize>(mixers: &[Option<Mixer>; M]) -> u32 {
    let mut cost = 0;
    for mo in mixers {
        match mo {
            Some (m) => cost += get_cost(*m),
            None => ()
        }
    }
    return cost;
}
