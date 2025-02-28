use swiftness::funvec::FunVec;
use swiftness::swiftness_fri::ComputeNextLayerCache;
use swiftness::swiftness_fri::FriVerifyCache;
use swiftness::swiftness_fri::group::FRI_GROUP;
use swiftness::swiftness_fri::layer::FriLayerComputationParams;
use swiftness::swiftness_fri::layer::compute_next_layer;
use swiftness::types::Felt;
use swiftness::types::StarkProof;
use swiftness_air::swiftness_commitment::table::decommit::MONTGOMERY_R;
use swiftness_air::swiftness_commitment::table::decommit::table_decommit;

use crate::Cache;
use crate::intermediate::Intermediate;
use crate::task::Task;
use crate::task::Tasks;

pub struct StarkVerifyLayerTask<'a> {
    pub cache: &'a mut FriVerifyCache,
    pub context: Option<StarkVerifyLayerContext<'a>>,
}

pub struct StarkVerifyLayerContext<'a> {
    pub target_layer_witness_leaves: &'a mut FunVec<Felt, 512>,
    pub target_layer_witness_table_withness: &'a swiftness_air::Witness,
    pub target_commitment: &'a swiftness_air::Commitment,
    pub params: FriLayerComputationParams<'a>,
}

impl Task for StarkVerifyLayerTask<'_> {
    // fri_verify_layers(
    fn execute(&mut self) {
        // Original

        let Self { cache, context, .. } = self;

        let FriVerifyCache {
            fri_queries,
            next_layer_cache,
            decommitment,
            ..
        } = cache;

        let Some(StarkVerifyLayerContext {
            target_layer_witness_leaves,
            target_layer_witness_table_withness,
            target_commitment,
            params,
        }) = context
        else {
            panic!("Not enough data in context");
        };

        // Compute next layer queries.
        compute_next_layer(
            next_layer_cache,
            fri_queries,
            target_layer_witness_leaves,
            params.clone(),
        )
        .unwrap();

        let ComputeNextLayerCache {
            verify_indices,
            verify_y_values,
            ..
        } = next_layer_cache;

        decommitment.values.flush();
        decommitment.montgomery_values.flush();
        decommitment.values.extend(verify_y_values.as_slice());
        for i in 0..verify_y_values.len() {
            decommitment
                .montgomery_values
                .push(verify_y_values.get(i).unwrap() * MONTGOMERY_R);
        }

        // Table decommitment.
        let _ = table_decommit(
            &mut cache.commitment,
            &target_commitment,
            verify_indices.as_slice(),
            &decommitment,
            &target_layer_witness_table_withness,
        );
    }

    fn children(&self) -> Vec<Tasks> {
        vec![Tasks::StarkVerifyLayerAssignNext]
    }
}

impl<'a> StarkVerifyLayerTask<'a> {
    pub fn view(
        layer_index: usize,
        proof: &'a mut StarkProof,
        cache: &'a mut Cache,
        intermediate: &'a mut Intermediate,
    ) -> Self {
        let cache = &mut cache.legacy.stark.fri;
        let commitment = &intermediate.verify.stark_commitment.fri;
        let witness = &mut proof.witness.fri_witness;

        let fri_step_sizes = commitment.config.fri_step_sizes.as_slice();

        let eval_points = commitment.eval_points.as_slice();
        let commitment_layer = commitment.inner_layers.as_slice();
        let layer_witness = witness.layers.as_slice_mut();

        let context = if fri_step_sizes.len() != 0 {
            let step_sizes = &fri_step_sizes[1..fri_step_sizes.len()];

            let target_layer_witness = layer_witness.get_mut(layer_index).unwrap();
            let target_layer_witness_leaves = &mut target_layer_witness.leaves;
            let target_layer_witness_table_withness = &target_layer_witness.table_witness;
            let target_commitment = commitment_layer.get(layer_index).unwrap();

            // Params.
            let coset_size = Felt::TWO.pow_felt(step_sizes.get(layer_index).unwrap());
            let params = FriLayerComputationParams {
                coset_size,
                fri_group: &FRI_GROUP,
                eval_point: *eval_points.get(layer_index).unwrap(),
            };

            let context = StarkVerifyLayerContext {
                target_layer_witness_leaves,
                target_layer_witness_table_withness,
                target_commitment,
                params,
            };

            Some(context)
        } else {
            None
        };

        StarkVerifyLayerTask { cache, context }
    }
}
