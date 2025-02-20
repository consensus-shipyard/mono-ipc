// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub struct Metrics {
    pub mean: f64,
    pub median: f64,
    pub max: f64,
    pub min: f64,
    pub percentile_90: f64,
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

impl Metrics {
    pub fn format_median(&self) -> String {
        format!("median: {:.2}", self.median)
    }
}

pub fn calc_metrics(data: Vec<f64>) -> Metrics {
    if data.is_empty() {
        return Metrics::default();
    }

    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let count = sorted_data.len();
    let mean: f64 = sorted_data.iter().sum::<f64>() / count as f64;

    let median = if count % 2 == 0 {
        (sorted_data[count / 2 - 1] + sorted_data[count / 2]) / 2.0
    } else {
        sorted_data[count / 2]
    };

    let max = *sorted_data.last().unwrap();
    let min = *sorted_data.first().unwrap();

    let percentile_90_index = ((count as f64) * 0.9).ceil() as usize - 1;
    let percentile_90 = sorted_data[percentile_90_index];

    Metrics {
        mean,
        median,
        max,
        min,
        percentile_90,
    }
}

#[cfg(test)]
mod tests {
    use super::super::FLOAT_TOLERANCE;
    use super::*;

    #[test]
    fn test_calc_dataset_metrics() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

        let expected_mean = 5.5;
        let expected_median = 5.5;
        let expected_max = 10.0;
        let expected_min = 1.0;
        let expected_percentile_90 = 9.0;

        let metrics = calc_metrics(data);

        assert!((metrics.mean - expected_mean).abs() < FLOAT_TOLERANCE);
        assert!((metrics.median - expected_median).abs() < FLOAT_TOLERANCE);
        assert!((metrics.max - expected_max).abs() < FLOAT_TOLERANCE);
        assert!((metrics.min - expected_min).abs() < FLOAT_TOLERANCE);
        assert!((metrics.percentile_90 - expected_percentile_90).abs() < FLOAT_TOLERANCE);
    }
}
