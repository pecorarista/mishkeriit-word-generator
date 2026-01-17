#[macro_export]
macro_rules! phoneme_enum {
    ($name:ident { $($v:ident),+ $(,)? }) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum $name {
            $($v),+
        }

        impl $name {
            pub fn to_str(self) -> &'static str {
                match self {
                    $(Self::$v => stringify!($v)),+
                }
            }
        }
    }
}

#[rustfmt::skip]
phoneme_enum!(C {
    M, N,
    P, T, K,
    B, D, G,
    F, S, Š, X, H,
    Z, Ž,
    C, Č,
    Q, R, L, J, W, Ğ
});

phoneme_enum!(V { I, U, E, O, A });

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Pattern {
    CV,
    CVC,
    CVV,
    CVVC,
    CCV,
    CCVC,
    CCVV,
    CCVVC
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VVKind {
    Long,
    Ai,
    Au
}
