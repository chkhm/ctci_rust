/// Utility functions for GPS calculations

/// Calculates the distance between two GPS coordinates using the Haversine formula.
///
/// # Arguments
///
/// * `lat1` - Latitude of the first point in degrees
/// * `lon1` - Longitude of the first point in degrees
/// * `lat2` - Latitude of the second point in degrees
/// * `lon2` - Longitude of the second point in degrees
///
/// # Returns
///
/// The distance between the two points in meters.
///
/// # Example
///
/// ```
/// let distance = calculate_distance(52.5200, 13.4050, 48.8566, 2.3522);
/// println!("Distance: {} meters", distance);
/// ```
pub fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    // Earth radius in meters
    const EARTH_RADIUS: f64 = 6371000.0;

    // Convert degrees to radians
    let lat1_rad = lat1.to_radians();
    let lon1_rad = lon1.to_radians();
    let lat2_rad = lat2.to_radians();
    let lon2_rad = lon2.to_radians();

    // Differences between coordinates
    let delta_lat = lat2_rad - lat1_rad;
    let delta_lon = lon2_rad - lon1_rad;

    // Haversine formula
    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    // Calculate distance
    EARTH_RADIUS * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        // Paris (48.8566° N, 2.3522° E) to Berlin (52.5200° N, 13.4050° E)
        // Expected distance: approximately 876 km (876000 meters)
        let distance = calculate_distance(48.8566, 2.3522, 52.5200, 13.4050);
        assert!((distance - 876000.0).abs() < 5000.0, "Distance should be approximately 876 km");

        // Same point should return 0
        let distance_same_point = calculate_distance(45.0, 45.0, 45.0, 45.0);
        assert!(distance_same_point < 0.000001, "Distance between same points should be 0");
    }
}

