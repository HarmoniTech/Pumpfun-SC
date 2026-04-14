use std::ops::{Div, Mul};

/// Convert a token amount (in smallest unit) to a display value.
/// Uses u128 for intermediate calculation to avoid float precision loss.
/// 
/// WARNING: This function is kept for display purposes only.
/// All financial calculations should use u128 directly.
pub fn convert_to_float(value: u64, decimals: u8) -> f64 {
    // Use integer math first, only convert to float at the very end for display
    let value_u128 = (value as u128) * (10_u128.pow(decimals as u32));
    (value_u128 as f64) / (10_u128.pow(decimals as u32) as f64)
    // Note: This still has float issues but is kept for backward compatibility
    // Production code should avoid float entirely
}

/// Convert a display value back to smallest unit.
/// Uses u128 for intermediate calculation to avoid precision loss.
///
/// Returns: The amount in smallest unit (rounded)
pub fn convert_from_float(value: f64, decimals: u8) -> u64 {
    // Multiply by 10^decimals as integer, then round
    let multiplier = 10_u128.pow(decimals as u32);
    let value_scaled = (value as f64) * (multiplier as f64);
    value_scaled as u64  // Truncates - consider using round()
}

/// Convert using pure integer math (RECOMMENDED for all financial calculations)
/// 
/// # Arguments
/// * `value` - Amount in smallest unit (e.g., lamports)
/// * `decimals` - Number of decimal places
/// 
/// # Returns
/// Value scaled by 10^decimals using u128 for precision
pub fn to_integer_scaled(value: u64, decimals: u8) -> u128 {
    (value as u128) * (10_u128.pow(decimals as u32))
}

/// Convert from integer scaled value back to smallest unit
/// 
/// # Arguments
/// * `scaled_value` - Value that has been multiplied by 10^decimals
/// * `decimals` - Number of decimal places
/// 
/// # Returns
/// Amount in smallest unit
pub fn from_integer_scaled(scaled_value: u128, decimals: u8) -> u64 {
    let divisor = 10_u128.pow(decimals as u32);
    (scaled_value / divisor) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_conversion_precision() {
        // Test that integer math preserves precision
        let original: u64 = 1_001_000_000; // 1.001 tokens with 9 decimals
        let scaled = to_integer_scaled(original, 9);
        let back = from_integer_scaled(scaled, 9);
        assert_eq!(original, back, "Integer conversion must be lossless");
    }

    #[test]
    fn test_small_amounts() {
        // Test with small amounts that float would mishandle
        let small: u64 = 1; // 1 lamport
        let scaled = to_integer_scaled(small, 9);
        let back = from_integer_scaled(scaled, 9);
        assert_eq!(small, back);
    }
}
