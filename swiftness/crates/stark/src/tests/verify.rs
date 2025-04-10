use crate::{
    fixtures::{commitment, domains, witness},
    types::CacheStark,
    verify::stark_verify,
};
use swiftness_air::{
    fixtures::public_input,
    layout::{recursive::Layout, StaticLayoutTrait},
};
use swiftness_fri::fixtures::queries;

#[test]
pub fn test_stark_verify() {
    let public_input = public_input::get();
    let queries = queries::get();
    let commitment = commitment::get();
    let mut witness = witness::get();
    let stark_domains = domains::get();

    let mut cache = CacheStark::default();

    stark_verify::<Layout>(
        &mut cache,
        Layout::NUM_COLUMNS_FIRST,
        Layout::NUM_COLUMNS_SECOND,
        &public_input,
        &queries,
        &commitment,
        &mut witness,
        &stark_domains,
    )
    .unwrap()
}
