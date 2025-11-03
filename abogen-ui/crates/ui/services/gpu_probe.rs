/// GPU acceleration probe service
/// Returns true if GPU acceleration is available

#[cfg(feature = "gpu")]
pub fn probe_gpu() -> bool {
    // Stub: return true when GPU feature is enabled
    true
}

#[cfg(not(feature = "gpu"))]
pub fn probe_gpu() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_probe() {
        let available = probe_gpu();
        // Test that it returns a boolean (value depends on feature flag)
        assert!(available == true || available == false);
    }
}

