use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

use crate::phoneme::{C, Pattern, V, VVKind};

fn sample_vv(
    rng: &mut impl rand::Rng,
    vowels: &[V],
    vowel_dist: &WeightedIndex<u32>,
    vv_kinds: &[VVKind],
    vv_kind_dist: &WeightedIndex<u32>
) -> (V, V) {
    match vv_kinds[vv_kind_dist.sample(rng)] {
        VVKind::Long => {
            let v = vowels[vowel_dist.sample(rng)];
            (v, v)
        }
        VVKind::Ai => (V::A, V::I),
        VVKind::Au => (V::A, V::U)
    }
}

pub fn expand_pattern(
    p: Pattern,
    rng: &mut impl rand::Rng,
    consonants: &[C],
    c_dist: &WeightedIndex<u32>,
    vowels: &[V],
    vowel_dist: &WeightedIndex<u32>,
    vv_kinds: &[VVKind],
    vv_kind_dist: &WeightedIndex<u32>,
    onsets: &[(C, C)],
    onset_dist: &WeightedIndex<u32>,
    codas: &[C],
    coda_dist: &WeightedIndex<u32>
) -> String {
    match p {
        Pattern::CV => {
            let c = consonants[c_dist.sample(rng)];
            let v = vowels[vowel_dist.sample(rng)];
            format!("{}{}", c.to_str(), v.to_str())
        }
        Pattern::CVC => {
            let c1 = consonants[c_dist.sample(rng)];
            let v = vowels[vowel_dist.sample(rng)];
            let c2 = codas[coda_dist.sample(rng)];
            format!("{}{}{}", c1.to_str(), v.to_str(), c2.to_str())
        }
        Pattern::CVV => {
            let c = consonants[c_dist.sample(rng)];
            let (v1, v2) = sample_vv(rng, vowels, vowel_dist, vv_kinds, vv_kind_dist);
            format!("{}{}{}", c.to_str(), v1.to_str(), v2.to_str())
        }
        Pattern::CVVC => {
            let c1 = consonants[c_dist.sample(rng)];
            let (v1, v2) = sample_vv(rng, vowels, vowel_dist, vv_kinds, vv_kind_dist);
            let c2 = codas[coda_dist.sample(rng)];
            format!(
                "{}{}{}{}",
                c1.to_str(),
                v1.to_str(),
                v2.to_str(),
                c2.to_str()
            )
        }
        Pattern::CCV => {
            let (c1, c2) = onsets[onset_dist.sample(rng)];
            let v = vowels[vowel_dist.sample(rng)];
            format!("{}{}{}", c1.to_str(), c2.to_str(), v.to_str())
        }
        Pattern::CCVC => {
            let (c1, c2) = onsets[onset_dist.sample(rng)];
            let v = vowels[vowel_dist.sample(rng)];
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
            let (v1, v2) = sample_vv(rng, vowels, vowel_dist, vv_kinds, vv_kind_dist);
            format!(
                "{}{}{}{}",
                c1.to_str(),
                c2.to_str(),
                v1.to_str(),
                v2.to_str()
            )
        }
        Pattern::CCVVC => {
            let (c1, c2) = onsets[onset_dist.sample(rng)];
            let (v1, v2) = sample_vv(rng, vowels, vowel_dist, vv_kinds, vv_kind_dist);
            let c3 = codas[coda_dist.sample(rng)];
            format!(
                "{}{}{}{}{}",
                c1.to_str(),
                c2.to_str(),
                v1.to_str(),
                v2.to_str(),
                c3.to_str()
            )
        }
    }
}
