#[macro_export]
macro_rules! parse_any_precision_variant_from_be_slice {
    ($typ:ident, $precision:ident, $bytes:ident) => {
        match $precision {
            Precision::Float32 => {
                let _ = $typ::<f32>::from_be_slice($bytes);
            }
            Precision::Float64 => {
                let _ = $typ::<f64>::from_be_slice($bytes);
            }
            Precision::Fp1220 => {
                let _ = $typ::<u32>::from_be_slice($bytes);
            }
            Precision::Fp1632 => {
                let _ = $typ::<u64>::from_be_slice($bytes);
            }
            Precision::Unknown(_) => {}
        }
    };
}

#[macro_export]
macro_rules! parse_float_precision_variant_from_be_slice {
    ($typ:ident, $precision:ident, $bytes:ident) => {
        match $precision {
            Precision::Float32 => {
                let _ = $typ::<f32>::from_be_slice($bytes);
            }
            Precision::Float64 => {
                let _ = $typ::<f64>::from_be_slice($bytes);
            }
            Precision::Fp1220 | Precision::Fp1632 => {}
            Precision::Unknown(_) => {}
        }
    };
}
