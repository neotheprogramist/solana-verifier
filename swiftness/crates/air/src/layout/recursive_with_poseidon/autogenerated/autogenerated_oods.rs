use crate::{
    consts::*,
    felt_nonzero,
    layout::recursive_with_poseidon::{LayoutTrait, StaticLayoutTrait},
};
use starknet_crypto::Felt;
use starknet_types_core::felt::NonZeroFelt;

#[inline(never)]
pub fn eval_oods_polynomial_inner<Layout: StaticLayoutTrait + LayoutTrait>(
    pow: &mut [Felt; 134],
    column_values: &[Felt],
    oods_values: &[Felt],
    constraint_coefficients: &[Felt],
    point: &Felt,
    oods_point: &Felt,
    trace_generator: &Felt,
) -> Felt {
    // Compute powers.
    pow[0] = trace_generator.pow_felt(&(FELT_0));
    pow[1] = trace_generator.pow_felt(&(FELT_4089));
    pow[2] = trace_generator.pow_felt(&(FELT_2011));
    pow[3] = trace_generator.pow_felt(&(FELT_1539));
    pow[4] = trace_generator.pow_felt(&(FELT_1));
    pow[5] = pow[4] * pow[4]; // pow(trace_generator, 2).
    pow[6] = pow[4] * pow[5]; // pow(trace_generator, 3).
    pow[7] = pow[4] * pow[6]; // pow(trace_generator, 4).
    pow[8] = pow[4] * pow[7]; // pow(trace_generator, 5).
    pow[9] = pow[4] * pow[8]; // pow(trace_generator, 6).
    pow[10] = pow[4] * pow[9]; // pow(trace_generator, 7).
    pow[11] = pow[4] * pow[10]; // pow(trace_generator, 8).
    pow[12] = pow[3] * pow[11]; // pow(trace_generator, 1547).
    pow[13] = pow[4] * pow[11]; // pow(trace_generator, 9).
    pow[14] = pow[4] * pow[13]; // pow(trace_generator, 10).
    pow[15] = pow[4] * pow[14]; // pow(trace_generator, 11).
    pow[16] = pow[4] * pow[15]; // pow(trace_generator, 12).
    pow[17] = pow[4] * pow[16]; // pow(trace_generator, 13).
    pow[18] = pow[4] * pow[17]; // pow(trace_generator, 14).
    pow[19] = pow[4] * pow[18]; // pow(trace_generator, 15).
    pow[20] = pow[4] * pow[19]; // pow(trace_generator, 16).
    pow[21] = pow[4] * pow[20]; // pow(trace_generator, 17).
    pow[22] = pow[6] * pow[21]; // pow(trace_generator, 20).
    pow[23] = pow[5] * pow[22]; // pow(trace_generator, 22).
    pow[24] = pow[5] * pow[23]; // pow(trace_generator, 24).
    pow[25] = pow[4] * pow[24]; // pow(trace_generator, 25).
    pow[26] = pow[6] * pow[25]; // pow(trace_generator, 28).
    pow[27] = pow[5] * pow[26]; // pow(trace_generator, 30).
    pow[28] = pow[5] * pow[27]; // pow(trace_generator, 32).
    pow[29] = pow[4] * pow[28]; // pow(trace_generator, 33).
    pow[30] = pow[3] * pow[28]; // pow(trace_generator, 1571).
    pow[31] = pow[6] * pow[29]; // pow(trace_generator, 36).
    pow[32] = pow[5] * pow[31]; // pow(trace_generator, 38).
    pow[33] = pow[5] * pow[32]; // pow(trace_generator, 40).
    pow[34] = pow[4] * pow[33]; // pow(trace_generator, 41).
    pow[35] = pow[4] * pow[34]; // pow(trace_generator, 42).
    pow[36] = pow[4] * pow[35]; // pow(trace_generator, 43).
    pow[37] = pow[4] * pow[36]; // pow(trace_generator, 44).
    pow[38] = pow[5] * pow[37]; // pow(trace_generator, 46).
    pow[39] = pow[5] * pow[38]; // pow(trace_generator, 48).
    pow[40] = pow[4] * pow[39]; // pow(trace_generator, 49).
    pow[41] = pow[6] * pow[40]; // pow(trace_generator, 52).
    pow[42] = pow[5] * pow[41]; // pow(trace_generator, 54).
    pow[43] = pow[5] * pow[42]; // pow(trace_generator, 56).
    pow[44] = pow[4] * pow[43]; // pow(trace_generator, 57).
    pow[45] = pow[6] * pow[44]; // pow(trace_generator, 60).
    pow[46] = pow[7] * pow[45]; // pow(trace_generator, 64).
    pow[47] = pow[4] * pow[46]; // pow(trace_generator, 65).
    pow[48] = pow[4] * pow[47]; // pow(trace_generator, 66).
    pow[49] = pow[10] * pow[48]; // pow(trace_generator, 73).
    pow[50] = pow[4] * pow[49]; // pow(trace_generator, 74).
    pow[51] = pow[4] * pow[50]; // pow(trace_generator, 75).
    pow[52] = pow[4] * pow[51]; // pow(trace_generator, 76).
    pow[53] = pow[8] * pow[52]; // pow(trace_generator, 81).
    pow[54] = pow[11] * pow[53]; // pow(trace_generator, 89).
    pow[55] = pow[11] * pow[54]; // pow(trace_generator, 97).
    pow[56] = pow[11] * pow[55]; // pow(trace_generator, 105).
    pow[57] = pow[4] * pow[56]; // pow(trace_generator, 106).
    pow[58] = pow[5] * pow[57]; // pow(trace_generator, 108).
    pow[59] = pow[22] * pow[58]; // pow(trace_generator, 128).
    pow[60] = pow[5] * pow[59]; // pow(trace_generator, 130).
    pow[61] = pow[10] * pow[60]; // pow(trace_generator, 137).
    pow[62] = pow[4] * pow[61]; // pow(trace_generator, 138).
    pow[63] = pow[4] * pow[62]; // pow(trace_generator, 139).
    pow[64] = pow[27] * pow[63]; // pow(trace_generator, 169).
    pow[65] = pow[5] * pow[64]; // pow(trace_generator, 171).
    pow[66] = pow[4] * pow[63]; // pow(trace_generator, 140).
    pow[67] = pow[4] * pow[65]; // pow(trace_generator, 172).
    pow[68] = pow[7] * pow[67]; // pow(trace_generator, 176).
    pow[69] = pow[7] * pow[68]; // pow(trace_generator, 180).
    pow[70] = pow[7] * pow[69]; // pow(trace_generator, 184).
    pow[71] = pow[7] * pow[70]; // pow(trace_generator, 188).
    pow[72] = pow[7] * pow[71]; // pow(trace_generator, 192).
    pow[73] = pow[5] * pow[72]; // pow(trace_generator, 194).
    pow[74] = pow[10] * pow[73]; // pow(trace_generator, 201).
    pow[75] = pow[4] * pow[74]; // pow(trace_generator, 202).
    pow[76] = pow[4] * pow[75]; // pow(trace_generator, 203).
    pow[77] = pow[72] * pow[74]; // pow(trace_generator, 393).
    pow[78] = pow[4] * pow[76]; // pow(trace_generator, 204).
    pow[79] = pow[27] * pow[78]; // pow(trace_generator, 234).
    pow[80] = pow[4] * pow[79]; // pow(trace_generator, 235).
    pow[81] = pow[4] * pow[80]; // pow(trace_generator, 236).
    pow[82] = pow[7] * pow[81]; // pow(trace_generator, 240).
    pow[83] = pow[7] * pow[82]; // pow(trace_generator, 244).
    pow[84] = pow[7] * pow[83]; // pow(trace_generator, 248).
    pow[85] = pow[7] * pow[84]; // pow(trace_generator, 252).
    pow[86] = pow[18] * pow[85]; // pow(trace_generator, 266).
    pow[87] = pow[4] * pow[86]; // pow(trace_generator, 267).
    pow[88] = pow[4] * pow[77]; // pow(trace_generator, 394).
    pow[89] = pow[19] * pow[88]; // pow(trace_generator, 409).
    pow[90] = pow[20] * pow[89]; // pow(trace_generator, 425).
    pow[91] = pow[28] * pow[90]; // pow(trace_generator, 457).
    pow[92] = pow[4] * pow[91]; // pow(trace_generator, 458).
    pow[93] = pow[4] * pow[92]; // pow(trace_generator, 459).
    pow[94] = pow[18] * pow[93]; // pow(trace_generator, 473).
    pow[95] = pow[20] * pow[94]; // pow(trace_generator, 489).
    pow[96] = pow[28] * pow[95]; // pow(trace_generator, 521).
    pow[97] = pow[28] * pow[96]; // pow(trace_generator, 553).
    pow[98] = pow[28] * pow[97]; // pow(trace_generator, 585).
    pow[99] = pow[24] * pow[98]; // pow(trace_generator, 609).
    pow[100] = pow[20] * pow[99]; // pow(trace_generator, 625).
    pow[101] = pow[20] * pow[100]; // pow(trace_generator, 641).
    pow[102] = pow[20] * pow[101]; // pow(trace_generator, 657).
    pow[103] = pow[84] * pow[102]; // pow(trace_generator, 905).
    pow[104] = pow[20] * pow[102]; // pow(trace_generator, 673).
    pow[105] = pow[20] * pow[103]; // pow(trace_generator, 921).
    pow[106] = pow[20] * pow[104]; // pow(trace_generator, 689).
    pow[107] = pow[20] * pow[105]; // pow(trace_generator, 937).
    pow[108] = pow[28] * pow[107]; // pow(trace_generator, 969).
    pow[109] = pow[25] * pow[106]; // pow(trace_generator, 714).
    pow[110] = pow[46] * pow[109]; // pow(trace_generator, 778).
    pow[111] = pow[4] * pow[108]; // pow(trace_generator, 970).
    pow[112] = pow[3] * pow[33]; // pow(trace_generator, 1579).
    pow[113] = pow[4] * pow[109]; // pow(trace_generator, 715).
    pow[114] = pow[4] * pow[110]; // pow(trace_generator, 779).
    pow[115] = pow[28] * pow[86]; // pow(trace_generator, 298).
    pow[116] = pow[4] * pow[111]; // pow(trace_generator, 971).
    pow[117] = pow[15] * pow[116]; // pow(trace_generator, 982).
    pow[118] = pow[6] * pow[117]; // pow(trace_generator, 985).
    pow[119] = pow[17] * pow[118]; // pow(trace_generator, 998).
    pow[120] = pow[6] * pow[119]; // pow(trace_generator, 1001).
    pow[121] = pow[17] * pow[120]; // pow(trace_generator, 1014).
    pow[122] = pow[22] * pow[121]; // pow(trace_generator, 1034).
    pow[123] = pow[2] * pow[11]; // pow(trace_generator, 2019).
    pow[124] = pow[2] * pow[27]; // pow(trace_generator, 2041).
    pow[125] = pow[7] * pow[124]; // pow(trace_generator, 2045).
    pow[126] = pow[2] * pow[31]; // pow(trace_generator, 2047).
    pow[127] = pow[4] * pow[122]; // pow(trace_generator, 1035).
    pow[128] = pow[2] * pow[32]; // pow(trace_generator, 2049).
    pow[129] = pow[2] * pow[33]; // pow(trace_generator, 2051).
    pow[130] = pow[2] * pow[35]; // pow(trace_generator, 2053).
    pow[131] = pow[8] * pow[130]; // pow(trace_generator, 2058).
    pow[132] = pow[2] * pow[39]; // pow(trace_generator, 2059).
    pow[133] = pow[1] * pow[21]; // pow(trace_generator, 4106).

    // Fetch columns.
    let column0 = column_values[0];
    let column1 = column_values[1];
    let column2 = column_values[2];
    let column3 = column_values[3];
    let column4 = column_values[4];
    let column5 = column_values[5];
    let column6 = column_values[6];
    let column7 = column_values[7];

    // Sum the OODS constraints on the trace polynomials.
    let mut total_sum = FELT_0;
    let mut value;

    value = (column0 - oods_values[0]).field_div(&felt_nonzero!(point - pow[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[0] * value;

    value = (column0 - oods_values[1]).field_div(&felt_nonzero!(point - pow[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[1] * value;

    value = (column0 - oods_values[2]).field_div(&felt_nonzero!(point - pow[5] * oods_point));
    total_sum = total_sum + constraint_coefficients[2] * value;

    value = (column0 - oods_values[3]).field_div(&felt_nonzero!(point - pow[6] * oods_point));
    total_sum = total_sum + constraint_coefficients[3] * value;

    value = (column0 - oods_values[4]).field_div(&felt_nonzero!(point - pow[7] * oods_point));
    total_sum = total_sum + constraint_coefficients[4] * value;

    value = (column0 - oods_values[5]).field_div(&felt_nonzero!(point - pow[8] * oods_point));
    total_sum = total_sum + constraint_coefficients[5] * value;

    value = (column0 - oods_values[6]).field_div(&felt_nonzero!(point - pow[9] * oods_point));
    total_sum = total_sum + constraint_coefficients[6] * value;

    value = (column0 - oods_values[7]).field_div(&felt_nonzero!(point - pow[10] * oods_point));
    total_sum = total_sum + constraint_coefficients[7] * value;

    value = (column0 - oods_values[8]).field_div(&felt_nonzero!(point - pow[11] * oods_point));
    total_sum = total_sum + constraint_coefficients[8] * value;

    value = (column0 - oods_values[9]).field_div(&felt_nonzero!(point - pow[13] * oods_point));
    total_sum = total_sum + constraint_coefficients[9] * value;

    value = (column0 - oods_values[10]).field_div(&felt_nonzero!(point - pow[14] * oods_point));
    total_sum = total_sum + constraint_coefficients[10] * value;

    value = (column0 - oods_values[11]).field_div(&felt_nonzero!(point - pow[15] * oods_point));
    total_sum = total_sum + constraint_coefficients[11] * value;

    value = (column0 - oods_values[12]).field_div(&felt_nonzero!(point - pow[16] * oods_point));
    total_sum = total_sum + constraint_coefficients[12] * value;

    value = (column0 - oods_values[13]).field_div(&felt_nonzero!(point - pow[17] * oods_point));
    total_sum = total_sum + constraint_coefficients[13] * value;

    value = (column0 - oods_values[14]).field_div(&felt_nonzero!(point - pow[18] * oods_point));
    total_sum = total_sum + constraint_coefficients[14] * value;

    value = (column0 - oods_values[15]).field_div(&felt_nonzero!(point - pow[19] * oods_point));
    total_sum = total_sum + constraint_coefficients[15] * value;

    value = (column1 - oods_values[16]).field_div(&felt_nonzero!(point - pow[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[16] * value;

    value = (column1 - oods_values[17]).field_div(&felt_nonzero!(point - pow[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[17] * value;

    value = (column1 - oods_values[18]).field_div(&felt_nonzero!(point - pow[5] * oods_point));
    total_sum = total_sum + constraint_coefficients[18] * value;

    value = (column1 - oods_values[19]).field_div(&felt_nonzero!(point - pow[6] * oods_point));
    total_sum = total_sum + constraint_coefficients[19] * value;

    value = (column1 - oods_values[20]).field_div(&felt_nonzero!(point - pow[7] * oods_point));
    total_sum = total_sum + constraint_coefficients[20] * value;

    value = (column1 - oods_values[21]).field_div(&felt_nonzero!(point - pow[8] * oods_point));
    total_sum = total_sum + constraint_coefficients[21] * value;

    value = (column1 - oods_values[22]).field_div(&felt_nonzero!(point - pow[11] * oods_point));
    total_sum = total_sum + constraint_coefficients[22] * value;

    value = (column1 - oods_values[23]).field_div(&felt_nonzero!(point - pow[13] * oods_point));
    total_sum = total_sum + constraint_coefficients[23] * value;

    value = (column1 - oods_values[24]).field_div(&felt_nonzero!(point - pow[14] * oods_point));
    total_sum = total_sum + constraint_coefficients[24] * value;

    value = (column1 - oods_values[25]).field_div(&felt_nonzero!(point - pow[15] * oods_point));
    total_sum = total_sum + constraint_coefficients[25] * value;

    value = (column1 - oods_values[26]).field_div(&felt_nonzero!(point - pow[16] * oods_point));
    total_sum = total_sum + constraint_coefficients[26] * value;

    value = (column1 - oods_values[27]).field_div(&felt_nonzero!(point - pow[17] * oods_point));
    total_sum = total_sum + constraint_coefficients[27] * value;

    value = (column1 - oods_values[28]).field_div(&felt_nonzero!(point - pow[20] * oods_point));
    total_sum = total_sum + constraint_coefficients[28] * value;

    value = (column1 - oods_values[29]).field_div(&felt_nonzero!(point - pow[35] * oods_point));
    total_sum = total_sum + constraint_coefficients[29] * value;

    value = (column1 - oods_values[30]).field_div(&felt_nonzero!(point - pow[36] * oods_point));
    total_sum = total_sum + constraint_coefficients[30] * value;

    value = (column1 - oods_values[31]).field_div(&felt_nonzero!(point - pow[50] * oods_point));
    total_sum = total_sum + constraint_coefficients[31] * value;

    value = (column1 - oods_values[32]).field_div(&felt_nonzero!(point - pow[51] * oods_point));
    total_sum = total_sum + constraint_coefficients[32] * value;

    value = (column1 - oods_values[33]).field_div(&felt_nonzero!(point - pow[57] * oods_point));
    total_sum = total_sum + constraint_coefficients[33] * value;

    value = (column1 - oods_values[34]).field_div(&felt_nonzero!(point - pow[62] * oods_point));
    total_sum = total_sum + constraint_coefficients[34] * value;

    value = (column1 - oods_values[35]).field_div(&felt_nonzero!(point - pow[63] * oods_point));
    total_sum = total_sum + constraint_coefficients[35] * value;

    value = (column1 - oods_values[36]).field_div(&felt_nonzero!(point - pow[65] * oods_point));
    total_sum = total_sum + constraint_coefficients[36] * value;

    value = (column1 - oods_values[37]).field_div(&felt_nonzero!(point - pow[75] * oods_point));
    total_sum = total_sum + constraint_coefficients[37] * value;

    value = (column1 - oods_values[38]).field_div(&felt_nonzero!(point - pow[76] * oods_point));
    total_sum = total_sum + constraint_coefficients[38] * value;

    value = (column1 - oods_values[39]).field_div(&felt_nonzero!(point - pow[79] * oods_point));
    total_sum = total_sum + constraint_coefficients[39] * value;

    value = (column1 - oods_values[40]).field_div(&felt_nonzero!(point - pow[80] * oods_point));
    total_sum = total_sum + constraint_coefficients[40] * value;

    value = (column1 - oods_values[41]).field_div(&felt_nonzero!(point - pow[86] * oods_point));
    total_sum = total_sum + constraint_coefficients[41] * value;

    value = (column1 - oods_values[42]).field_div(&felt_nonzero!(point - pow[87] * oods_point));
    total_sum = total_sum + constraint_coefficients[42] * value;

    value = (column1 - oods_values[43]).field_div(&felt_nonzero!(point - pow[115] * oods_point));
    total_sum = total_sum + constraint_coefficients[43] * value;

    value = (column1 - oods_values[44]).field_div(&felt_nonzero!(point - pow[88] * oods_point));
    total_sum = total_sum + constraint_coefficients[44] * value;

    value = (column1 - oods_values[45]).field_div(&felt_nonzero!(point - pow[92] * oods_point));
    total_sum = total_sum + constraint_coefficients[45] * value;

    value = (column1 - oods_values[46]).field_div(&felt_nonzero!(point - pow[93] * oods_point));
    total_sum = total_sum + constraint_coefficients[46] * value;

    value = (column1 - oods_values[47]).field_div(&felt_nonzero!(point - pow[109] * oods_point));
    total_sum = total_sum + constraint_coefficients[47] * value;

    value = (column1 - oods_values[48]).field_div(&felt_nonzero!(point - pow[113] * oods_point));
    total_sum = total_sum + constraint_coefficients[48] * value;

    value = (column1 - oods_values[49]).field_div(&felt_nonzero!(point - pow[110] * oods_point));
    total_sum = total_sum + constraint_coefficients[49] * value;

    value = (column1 - oods_values[50]).field_div(&felt_nonzero!(point - pow[114] * oods_point));
    total_sum = total_sum + constraint_coefficients[50] * value;

    value = (column1 - oods_values[51]).field_div(&felt_nonzero!(point - pow[111] * oods_point));
    total_sum = total_sum + constraint_coefficients[51] * value;

    value = (column1 - oods_values[52]).field_div(&felt_nonzero!(point - pow[116] * oods_point));
    total_sum = total_sum + constraint_coefficients[52] * value;

    value = (column1 - oods_values[53]).field_div(&felt_nonzero!(point - pow[122] * oods_point));
    total_sum = total_sum + constraint_coefficients[53] * value;

    value = (column1 - oods_values[54]).field_div(&felt_nonzero!(point - pow[127] * oods_point));
    total_sum = total_sum + constraint_coefficients[54] * value;

    value = (column1 - oods_values[55]).field_div(&felt_nonzero!(point - pow[131] * oods_point));
    total_sum = total_sum + constraint_coefficients[55] * value;

    value = (column1 - oods_values[56]).field_div(&felt_nonzero!(point - pow[132] * oods_point));
    total_sum = total_sum + constraint_coefficients[56] * value;

    value = (column1 - oods_values[57]).field_div(&felt_nonzero!(point - pow[133] * oods_point));
    total_sum = total_sum + constraint_coefficients[57] * value;

    value = (column2 - oods_values[58]).field_div(&felt_nonzero!(point - pow[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[58] * value;

    value = (column2 - oods_values[59]).field_div(&felt_nonzero!(point - pow[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[59] * value;

    value = (column2 - oods_values[60]).field_div(&felt_nonzero!(point - pow[5] * oods_point));
    total_sum = total_sum + constraint_coefficients[60] * value;

    value = (column2 - oods_values[61]).field_div(&felt_nonzero!(point - pow[6] * oods_point));
    total_sum = total_sum + constraint_coefficients[61] * value;

    value = (column3 - oods_values[62]).field_div(&felt_nonzero!(point - pow[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[62] * value;

    value = (column3 - oods_values[63]).field_div(&felt_nonzero!(point - pow[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[63] * value;

    value = (column3 - oods_values[64]).field_div(&felt_nonzero!(point - pow[5] * oods_point));
    total_sum = total_sum + constraint_coefficients[64] * value;

    value = (column3 - oods_values[65]).field_div(&felt_nonzero!(point - pow[6] * oods_point));
    total_sum = total_sum + constraint_coefficients[65] * value;

    value = (column3 - oods_values[66]).field_div(&felt_nonzero!(point - pow[7] * oods_point));
    total_sum = total_sum + constraint_coefficients[66] * value;

    value = (column3 - oods_values[67]).field_div(&felt_nonzero!(point - pow[11] * oods_point));
    total_sum = total_sum + constraint_coefficients[67] * value;

    value = (column3 - oods_values[68]).field_div(&felt_nonzero!(point - pow[16] * oods_point));
    total_sum = total_sum + constraint_coefficients[68] * value;

    value = (column3 - oods_values[69]).field_div(&felt_nonzero!(point - pow[20] * oods_point));
    total_sum = total_sum + constraint_coefficients[69] * value;

    value = (column3 - oods_values[70]).field_div(&felt_nonzero!(point - pow[22] * oods_point));
    total_sum = total_sum + constraint_coefficients[70] * value;

    value = (column3 - oods_values[71]).field_div(&felt_nonzero!(point - pow[24] * oods_point));
    total_sum = total_sum + constraint_coefficients[71] * value;

    value = (column3 - oods_values[72]).field_div(&felt_nonzero!(point - pow[26] * oods_point));
    total_sum = total_sum + constraint_coefficients[72] * value;

    value = (column3 - oods_values[73]).field_div(&felt_nonzero!(point - pow[28] * oods_point));
    total_sum = total_sum + constraint_coefficients[73] * value;

    value = (column3 - oods_values[74]).field_div(&felt_nonzero!(point - pow[31] * oods_point));
    total_sum = total_sum + constraint_coefficients[74] * value;

    value = (column3 - oods_values[75]).field_div(&felt_nonzero!(point - pow[33] * oods_point));
    total_sum = total_sum + constraint_coefficients[75] * value;

    value = (column3 - oods_values[76]).field_div(&felt_nonzero!(point - pow[37] * oods_point));
    total_sum = total_sum + constraint_coefficients[76] * value;

    value = (column3 - oods_values[77]).field_div(&felt_nonzero!(point - pow[39] * oods_point));
    total_sum = total_sum + constraint_coefficients[77] * value;

    value = (column3 - oods_values[78]).field_div(&felt_nonzero!(point - pow[41] * oods_point));
    total_sum = total_sum + constraint_coefficients[78] * value;

    value = (column3 - oods_values[79]).field_div(&felt_nonzero!(point - pow[43] * oods_point));
    total_sum = total_sum + constraint_coefficients[79] * value;

    value = (column3 - oods_values[80]).field_div(&felt_nonzero!(point - pow[45] * oods_point));
    total_sum = total_sum + constraint_coefficients[80] * value;

    value = (column3 - oods_values[81]).field_div(&felt_nonzero!(point - pow[46] * oods_point));
    total_sum = total_sum + constraint_coefficients[81] * value;

    value = (column3 - oods_values[82]).field_div(&felt_nonzero!(point - pow[48] * oods_point));
    total_sum = total_sum + constraint_coefficients[82] * value;

    value = (column3 - oods_values[83]).field_div(&felt_nonzero!(point - pow[59] * oods_point));
    total_sum = total_sum + constraint_coefficients[83] * value;

    value = (column3 - oods_values[84]).field_div(&felt_nonzero!(point - pow[60] * oods_point));
    total_sum = total_sum + constraint_coefficients[84] * value;

    value = (column3 - oods_values[85]).field_div(&felt_nonzero!(point - pow[68] * oods_point));
    total_sum = total_sum + constraint_coefficients[85] * value;

    value = (column3 - oods_values[86]).field_div(&felt_nonzero!(point - pow[69] * oods_point));
    total_sum = total_sum + constraint_coefficients[86] * value;

    value = (column3 - oods_values[87]).field_div(&felt_nonzero!(point - pow[70] * oods_point));
    total_sum = total_sum + constraint_coefficients[87] * value;

    value = (column3 - oods_values[88]).field_div(&felt_nonzero!(point - pow[71] * oods_point));
    total_sum = total_sum + constraint_coefficients[88] * value;

    value = (column3 - oods_values[89]).field_div(&felt_nonzero!(point - pow[72] * oods_point));
    total_sum = total_sum + constraint_coefficients[89] * value;

    value = (column3 - oods_values[90]).field_div(&felt_nonzero!(point - pow[73] * oods_point));
    total_sum = total_sum + constraint_coefficients[90] * value;

    value = (column3 - oods_values[91]).field_div(&felt_nonzero!(point - pow[82] * oods_point));
    total_sum = total_sum + constraint_coefficients[91] * value;

    value = (column3 - oods_values[92]).field_div(&felt_nonzero!(point - pow[83] * oods_point));
    total_sum = total_sum + constraint_coefficients[92] * value;

    value = (column3 - oods_values[93]).field_div(&felt_nonzero!(point - pow[84] * oods_point));
    total_sum = total_sum + constraint_coefficients[93] * value;

    value = (column3 - oods_values[94]).field_div(&felt_nonzero!(point - pow[85] * oods_point));
    total_sum = total_sum + constraint_coefficients[94] * value;

    value = (column4 - oods_values[95]).field_div(&felt_nonzero!(point - pow[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[95] * value;

    value = (column4 - oods_values[96]).field_div(&felt_nonzero!(point - pow[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[96] * value;

    value = (column4 - oods_values[97]).field_div(&felt_nonzero!(point - pow[5] * oods_point));
    total_sum = total_sum + constraint_coefficients[97] * value;

    value = (column4 - oods_values[98]).field_div(&felt_nonzero!(point - pow[6] * oods_point));
    total_sum = total_sum + constraint_coefficients[98] * value;

    value = (column4 - oods_values[99]).field_div(&felt_nonzero!(point - pow[7] * oods_point));
    total_sum = total_sum + constraint_coefficients[99] * value;

    value = (column4 - oods_values[100]).field_div(&felt_nonzero!(point - pow[8] * oods_point));
    total_sum = total_sum + constraint_coefficients[100] * value;

    value = (column4 - oods_values[101]).field_div(&felt_nonzero!(point - pow[9] * oods_point));
    total_sum = total_sum + constraint_coefficients[101] * value;

    value = (column4 - oods_values[102]).field_div(&felt_nonzero!(point - pow[10] * oods_point));
    total_sum = total_sum + constraint_coefficients[102] * value;

    value = (column4 - oods_values[103]).field_div(&felt_nonzero!(point - pow[11] * oods_point));
    total_sum = total_sum + constraint_coefficients[103] * value;

    value = (column4 - oods_values[104]).field_div(&felt_nonzero!(point - pow[13] * oods_point));
    total_sum = total_sum + constraint_coefficients[104] * value;

    value = (column4 - oods_values[105]).field_div(&felt_nonzero!(point - pow[15] * oods_point));
    total_sum = total_sum + constraint_coefficients[105] * value;

    value = (column4 - oods_values[106]).field_div(&felt_nonzero!(point - pow[16] * oods_point));
    total_sum = total_sum + constraint_coefficients[106] * value;

    value = (column4 - oods_values[107]).field_div(&felt_nonzero!(point - pow[17] * oods_point));
    total_sum = total_sum + constraint_coefficients[107] * value;

    value = (column4 - oods_values[108]).field_div(&felt_nonzero!(point - pow[37] * oods_point));
    total_sum = total_sum + constraint_coefficients[108] * value;

    value = (column4 - oods_values[109]).field_div(&felt_nonzero!(point - pow[52] * oods_point));
    total_sum = total_sum + constraint_coefficients[109] * value;

    value = (column4 - oods_values[110]).field_div(&felt_nonzero!(point - pow[58] * oods_point));
    total_sum = total_sum + constraint_coefficients[110] * value;

    value = (column4 - oods_values[111]).field_div(&felt_nonzero!(point - pow[66] * oods_point));
    total_sum = total_sum + constraint_coefficients[111] * value;

    value = (column4 - oods_values[112]).field_div(&felt_nonzero!(point - pow[67] * oods_point));
    total_sum = total_sum + constraint_coefficients[112] * value;

    value = (column4 - oods_values[113]).field_div(&felt_nonzero!(point - pow[78] * oods_point));
    total_sum = total_sum + constraint_coefficients[113] * value;

    value = (column4 - oods_values[114]).field_div(&felt_nonzero!(point - pow[81] * oods_point));
    total_sum = total_sum + constraint_coefficients[114] * value;

    value = (column4 - oods_values[115]).field_div(&felt_nonzero!(point - pow[3] * oods_point));
    total_sum = total_sum + constraint_coefficients[115] * value;

    value = (column4 - oods_values[116]).field_div(&felt_nonzero!(point - pow[12] * oods_point));
    total_sum = total_sum + constraint_coefficients[116] * value;

    value = (column4 - oods_values[117]).field_div(&felt_nonzero!(point - pow[30] * oods_point));
    total_sum = total_sum + constraint_coefficients[117] * value;

    value = (column4 - oods_values[118]).field_div(&felt_nonzero!(point - pow[112] * oods_point));
    total_sum = total_sum + constraint_coefficients[118] * value;

    value = (column4 - oods_values[119]).field_div(&felt_nonzero!(point - pow[2] * oods_point));
    total_sum = total_sum + constraint_coefficients[119] * value;

    value = (column4 - oods_values[120]).field_div(&felt_nonzero!(point - pow[123] * oods_point));
    total_sum = total_sum + constraint_coefficients[120] * value;

    value = (column4 - oods_values[121]).field_div(&felt_nonzero!(point - pow[124] * oods_point));
    total_sum = total_sum + constraint_coefficients[121] * value;

    value = (column4 - oods_values[122]).field_div(&felt_nonzero!(point - pow[125] * oods_point));
    total_sum = total_sum + constraint_coefficients[122] * value;

    value = (column4 - oods_values[123]).field_div(&felt_nonzero!(point - pow[126] * oods_point));
    total_sum = total_sum + constraint_coefficients[123] * value;

    value = (column4 - oods_values[124]).field_div(&felt_nonzero!(point - pow[128] * oods_point));
    total_sum = total_sum + constraint_coefficients[124] * value;

    value = (column4 - oods_values[125]).field_div(&felt_nonzero!(point - pow[129] * oods_point));
    total_sum = total_sum + constraint_coefficients[125] * value;

    value = (column4 - oods_values[126]).field_div(&felt_nonzero!(point - pow[130] * oods_point));
    total_sum = total_sum + constraint_coefficients[126] * value;

    value = (column4 - oods_values[127]).field_div(&felt_nonzero!(point - pow[1] * oods_point));
    total_sum = total_sum + constraint_coefficients[127] * value;

    value = (column5 - oods_values[128]).field_div(&felt_nonzero!(point - pow[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[128] * value;

    value = (column5 - oods_values[129]).field_div(&felt_nonzero!(point - pow[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[129] * value;

    value = (column5 - oods_values[130]).field_div(&felt_nonzero!(point - pow[5] * oods_point));
    total_sum = total_sum + constraint_coefficients[130] * value;

    value = (column5 - oods_values[131]).field_div(&felt_nonzero!(point - pow[7] * oods_point));
    total_sum = total_sum + constraint_coefficients[131] * value;

    value = (column5 - oods_values[132]).field_div(&felt_nonzero!(point - pow[9] * oods_point));
    total_sum = total_sum + constraint_coefficients[132] * value;

    value = (column5 - oods_values[133]).field_div(&felt_nonzero!(point - pow[11] * oods_point));
    total_sum = total_sum + constraint_coefficients[133] * value;

    value = (column5 - oods_values[134]).field_div(&felt_nonzero!(point - pow[13] * oods_point));
    total_sum = total_sum + constraint_coefficients[134] * value;

    value = (column5 - oods_values[135]).field_div(&felt_nonzero!(point - pow[14] * oods_point));
    total_sum = total_sum + constraint_coefficients[135] * value;

    value = (column5 - oods_values[136]).field_div(&felt_nonzero!(point - pow[16] * oods_point));
    total_sum = total_sum + constraint_coefficients[136] * value;

    value = (column5 - oods_values[137]).field_div(&felt_nonzero!(point - pow[18] * oods_point));
    total_sum = total_sum + constraint_coefficients[137] * value;

    value = (column5 - oods_values[138]).field_div(&felt_nonzero!(point - pow[20] * oods_point));
    total_sum = total_sum + constraint_coefficients[138] * value;

    value = (column5 - oods_values[139]).field_div(&felt_nonzero!(point - pow[21] * oods_point));
    total_sum = total_sum + constraint_coefficients[139] * value;

    value = (column5 - oods_values[140]).field_div(&felt_nonzero!(point - pow[23] * oods_point));
    total_sum = total_sum + constraint_coefficients[140] * value;

    value = (column5 - oods_values[141]).field_div(&felt_nonzero!(point - pow[24] * oods_point));
    total_sum = total_sum + constraint_coefficients[141] * value;

    value = (column5 - oods_values[142]).field_div(&felt_nonzero!(point - pow[25] * oods_point));
    total_sum = total_sum + constraint_coefficients[142] * value;

    value = (column5 - oods_values[143]).field_div(&felt_nonzero!(point - pow[27] * oods_point));
    total_sum = total_sum + constraint_coefficients[143] * value;

    value = (column5 - oods_values[144]).field_div(&felt_nonzero!(point - pow[29] * oods_point));
    total_sum = total_sum + constraint_coefficients[144] * value;

    value = (column5 - oods_values[145]).field_div(&felt_nonzero!(point - pow[32] * oods_point));
    total_sum = total_sum + constraint_coefficients[145] * value;

    value = (column5 - oods_values[146]).field_div(&felt_nonzero!(point - pow[34] * oods_point));
    total_sum = total_sum + constraint_coefficients[146] * value;

    value = (column5 - oods_values[147]).field_div(&felt_nonzero!(point - pow[38] * oods_point));
    total_sum = total_sum + constraint_coefficients[147] * value;

    value = (column5 - oods_values[148]).field_div(&felt_nonzero!(point - pow[40] * oods_point));
    total_sum = total_sum + constraint_coefficients[148] * value;

    value = (column5 - oods_values[149]).field_div(&felt_nonzero!(point - pow[42] * oods_point));
    total_sum = total_sum + constraint_coefficients[149] * value;

    value = (column5 - oods_values[150]).field_div(&felt_nonzero!(point - pow[44] * oods_point));
    total_sum = total_sum + constraint_coefficients[150] * value;

    value = (column5 - oods_values[151]).field_div(&felt_nonzero!(point - pow[47] * oods_point));
    total_sum = total_sum + constraint_coefficients[151] * value;

    value = (column5 - oods_values[152]).field_div(&felt_nonzero!(point - pow[49] * oods_point));
    total_sum = total_sum + constraint_coefficients[152] * value;

    value = (column5 - oods_values[153]).field_div(&felt_nonzero!(point - pow[53] * oods_point));
    total_sum = total_sum + constraint_coefficients[153] * value;

    value = (column5 - oods_values[154]).field_div(&felt_nonzero!(point - pow[54] * oods_point));
    total_sum = total_sum + constraint_coefficients[154] * value;

    value = (column5 - oods_values[155]).field_div(&felt_nonzero!(point - pow[55] * oods_point));
    total_sum = total_sum + constraint_coefficients[155] * value;

    value = (column5 - oods_values[156]).field_div(&felt_nonzero!(point - pow[56] * oods_point));
    total_sum = total_sum + constraint_coefficients[156] * value;

    value = (column5 - oods_values[157]).field_div(&felt_nonzero!(point - pow[61] * oods_point));
    total_sum = total_sum + constraint_coefficients[157] * value;

    value = (column5 - oods_values[158]).field_div(&felt_nonzero!(point - pow[64] * oods_point));
    total_sum = total_sum + constraint_coefficients[158] * value;

    value = (column5 - oods_values[159]).field_div(&felt_nonzero!(point - pow[74] * oods_point));
    total_sum = total_sum + constraint_coefficients[159] * value;

    value = (column5 - oods_values[160]).field_div(&felt_nonzero!(point - pow[77] * oods_point));
    total_sum = total_sum + constraint_coefficients[160] * value;

    value = (column5 - oods_values[161]).field_div(&felt_nonzero!(point - pow[89] * oods_point));
    total_sum = total_sum + constraint_coefficients[161] * value;

    value = (column5 - oods_values[162]).field_div(&felt_nonzero!(point - pow[90] * oods_point));
    total_sum = total_sum + constraint_coefficients[162] * value;

    value = (column5 - oods_values[163]).field_div(&felt_nonzero!(point - pow[91] * oods_point));
    total_sum = total_sum + constraint_coefficients[163] * value;

    value = (column5 - oods_values[164]).field_div(&felt_nonzero!(point - pow[94] * oods_point));
    total_sum = total_sum + constraint_coefficients[164] * value;

    value = (column5 - oods_values[165]).field_div(&felt_nonzero!(point - pow[95] * oods_point));
    total_sum = total_sum + constraint_coefficients[165] * value;

    value = (column5 - oods_values[166]).field_div(&felt_nonzero!(point - pow[96] * oods_point));
    total_sum = total_sum + constraint_coefficients[166] * value;

    value = (column5 - oods_values[167]).field_div(&felt_nonzero!(point - pow[97] * oods_point));
    total_sum = total_sum + constraint_coefficients[167] * value;

    value = (column5 - oods_values[168]).field_div(&felt_nonzero!(point - pow[98] * oods_point));
    total_sum = total_sum + constraint_coefficients[168] * value;

    value = (column5 - oods_values[169]).field_div(&felt_nonzero!(point - pow[99] * oods_point));
    total_sum = total_sum + constraint_coefficients[169] * value;

    value = (column5 - oods_values[170]).field_div(&felt_nonzero!(point - pow[100] * oods_point));
    total_sum = total_sum + constraint_coefficients[170] * value;

    value = (column5 - oods_values[171]).field_div(&felt_nonzero!(point - pow[101] * oods_point));
    total_sum = total_sum + constraint_coefficients[171] * value;

    value = (column5 - oods_values[172]).field_div(&felt_nonzero!(point - pow[102] * oods_point));
    total_sum = total_sum + constraint_coefficients[172] * value;

    value = (column5 - oods_values[173]).field_div(&felt_nonzero!(point - pow[104] * oods_point));
    total_sum = total_sum + constraint_coefficients[173] * value;

    value = (column5 - oods_values[174]).field_div(&felt_nonzero!(point - pow[106] * oods_point));
    total_sum = total_sum + constraint_coefficients[174] * value;

    value = (column5 - oods_values[175]).field_div(&felt_nonzero!(point - pow[103] * oods_point));
    total_sum = total_sum + constraint_coefficients[175] * value;

    value = (column5 - oods_values[176]).field_div(&felt_nonzero!(point - pow[105] * oods_point));
    total_sum = total_sum + constraint_coefficients[176] * value;

    value = (column5 - oods_values[177]).field_div(&felt_nonzero!(point - pow[107] * oods_point));
    total_sum = total_sum + constraint_coefficients[177] * value;

    value = (column5 - oods_values[178]).field_div(&felt_nonzero!(point - pow[108] * oods_point));
    total_sum = total_sum + constraint_coefficients[178] * value;

    value = (column5 - oods_values[179]).field_div(&felt_nonzero!(point - pow[117] * oods_point));
    total_sum = total_sum + constraint_coefficients[179] * value;

    value = (column5 - oods_values[180]).field_div(&felt_nonzero!(point - pow[118] * oods_point));
    total_sum = total_sum + constraint_coefficients[180] * value;

    value = (column5 - oods_values[181]).field_div(&felt_nonzero!(point - pow[119] * oods_point));
    total_sum = total_sum + constraint_coefficients[181] * value;

    value = (column5 - oods_values[182]).field_div(&felt_nonzero!(point - pow[120] * oods_point));
    total_sum = total_sum + constraint_coefficients[182] * value;

    value = (column5 - oods_values[183]).field_div(&felt_nonzero!(point - pow[121] * oods_point));
    total_sum = total_sum + constraint_coefficients[183] * value;

    value = (column6 - oods_values[184]).field_div(&felt_nonzero!(point - pow[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[184] * value;

    value = (column6 - oods_values[185]).field_div(&felt_nonzero!(point - pow[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[185] * value;

    value = (column6 - oods_values[186]).field_div(&felt_nonzero!(point - pow[5] * oods_point));
    total_sum = total_sum + constraint_coefficients[186] * value;

    value = (column6 - oods_values[187]).field_div(&felt_nonzero!(point - pow[6] * oods_point));
    total_sum = total_sum + constraint_coefficients[187] * value;

    value = (column7 - oods_values[188]).field_div(&felt_nonzero!(point - pow[0] * oods_point));
    total_sum = total_sum + constraint_coefficients[188] * value;

    value = (column7 - oods_values[189]).field_div(&felt_nonzero!(point - pow[4] * oods_point));
    total_sum = total_sum + constraint_coefficients[189] * value;

    value = (column7 - oods_values[190]).field_div(&felt_nonzero!(point - pow[5] * oods_point));
    total_sum = total_sum + constraint_coefficients[190] * value;

    value = (column7 - oods_values[191]).field_div(&felt_nonzero!(point - pow[8] * oods_point));
    total_sum = total_sum + constraint_coefficients[191] * value;

    // Sum the OODS boundary constraints on the composition polynomials.
    let oods_point_to_deg = oods_point.pow_felt(&(Layout::CONSTRAINT_DEGREE.into()));

    value = (column_values
        [Layout::NUM_COLUMNS_FIRST as usize + Layout::NUM_COLUMNS_SECOND as usize]
        - oods_values[192])
        .field_div(&felt_nonzero!(point - oods_point_to_deg));
    total_sum = total_sum + constraint_coefficients[192] * value;

    value = (column_values
        [Layout::NUM_COLUMNS_FIRST as usize + Layout::NUM_COLUMNS_SECOND as usize + 1]
        - oods_values[193])
        .field_div(&felt_nonzero!(point - oods_point_to_deg));

    total_sum + constraint_coefficients[193] * value
}
