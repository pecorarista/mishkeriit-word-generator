use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;

mod pattern;
mod phoneme;
mod rewrite;

use phoneme::{C, Pattern, V};

pub fn build_weighted_index<T: Copy>(items: &[(T, u32)]) -> (Vec<T>, WeightedIndex<u32>) {
    let values: Vec<T> = items.iter().map(|(v, _)| *v).collect();
    let weights: Vec<u32> = items.iter().map(|(_, w)| *w).collect();
    let dist = WeightedIndex::new(&weights).unwrap_or_else(|e| panic!("invalid weights: {:?}", e));
    (values, dist)
}

fn generate_word(
    n_syllables: usize,
    rng: &mut impl rand::Rng,
    patterns: &[Pattern],
    pattern_dist: &rand::distr::weighted::WeightedIndex<u32>,
    consonants: &[C],
    consonant_dist: &rand::distr::weighted::WeightedIndex<u32>,
    vowels: &[V],
    vowel_dist: &rand::distr::weighted::WeightedIndex<u32>,
    onset_ccs: &[(C, C)],
    onset_cc_dist: &rand::distr::weighted::WeightedIndex<u32>,
    codas: &[C],
    coda_dist: &rand::distr::weighted::WeightedIndex<u32>,
) -> String {
    let mut syllables = Vec::with_capacity(n_syllables);
    for _ in 0..n_syllables {
        let p = patterns[pattern_dist.sample(rng)];
        let raw_syllable = pattern::expand_pattern(
            p,
            rng,
            consonants,
            consonant_dist,
            vowels,
            vowel_dist,
            onset_ccs,
            onset_cc_dist,
            codas,
            coda_dist,
        );
        let syllable = rewrite::apply_all(&raw_syllable);
        syllables.push(syllable);
    }
    syllables.join(".")
}

fn main() {
    const PATTERN_WEIGHTS: &[(Pattern, u32)] = &[
        (Pattern::CV, 30),
        (Pattern::CVC, 25),
        (Pattern::CVV, 10),
        (Pattern::CVVC, 8),
        (Pattern::CCV, 12),
        (Pattern::CCVC, 8),
        (Pattern::CCVV, 4),
        (Pattern::CCVVC, 3),
    ];

    #[rustfmt::skip]
    const CONSONANT_WEIGHTS: &[(C, u32)] = &[
        (C::M, 5), (C::N, 5),
        (C::P, 8), (C::T, 8), (C::K, 8),
        (C::B, 6), (C::D, 6), (C::G, 6),
        (C::F, 4), (C::S, 6), (C::Š, 4), (C::X, 3), (C::H, 3),
        (C::Z, 4), (C::Ž, 3),
        (C::C, 3), (C::Č, 3),
        (C::Q, 6), (C::R, 6), (C::L, 6), (C::J, 4), (C::W, 4), (C::Ğ, 2)
    ];

    #[rustfmt::skip]
    const VOWEL_WEIGHTS: &[(V, u32)] = &[
        (V::I, 5),
        (V::U, 5),
        (V::E, 5),
        (V::O, 5),
        (V::A, 5)
    ];

    #[rustfmt::skip]
    const ONSET_CC_WEIGHTS: &[( (C, C), u32 )] = &[
        // Cr: (p | t | k | b | d | g )r
        ((C::P, C::R), 5),
        ((C::T, C::R), 5),
        ((C::K, C::R), 5),
        ((C::B, C::R), 4),
        ((C::D, C::R), 4),
        ((C::G, C::R), 4),
        // Ct: (s | š)t
        ((C::S, C::T), 3),
        ((C::Š, C::T), 3),
        // Cw: (m | n | p | t | k s | š)w
        ((C::M, C::W), 3),
        ((C::N, C::W), 3),
        ((C::P, C::W), 3),
        ((C::T, C::W), 3),
        ((C::K, C::W), 3),
        ((C::S, C::W), 3),
        ((C::Š, C::W), 3),
    ];

    const CODA_WEIGHTS: &[(C, u32)] = &[(C::M, 5), (C::N, 5), (C::R, 4), (C::L, 4), (C::Ğ, 2)];

    const SYLLABLE_COUNT_WEIGHTS: &[(usize, u32)] = &[(1, 3), (2, 5), (3, 6), (4, 2)];
    let (ns, n_dist) = build_weighted_index(SYLLABLE_COUNT_WEIGHTS);
    let (patterns, pattern_dist) = build_weighted_index(PATTERN_WEIGHTS);
    let (consonants, consonant_dist) = build_weighted_index(CONSONANT_WEIGHTS);
    let (vowels, vowel_dist) = build_weighted_index(VOWEL_WEIGHTS);
    let (onset_ccs, onset_cc_dist) = build_weighted_index(ONSET_CC_WEIGHTS);
    let (codas, coda_dist) = build_weighted_index(CODA_WEIGHTS);

    let mut rng = rand::rng();
    for _ in 0..20 {
        let n = ns[n_dist.sample(&mut rng)];
        let word = generate_word(
            n,
            &mut rng,
            &patterns,
            &pattern_dist,
            &consonants,
            &consonant_dist,
            &vowels,
            &vowel_dist,
            &onset_ccs,
            &onset_cc_dist,
            &codas,
            &coda_dist,
        );
        println!("{}", word);
    }
}
