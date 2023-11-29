use wasm_bindgen::prelude::wasm_bindgen;

pub enum Distance {
    Normalized(f64),
    Unnormalized(usize),
}

impl Distance {
    pub fn normalize(&self, max: usize) -> f64 {
        match *self {
            Distance::Normalized(x) => x,
            Distance::Unnormalized(x) => x as f64 / max as f64,
        }
    }

    pub fn as_f64(&self) -> f64 {
        match *self {
            Distance::Normalized(x) => x,
            Distance::Unnormalized(x) => x as f64,
        }
    }
}

impl From<f64> for Distance {
    fn from(value: f64) -> Self {
        Self::Normalized(value)
    }
}

impl From<usize> for Distance {
    fn from(value: usize) -> Self {
        Self::Unnormalized(value)
    }
}

pub enum Method {
    //Edit-based
    DamerauLevenshtein, //both optimal string alignment and restricted.
    Hamming,
    Jaro,
    JaroWinkler,
    Levenshtein,
    Sift4Common,
    Sift4Simple,
    SmithWaterman,

    //Token-based:
    Bag,
    Cosine,     //(aka Orchini, Tucker, Otsuka–Ochiai)
    EntropyNCD, //(Entropy-based Normalized Compression Distance)
    Jaccard,    //(aka Tanimoto, Critical Success Index)
    Overlap,    //(aka Szymkiewicz–Simpson)
    Roberts,
    SorensenDice, //(aka F1, Czekanowski, Zijdenbos)
    Tversky,

    //Sequence-based:
    LCSSeq,            //(Longest Common SubSequence)
    LCSStr,            //(Longest Common SubString)
    RatcliffObershelp, //(aka Gestalt pattern matching)

    //Naive:
    Prefix,
    Suffix,
    Length,
}

impl Method {
    pub fn calculate(&self, s1: &str, s2: &str) -> Distance {
        use textdistance::str::*;

        match self {
            Method::DamerauLevenshtein => damerau_levenshtein(s1, s2).into(),
            Method::Hamming => hamming(s1, s2).into(),
            Method::Jaro => jaro(s1, s2).into(),
            Method::JaroWinkler => jaro_winkler(s1, s2).into(),
            Method::Levenshtein => levenshtein(s1, s2).into(),
            Method::Sift4Common => sift4_common(s1, s2).into(),
            Method::Sift4Simple => sift4_simple(s1, s2).into(),
            Method::SmithWaterman => smith_waterman(s1, s2).into(),
            Method::Bag => bag(s1, s2).into(),
            Method::Cosine => cosine(s1, s2).into(),
            Method::EntropyNCD => entropy_ncd(s1, s2).into(),
            Method::Jaccard => jaccard(s1, s2).into(),
            Method::Overlap => overlap(s1, s2).into(),
            Method::Roberts => roberts(s1, s2).into(),
            Method::SorensenDice => sorensen_dice(s1, s2).into(),
            Method::Tversky => tversky(s1, s2).into(),
            Method::LCSSeq => lcsseq(s1, s2).into(),
            Method::LCSStr => lcsstr(s1, s2).into(),
            Method::RatcliffObershelp => ratcliff_obershelp(s1, s2).into(),
            Method::Prefix => prefix(s1, s2).into(),
            Method::Suffix => suffix(s1, s2).into(),
            Method::Length => length(s1, s2).into(),
        }
    }

    pub fn all() -> [Method; 22] {
        [
            Self::DamerauLevenshtein,
            Self::Hamming,
            Self::Jaro,
            Self::JaroWinkler,
            Self::Levenshtein,
            Self::Sift4Common,
            Self::Sift4Simple,
            Self::SmithWaterman,
            Self::Bag,
            Self::Cosine,
            Self::EntropyNCD,
            Self::Jaccard,
            Self::Overlap,
            Self::Roberts,
            Self::SorensenDice,
            Self::Tversky,
            Self::LCSSeq,
            Self::LCSStr,
            Self::RatcliffObershelp,
            Self::Prefix,
            Self::Suffix,
            Self::Length,
        ]
    }

    pub fn unnormalized() -> impl Iterator<Item = Method> {
        Self::all()
            .into_iter()
            .filter(|x| !x.is_normalized())
    }

    pub fn normalized() -> impl Iterator<Item = Method> {
        Self::all()
            .into_iter()
            .filter(|x| x.is_normalized())
    }

    pub fn is_normalized(&self) -> bool {
        use textdistance::str::*;

        fn norm<T: 'static>(_: impl FnOnce(&str, &str) -> T) -> bool {
            use std::any::TypeId;
            let id = TypeId::of::<T>();
            id == TypeId::of::<f64>()
        }

