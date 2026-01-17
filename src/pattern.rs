use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

use crate::phoneme::{C, Pattern, V};

pub fn expand_pattern(
    p: Pattern,
    rng: &mut impl rand::Rng,
    consonants: &[C],
    c_dist: &WeightedIndex<u32>,
    vowels: &[V],
    v_dist: &WeightedIndex<u32>,
    onsets: &[(C, C)],
    onset_dist: &WeightedIndex<u32>,
    codas: &[C],
    coda_dist: &WeightedIndex<u32>,
) -> String {
    match p {
        Pattern::CV => {
            let c = consonants[c_dist.sample(rng)];
            let v = vowels[v_dist.sample(rng)];
            format!("{}{}", c.to_str(), v.to_str())
        }
        Pattern::CVC => {
            let c1 = consonants[c_dist.sample(rng)];
            let v = vowels[v_dist.sample(rng)];
            let c2 = codas[coda_dist.sample(rng)];
            format!("{}{}{}", c1.to_str(), v.to_str(), c2.to_str())
        }
        Pattern::CVV => {
            let c = consonants[c_dist.sample(rng)];
            let v = vowels[v_dist.sample(rng)];
            format!("{}{}{}", c.to_str(), v.to_str(), v.to_str())
        }
        Pattern::CVVC => {
            let c1 = consonants[c_dist.sample(rng)];
            let v = vowels[v_dist.sample(rng)];
            let c2 = codas[coda_dist.sample(rng)];
            format!("{}{}{}{}", c1.to_str(), v.to_str(), v.to_str(), c2.to_str())
        }
        Pattern::CCV => {
            let (c1, c2) = onsets[onset_dist.sample(rng)];
            let v = vowels[v_dist.sample(rng)];
            format!("{}{}{}", c1.to_str(), c2.to_str(), v.to_str())
        }
        Pattern::CCVC => {
            let (c1, c2) = onsets[onset_dist.sample(rng)];
            let v = vowels[v_dist.sample(rng)];
            let c3 = codas[coda_dist.sample(rng)];
            format!(
                "{}{}{}{}",
                c1.to_str(),
                c2.to_str(),
                v.to_str(),
                c3.to_str()
            )
        }
        Pattern::CCVV => {
            let (c1, c2) = onsets[onset_dist.sample(rng)];
            let v = vowels[v_dist.sample(rng)];
            format!("{}{}{}{}", c1.to_str(), c2.to_str(), v.to_str(), v.to_str())
        }
        Pattern::CCVVC => {
            let (c1, c2) = onsets[onset_dist.sample(rng)];
            let v = vowels[v_dist.sample(rng)];
            let c3 = codas[coda_dist.sample(rng)];
            format!(
                "{}{}{}{}{}",
                c1.to_str(),
                c2.to_str(),
                v.to_str(),
                v.to_str(),
                c3.to_str()
            )
        }
    }
}
