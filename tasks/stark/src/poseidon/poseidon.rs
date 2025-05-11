pub struct PoseidonHashSingle;

pub struct PoseidonHashTwo;

pub struct PoseidonHashMany;

// impl Poseidon {
//     fn hash(x: &Felt, y: &Felt) -> Felt {
//         let mut state: Vec<Felt> = vec![x.clone(), y.clone(), Felt::from_hex("0x2").unwrap()];
//         Self::hades_permutation(&mut state);
//         let x = &state[0];
//         x.clone()
//     }

//     fn hash_single(x: &Felt) -> Felt {
//         let mut state: Vec<Felt> = vec![x.clone(), Felt::ZERO, Felt::ONE];
//         Self::hades_permutation(&mut state);
//         let x = &state[0];
//         x.clone()
//     }

//     fn hash_many(inputs: &[Felt]) -> Felt {
//         let r = Self::RATE; // chunk size
//         let m = Self::STATE_SIZE; // state size

//         // Pad input with 1 followed by 0's (if necessary).
//         let mut values = inputs.to_owned();
//         values.push(Felt::ONE);
//         values.resize(values.len().div_ceil(r) * r, Felt::ZERO);

//         assert!(values.len() % r == 0);
//         let mut state: Vec<Felt> = vec![Felt::ZERO; m];

//         // Process each block
//         for block in values.chunks(r) {
//             let mut block_state: Vec<Felt> =
//                 state[0..r].iter().zip(block).map(|(s, b)| s + b).collect();
//             block_state.extend_from_slice(&state[r..]);

//             Self::hades_permutation(&mut block_state);
//             state = block_state;
//         }

//         state[0].clone()
//     }
// }