        match self {
            Method::DamerauLevenshtein => norm(damerau_levenshtein),
            Method::Hamming => norm(hamming),
            Method::Jaro => norm(jaro),
            Method::JaroWinkler => norm(jaro_winkler),
            Method::Levenshtein => norm(levenshtein),
            Method::Sift4Common => norm(sift4_common),
            Method::Sift4Simple => norm(sift4_simple),
            Method::SmithWaterman => norm(smith_waterman),
            Method::Bag => norm(bag),
            Method::Cosine => norm(cosine),
            Method::EntropyNCD => norm(entropy_ncd),
            Method::Jaccard => norm(jaccard),
            Method::Overlap => norm(overlap),
            Method::Roberts => norm(roberts),
            Method::SorensenDice => norm(sorensen_dice),
            Method::Tversky => norm(tversky),
            Method::LCSSeq => norm(lcsseq),
            Method::LCSStr => norm(lcsstr),
            Method::RatcliffObershelp => norm(ratcliff_obershelp),
            Method::Prefix => norm(prefix),
            Method::Suffix => norm(suffix),
            Method::Length => norm(length),
        }

    }

    pub fn name(&self) -> &'static str {
        match self {
            Method::DamerauLevenshtein => "Damerau Levenshtein",
            Method::Hamming => "Hamming",
            Method::Jaro => "Jaro",
            Method::JaroWinkler => "Jaro Winkler",
            Method::Levenshtein => "Levenshtein",
            Method::Sift4Common => "Sift4 Common",
            Method::Sift4Simple => "Sift4 Simple",
            Method::SmithWaterman => "Smith Waterman",
            Method::Bag => "Bag",
            Method::Cosine => "Cosine",
            Method::EntropyNCD => "Entropy NCD",
            Method::Jaccard => "Jaccard",
            Method::Overlap => "Overlap",
            Method::Roberts => "Roberts",
            Method::SorensenDice => "Sorensen Dice",
            Method::Tversky => "Tversky",
            Method::LCSSeq => "Common Subsequence",
            Method::LCSStr => "Common Substring",
            Method::RatcliffObershelp => "Ratcliff Obershelp",
            Method::Prefix => "Prefix",
            Method::Suffix => "Suffix",
            Method::Length => "Length",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Method::DamerauLevenshtein => "damerau_levenshtein",
            Method::Hamming => "hamming",
            Method::Jaro => "jaro",
            Method::JaroWinkler => "jaro_winkler",
            Method::Levenshtein => "levenshtein",
            Method::Sift4Common => "sift4_common",
            Method::Sift4Simple => "sift4_simple",
            Method::SmithWaterman => "smith_waterman",
            Method::Bag => "bag",
            Method::Cosine => "cosine",
            Method::EntropyNCD => "entropy_n_c_d",
            Method::Jaccard => "jaccard",
            Method::Overlap => "overlap",
            Method::Roberts => "roberts",
            Method::SorensenDice => "sorensen_dice",
            Method::Tversky => "tversky",
            Method::LCSSeq => "lcs_seq",
            Method::LCSStr => "lcs_str",
            Method::RatcliffObershelp => "ratcliff_obershelp",
            Method::Prefix => "prefix",
            Method::Suffix => "suffix",
            Method::Length => "length",
        }
    }
} 

impl From<String> for Method {
    fn from(value: String) -> Self {
        todo!()
    }
}

impl From<Method> for String {
    fn from(value: Method) -> Self {
        todo!()
    }
}

#[wasm_bindgen]
pub struct NamedDistance {
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
    pub distance: f64,
}
impl NamedDistance {
    pub fn new(m: Method, s1: &str, s2: &str) -> Self {
        Self {
            name: m.name().into(),
            distance: m.calculate(s1, s2).as_f64(),
        }
    }

    pub fn new_normalized(m: Method, s1: &str, s2: &str) -> Self {
        Self {
            name: m.name().into(),
            distance: m.calculate(s1, s2).normalize(s2.len()),
        }
    }
}

#[wasm_bindgen]
pub fn calculate_all(s1: &str, s2: &str) -> Vec<NamedDistance> {
    Method::all()
        .into_iter()
        .map(|x| NamedDistance::new_normalized(x, s1, s2))
        .collect()
}

#[wasm_bindgen]
pub fn calculate_unnormalized(s1: &str, s2: &str) -> Vec<NamedDistance> {
    Method::unnormalized()
        .map(|x| NamedDistance::new(x, s1, s2))
        .collect()
}

#[wasm_bindgen]
pub fn calculate_normalized(s1: &str, s2: &str) -> Vec<NamedDistance> {
    Method::normalized()
        .map(|x| NamedDistance::new(x, s1, s2))
        .collect()
}
