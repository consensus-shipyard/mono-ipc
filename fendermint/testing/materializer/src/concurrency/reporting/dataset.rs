// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use std::fmt::{Display, Formatter};
use statrs::statistics::{Data, OrderStatistics, Distribution, Max, Min};

#[derive(Debug, Default)]
pub struct Metrics {
    pub mean: f64,
    pub median: f64,
    pub max: f64,
    pub min: f64,
    pub percentile_90: f64,
}

impl From<Vec<f64>> for Metrics {
    fn from(mut data: Vec<f64>) -> Self {
        if data.is_empty() {
            return Metrics::default();
        }

        data.sort_by(|a, b| a.partial_cmp(b).unwrap()); // Sort once before using Data
        let mut data = Data::new(data);

        Metrics {
            mean: data.mean().unwrap(),
            median: data.median(),
            max: data.max(),
            min: data.min(),
            percentile_90: data.percentile(90),
        }
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mean: {:.2}, median: {:.2}, max: {:.2}, min: {:.2}, 90th: {:.2}",
            self.mean, self.median, self.max, self.min, self.percentile_90
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::FLOAT_TOLERANCE;
    use super::*;

    #[test]
    fn test_metrics() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

        let expected_mean = 5.5;
        let expected_median = 5.5;
        let expected_max = 10.0;
        let expected_min = 1.0;
        let expected_percentile_90 = 9.0;

        let metrics: Metrics = data.into();

        assert!((metrics.mean - expected_mean).abs() < FLOAT_TOLERANCE);
        assert!((metrics.median - expected_median).abs() < FLOAT_TOLERANCE);
        assert!((metrics.max - expected_max).abs() < FLOAT_TOLERANCE);
        assert!((metrics.min - expected_min).abs() < FLOAT_TOLERANCE);
        assert!((metrics.percentile_90 - expected_percentile_90).abs() < FLOAT_TOLERANCE);
    }
}
