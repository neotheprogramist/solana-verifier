use alloc::vec;
use starknet_crypto::Felt;
use swiftness_commitment::{
    table::{config::Config as TableCommitmentConfig, types::Commitment as TableCommitment},
    vector::{config::Config as VectorCommitmentConfig, types::Commitment as VectorCommitment},
};

use crate::types::Commitment as FriCommitment;

use super::{config, last_layer_coefficients};

pub fn get() -> FriCommitment {
    FriCommitment {
        config: config::get(),
        inner_layers: vec![
            TableCommitment {
                config: TableCommitmentConfig {
                    n_columns: Felt::from_hex_unchecked("0x10"),
                    vector: VectorCommitmentConfig {
                        height: Felt::from_hex_unchecked("0x10"),
                        n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                    },
                },
                vector_commitment: VectorCommitment {
                    config: VectorCommitmentConfig {
                        height: Felt::from_hex_unchecked("0x10"),
                        n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                    },
                    commitment_hash: Felt::from_hex_unchecked(
                        "0x31b917291bbb3d38f7bc196dee1f3638ca197512162a4bdeb1ce814619c1625",
                    ),
                },
            },
            TableCommitment {
                config: TableCommitmentConfig {
                    n_columns: Felt::from_hex_unchecked("0x8"),
                    vector: VectorCommitmentConfig {
                        height: Felt::from_hex_unchecked("0xd"),
                        n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                    },
                },
                vector_commitment: VectorCommitment {
                    config: VectorCommitmentConfig {
                        height: Felt::from_hex_unchecked("0xd"),
                        n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                    },
                    commitment_hash: Felt::from_hex_unchecked(
                        "0x6945b2895872a701b3451cdf93dca9cba3cad8f250d5866ca5c263e41c8f2b2",
                    ),
                },
            },
            TableCommitment {
                config: TableCommitmentConfig {
                    n_columns: Felt::from_hex_unchecked("0x4"),
                    vector: VectorCommitmentConfig {
                        height: Felt::from_hex_unchecked("0xb"),
                        n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                    },
                },
                vector_commitment: VectorCommitment {
                    config: VectorCommitmentConfig {
                        height: Felt::from_hex_unchecked("0xb"),
                        n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                    },
                    commitment_hash: Felt::from_hex_unchecked(
                        "0x786c3ebbd4cab0c782d36860cd51887712953c48ce72c8d10acf5698c5a1213",
                    ),
                },
            },
            TableCommitment {
                config: TableCommitmentConfig {
                    n_columns: Felt::from_hex_unchecked("0x4"),
                    vector: VectorCommitmentConfig {
                        height: Felt::from_hex_unchecked("0x9"),
                        n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                    },
                },
                vector_commitment: VectorCommitment {
                    config: VectorCommitmentConfig {
                        height: Felt::from_hex_unchecked("0x9"),
                        n_verifier_friendly_commitment_layers: Felt::from_hex_unchecked("0x64"),
                    },
                    commitment_hash: Felt::from_hex_unchecked(
                        "0x1e9b0fa29ebe52b9c9a43a1d44e555ce42da3199370134d758735bfe9f40269",
                    ),
                },
            },
        ],
        eval_points: vec![
            Felt::from_hex_unchecked(
                "0x3fa22931f1e5f47eb6273e90ee38c37a21730bb432f6ef09c7c8f8c4e7b7fff",
            ),
            Felt::from_hex_unchecked(
                "0x72089def6cbdc9a9ad42dab128c499727e36b05d40df74bbccff39650569bd",
            ),
            Felt::from_hex_unchecked(
                "0x311de180838f0ad6e82a20d03978ddefb1c73bcee966471479a6c70fdc05f34",
            ),
            Felt::from_hex_unchecked(
                "0x18e241a9c138d318daa567510ba31c4ebeecdaab9076b3a8828dbb4b3303e3",
            ),
        ],
        last_layer_coefficients: last_layer_coefficients::get(),
    }
}
