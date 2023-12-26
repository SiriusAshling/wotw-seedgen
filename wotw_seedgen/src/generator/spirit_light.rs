use super::SEED_FAILED_MESSAGE;
use rand::{distributions::Uniform, Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::mem;

const MIN_SPIRIT_LIGHT: f32 = 50.;

pub struct SpiritLightProvider {
    rng: Pcg64Mcg,
    amount: f32,
    next_amount: f32,
    noise: Uniform<f32>,
}
impl SpiritLightProvider {
    pub fn new(amount: i32, rng: &mut Pcg64Mcg) -> Self {
        Self {
            rng: Pcg64Mcg::from_rng(rng).expect(SEED_FAILED_MESSAGE),
            amount: amount as f32,
            next_amount: MIN_SPIRIT_LIGHT,
            noise: Uniform::new_inclusive(0.75, 1.25),
        }
    }

    pub fn take(&mut self, placements_remaining: usize) -> usize {
        // We want spirit_light(placements_remaining) = a * x + b
        // And spirit_light(placements_remaining) = self.next_amount
        // And ∫₁ˢˡᵒᵗˢʳᵉᵐᵃᶦⁿᶦⁿᵍ spirit_light dx = self.amount
        // And next = spirit_light(placements_remaining - 1)
        //
        // So spirit_light(placements_remaining) = a * placements_remaining + b = self.next_amount
        // ... b = self.next_amount - a * placements_remaining
        // And ∫₁ˢˡᵒᵗˢʳᵉᵐᵃᶦⁿᶦⁿᵍ spirit_light dx = 1/2 * (placements_remaining - 1) * (a * placements_remaining + a + 2 * b) = self.amount
        // ... a * placements_remaining + a + 2 * b = 2 * self.amount / (placements_remaining - 1)
        // ... a * placements_remaining + a + 2 * self.next_amount - 2 * a * placements_remaining = 2 * self.amount / (placements_remaining - 1)
        // ... a * (placements_remaining + 1 - 2 * placements_remaining) = 2 * self.amount / (placements_remaining - 1) - 2 * self.next_amount
        // ... a = (2 * self.amount / (placements_remaining - 1) - 2 * self.next_amount) / (placements_remaining + 1 - 2 * placements_remaining)

        let placements_remaining = placements_remaining as f32;
        let a = (2. * self.amount / (placements_remaining - 1.) - 2. * self.next_amount)
            / (placements_remaining + 1. - 2. * placements_remaining);
        let b = self.next_amount - a * placements_remaining;
        let next = (a * (placements_remaining - 1.) + b) * self.rng.sample(self.noise);
        self.amount -= self.next_amount;
        mem::replace(&mut self.next_amount, next).round() as usize
    }
}
