trait Method {
    fn name(&self) -> String;
    fn is_normalized(&self) -> bool;
    fn distance(&self, reference: &str, value: &str) -> f32;

    fn normalize(&self, value: f32, max: f32) -> f32 {
        if self.is_normalized() {
            value
        } else {
            value / max
        }
    }
}

pub enum Distance {
    Normalized(f64),
    Unnormalized(usize),
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

pub enum Methods {
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

impl Methods {
    pub fn calculate(&self, s1: &str, s2: &str) -> Distance {
        use textdistance::str::*;

        match self {
            Methods::DamerauLevenshtein => damerau_levenshtein(s1, s2).into(),
            Methods::Hamming => hamming(s1, s2).into(),
            Methods::Jaro => jaro(s1, s2).into(),
            Methods::JaroWinkler => jaro_winkler(s1, s2).into(),
            Methods::Levenshtein => levenshtein(s1, s2).into(),
            Methods::Sift4Common => sift4_common(s1, s2).into(),
            Methods::Sift4Simple => sift4_simple(s1, s2).into(),
            Methods::SmithWaterman => smith_waterman(s1, s2).into(),
            Methods::Bag => bag(s1, s2).into(),
            Methods::Cosine => cosine(s1, s2).into(),
            Methods::EntropyNCD => entropy_ncd(s1, s2).into(),
            Methods::Jaccard => jaccard(s1, s2).into(),
            Methods::Overlap => overlap(s1, s2).into(),
            Methods::Roberts => roberts(s1, s2).into(),
            Methods::SorensenDice => sorensen_dice(s1, s2).into(),
            Methods::Tversky => tversky(s1, s2).into(),
            Methods::LCSSeq => lcsseq(s1, s2).into(),
            Methods::LCSStr => lcsstr(s1, s2).into(),
            Methods::RatcliffObershelp => ratcliff_obershelp(s1, s2).into(),
            Methods::Prefix => prefix(s1, s2).into(),
            Methods::Suffix => suffix(s1, s2).into(),
            Methods::Length => length(s1, s2).into(),
        }
    }

    pub fn all() -> [Methods; 22] {
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

    pub fn unnormalized() -> impl Iterator<Item = Methods> {
        Self::all()
            .into_iter()
            .filter(|x| !x.is_normalized())
    }

    pub fn normalized() -> impl Iterator<Item = Methods> {
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
            Methods::DamerauLevenshtein => norm(damerau_levenshtein),
            Methods::Hamming => norm(hamming),
            Methods::Jaro => norm(jaro),
            Methods::JaroWinkler => norm(jaro_winkler),
            Methods::Levenshtein => norm(levenshtein),
            Methods::Sift4Common => norm(sift4_common),
            Methods::Sift4Simple => norm(sift4_simple),
            Methods::SmithWaterman => norm(smith_waterman),
            Methods::Bag => norm(bag),
            Methods::Cosine => norm(cosine),
            Methods::EntropyNCD => norm(entropy_ncd),
            Methods::Jaccard => norm(jaccard),
            Methods::Overlap => norm(overlap),
            Methods::Roberts => norm(roberts),
            Methods::SorensenDice => norm(sorensen_dice),
            Methods::Tversky => norm(tversky),
            Methods::LCSSeq => norm(lcsseq),
            Methods::LCSStr => norm(lcsstr),
            Methods::RatcliffObershelp => norm(ratcliff_obershelp),
            Methods::Prefix => norm(prefix),
            Methods::Suffix => norm(suffix),
            Methods::Length => norm(length),
        }

    }
} 

impl From<String> for Methods {
    fn from(value: String) -> Self {
        todo!()
    }
}

impl From<Methods> for String {
    fn from(value: Methods) -> Self {
        todo!()
    }
}
