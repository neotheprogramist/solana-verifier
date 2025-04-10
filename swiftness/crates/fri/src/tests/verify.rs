use crate::{
    fixtures::{cache, commitment, decommitment, queries, witness},
    fri::fri_verify,
    types::DecommitmentRef,
};

use super::*;

#[test]
fn test_fri_verify() {
    let queries = queries::get();
    let commitment = commitment::get();
    let decommitment = decommitment::get();
    let decommitment_ref = DecommitmentRef {
        values: decommitment.values.as_slice(),
        points: decommitment.points.as_slice(),
    };
    let mut withness = witness::get();
    let mut cache = cache::get();

    fri_verify(&mut cache, &queries, &commitment, &decommitment_ref, &mut withness).unwrap();
}
