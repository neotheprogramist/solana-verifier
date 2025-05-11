use crate::felt::Felt;
use lambdaworks_crypto::hash::poseidon::{
    starknet::PoseidonCairoStark252, Poseidon as PoseidonLambdaworks,
};
pub struct Poseidon;

// impl Poseidon {
//     /// Computes the Hades permutation over a mutable state of 3 Felts, as defined
//     /// in <https://docs.starknet.io/documentation/architecture_and_concepts/Cryptography/hash-functions/#poseidon_array_hash>
//     pub fn hades_permutation(state: &mut [Felt; 3]) {
//         let mut state_inner = [state[0].0, state[1].0, state[2].0];
//         PoseidonCairoStark252::hades_permutation(&mut state_inner);
//         for i in 0..3 {
//             state[i] = Felt(state_inner[i]);
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_hades_permutation() {
    //     let mut state = [
    //         Felt::from_hex("0x9").unwrap(),
    //         Felt::from_hex("0xb").unwrap(),
    //         Felt::from_hex("0x2").unwrap(),
    //     ];
    //     let expected = [
    //         Felt::from_hex("0x510f3a3faf4084e3b1e95fd44c30746271b48723f7ea9c8be6a9b6b5408e7e6")
    //             .unwrap(),
    //         Felt::from_hex("0x4f511749bd4101266904288021211333fb0a514cb15381af087462fa46e6bd9")
    //             .unwrap(),
    //         Felt::from_hex("0x186f6dd1a6e79cb1b66d505574c349272cd35c07c223351a0990410798bb9d8")
    //             .unwrap(),
    //     ];
    //     Poseidon::hades_permutation(&mut state);

    //     assert_eq!(state, expected);
    // }
}
