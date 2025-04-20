use crate::effects::*;
use crate::mixers::*;
use crate::my_set::*;
use crate::mixing::*;
use std::thread;

fn find_max_inner<S, const N: usize, const M: usize>(
    effects: S,
    // Invariant: All non None elements are before all None elements
    mixers: [Option<Mixer>; M],
    // Invariant: The first None element in mixers is at mixers[first_mixer_none], or all elements
    // in Mixers are Some(...) and first_mixer_none == M
    first_mixer_none: usize,
    base_value: u32,
) -> (S, [Option<Mixer>; M], u32)
where
    S: MySet<Effect, N>,
    S: Sync,
    S: Send,
{
    if first_mixer_none >= M {
        // We've reached max number of mixers we can add, so we return
        let profit = get_profit(&effects, &mixers, base_value);
        return (effects, mixers, profit);
    } else {
        // We can still add mixers, so we find the max for each possible mixer recursively
        let mix_maxes;
        let find_max_recur = |m, effects, mixers: [Option<Mixer>; M]| {
            let m_mix = mix(m, effects);
            let mut m_arr = mixers;
            m_arr[first_mixer_none] = Some(m);
            // We've added a mixer, so we increment first_mixer_none
            find_max_inner::<S, N, M>(m_mix, m_arr, first_mixer_none + 1, base_value)
        };
        // In order to make runtime a bit more reasonable, we sometimes spawn a thread for each
        // mixer. The computation is completely independent, and we require a join before iterating
        // to calculate the max found
        //
        // Setting this bound below determines when threads get spawned, with higher numbers
        // spawning more threads. In general, setting the right hand side bound to x will result in
        // M^x threads, where M is the number of mixers. Adjust according to system, for my setup 0,
        // which corresponds to 16 threads, was ideal
        if first_mixer_none <= 0 {
            // Scoped so we can capture a borrow in the thread
            mix_maxes = thread::scope(|s| {
                let mix_threads = MIXERS.map(|m| {
                    let effects_inner = &effects;
                    thread::Scope::spawn(s, move || {
                        find_max_recur(m, effects_inner, mixers.clone())
                    })
                });
                // Guaranteed to join here, but the typechecker only knows a join will definitely
                // happen because of the thread::scope
                mix_threads.map(|handle| handle.join().unwrap())
            })
        } else {
            mix_maxes = MIXERS.map(|m| {
                find_max_recur(m, &effects, mixers.clone())
            })
        }
        let mut largest = (S::new(), [None; M], 0);
        for mresults in mix_maxes {
            if mresults.2 > largest.2 {
                largest = mresults
            }
        }
        return largest;
    }
}

pub fn find_max<S, const N: usize, const M: usize>(
    effects: S,
    base_value: u32,
) -> (S, [Option<Mixer>; M], u32)
where
    S: MySet<Effect, N>,
    S: Sync,
    S: Send,
{
    let mixers = [None; M];
    find_max_inner(effects, mixers, 0, base_value)
}
