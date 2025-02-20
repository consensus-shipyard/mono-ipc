// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use ethers::prelude::{Block, H256, U256};

pub fn calc_tps(blocks: Vec<Block<H256>>) -> Vec<f64> {
    let mut tps = Vec::new();

    let offset = 1; // The first block is skipped for lacking a block interval.
    for i in offset..blocks.len() {
        let prev_block = &blocks[i - 1];
        let curr_block = &blocks[i];

        let interval = curr_block.timestamp.saturating_sub(prev_block.timestamp);

        if interval.le(&U256::zero()) {
            continue;
        }

        let interval = interval.as_u64() as f64;
        let tx_count = curr_block.transactions.len() as f64;
        let block_tps = tx_count / interval;
        tps.push(block_tps);
    }
    tps
}

#[cfg(test)]
mod tests {
    use super::super::FLOAT_TOLERANCE;
    use super::*;

    #[test]
    fn test_calc_tps() {
        let tx = H256::random();
        let blocks = vec![
            Block {
                timestamp: U256::from(100),
                transactions: vec![tx; 500],
                ..Default::default()
            },
            Block {
                timestamp: U256::from(110),
                transactions: vec![tx; 600],
                ..Default::default()
            },
            Block {
                timestamp: U256::from(130),
                transactions: vec![tx; 1400],
                ..Default::default()
            },
            Block {
                timestamp: U256::from(160),
                transactions: vec![tx; 2400],
                ..Default::default()
            },
            Block {
                timestamp: U256::from(200),
                transactions: vec![tx; 4000],
                ..Default::default()
            },
        ]
        .into_iter()
        .collect();

        let tps = calc_tps(blocks);
        assert_eq!(tps.len(), 4); // Block 1 is skipped.
        let expected_tps = [
            600.0 / 10.0,  // Block 2: 600 transactions in 10 seconds = 60 TPS
            1400.0 / 20.0, // Block 3: 1400 transactions in 20 seconds = 70 TPS
            2400.0 / 30.0, // Block 4: 2400 transactions in 30 seconds = 80 TPS
            4000.0 / 40.0, // Block 5: 4000 transactions in 40 seconds = 100 TPS
        ];

        for (i, &expected) in expected_tps.iter().enumerate() {
            assert!(
                (tps[i] - expected).abs() < FLOAT_TOLERANCE,
                "mismatch at index {}: got {}, expected {}",
                i,
                tps[i],
                expected
            );
        }
    }
}
