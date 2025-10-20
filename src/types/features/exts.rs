use easy_ext::ext;
use enumflags2::BitFlags;

use super::EcFeature;

pub type EcFeatures = BitFlags<EcFeature>;

const fn u64_from_u32s(v: [u32; 2]) -> u64 {
    (v[0] as u64) | (v[1] as u64) << 32
}

pub const fn ec_features_from_u32s(v: [u32; 2]) -> EcFeatures {
    EcFeatures::from_bits_truncate_c(u64_from_u32s(v), BitFlags::CONST_TOKEN)
}

#[ext(EcFeaturesExt)]
pub impl EcFeatures {
    fn from_u32s(v: [u32; 2]) -> Self {
        ec_features_from_u32s(v)
    }
}
