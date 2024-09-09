use super::{
    attribute_parser::group_attribute_parser,
    base::{qstring, tstring, ws},
};

use crate::{LibRes, LibertyJson};

use nom::{
    branch::alt,
    bytes::complete::tag,
    error::context,
    multi::many0,
    sequence::{delimited, terminated, tuple},
};
use serde_json::map::Map;

// header group is named group, and only contain attributes
pub fn header_group_parser(input: &str) -> LibRes<&str, (&str, LibertyJson)> {
    context(
        "Named Group Parser",
        tuple((
            tuple((
                tstring,
                delimited(tag("("), alt((qstring, tstring)), tag(")")),
            )),
            delimited(ws(tag("{")), many0(group_attribute_parser), ws(tag("}"))),
        )),
    )(input)
    .map(|(res, data)| {
        let mut json_data = Map::new();
        let mut result = Map::new();
        for attr in data.1 {
            json_data.insert(attr.0.to_string(), attr.1);
        }
        result.insert(((data.0).1).into(), LibertyJson::from(json_data));
        // group attribute is unique, so insert directly

        (res, ((data.0).0, LibertyJson::from(result)))
    })
}

// assume named group only contain unnamed group
pub fn named_group_parser(input: &str) -> LibRes<&str, (&str, LibertyJson)> {
    context(
        "Named Group Parser",
        tuple((
            tuple((tstring, delimited(tag("("), tstring, tag(")")))),
            delimited(
                ws(tag("{")),
                tuple((many0(group_attribute_parser), many0(unnamed_group_parser))),
                ws(tag("}")),
            ),
        )),
    )(input)
    .map(|(res, data)| {
        let mut json_data = Map::new();
        let mut result = Map::new();
        // json_data.insert("name".into(), LibertyJson::from((data.0).1.to_string()));
        // group attribute is unique, so insert directly
        if !(data.1).0.is_empty() {
            for attr in (data.1).0 {
                json_data.insert(attr.0.to_string(), attr.1);
            }
        }
        // group duplicated group, like timing(),power()
        if !(data.1).1.is_empty() {
            // first check similarity
            let unique_groups = merge_same_group((data.1).1);
            for grp in unique_groups {
                json_data.insert(grp.0, grp.1);
            }
        }
        result.insert((data.0).1.into(), LibertyJson::from(json_data));
        (res, ((data.0).0, LibertyJson::from(result)))
    })
}

pub fn unnamed_group_parser(input: &str) -> LibRes<&str, (&str, LibertyJson)> {
    context(
        "UnNamed Group Parser",
        tuple((
            terminated(tstring, tag("()")),
            delimited(
                ws(tag("{")),
                tuple((
                    many0(group_attribute_parser),
                    many0(alt((unnamed_group_parser, named_group_parser))),
                )),
                ws(tag("}")),
            ),
        )),
    )(input)
    .map(|(res, data)| {
        let mut json_data = Map::new();
        if !(data.1).0.is_empty() {
            for attr in (data.1).0 {
                json_data.insert(attr.0.to_string(), attr.1);
            }
        }
        if !(data.1).1.is_empty() {
            let unique_groups = merge_same_group((data.1).1);
            for grp in unique_groups {
                json_data.insert(grp.0, grp.1);
            }
        }
        (res, (data.0, LibertyJson::from(json_data)))
    })
}

use std::collections::HashSet;
// check duplicated key situation in all (key,value), return ifself if no duplicated key,
// else merge all values with duplicated key into new value and return non-duplicated key (key,value)s
pub fn merge_same_group(groups: Vec<(&str, LibertyJson)>) -> Vec<(String, LibertyJson)> {
    let mut key_set = HashSet::new();
    let mut last_insert_fail = false;
    let mut need_merge_group_name = String::new();
    let mut result = Vec::new();
    let mut need_merge = Vec::new();
    for (k, v) in groups {
        if key_set.insert(k) {
            result.push((k.to_string(), v));
            if last_insert_fail == true {
                result.push((
                    need_merge_group_name.clone(),
                    LibertyJson::from(need_merge.clone()),
                ));
                need_merge = Vec::new();
            }
        } else {
            last_insert_fail = true;
            need_merge.push(v);
            if last_insert_fail == false {
                need_merge_group_name = k.to_string();
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_group_1() {
        let input = "    pin(A) { 
      capacitance : 0.0004127 ; 
      direction : input ; 
      fall_capacitance : 0.0004127 ; 
      input_voltage : default ; 
      nextstate_type : data ; 
      related_ground_pin : VSS ; 
      related_power_pin : VDD ; 
      rise_capacitance : 0.00041609 ; 

      internal_power() { 
        related_pg_pin : \"VDD\" ; 
        when : \"!B&!CK&!SE&!SI\" ; 

        fall_power(pwr_tin_7) { 
          index_1(\"0.01, 0.043638, 0.16172, 0.3872, 0.73849, \\
                  1.2314, 1.88\");
          values(\"0.00027584, 0.00027819, 0.0002809, 0.00028257, \\
                 0.00028288, 0.00028339, 0.00028355\");
        }

        rise_power(pwr_tin_7) { 
          index_1(\"0.01, 0.043638, 0.16172, 0.3872, 0.73849, \\
                  1.2314, 1.88\");
          values(\"-0.00016661, -0.00016874, -0.00016881, -0.00017039, \\
                 -0.00017043, -0.00017127, -0.00017152\");
        }
      }
  }";
        let (_, _) = named_group_parser(input).unwrap();
    }

    #[test]
    fn test_group_2() {
        let input = "      internal_power() { 
        related_pg_pin : \"VDD\" ; 
        when : \"!B&!CK&!SE&!SI\" ; 

        fall_power(pwr_tin_7) { 
          index_1(\"0.01, 0.043638, 0.16172, 0.3872, 0.73849, \
                  1.2314, 1.88\");
          values(\"0.00027584, 0.00027819, 0.0002809, 0.00028257, \
                 0.00028288, 0.00028339, 0.00028355\");
        }

        rise_power(pwr_tin_7) { 
          index_1(\"0.01, 0.043638, 0.16172, 0.3872, 0.73849, \
                  1.2314, 1.88\");
          values(\"-0.00016661, -0.00016874, -0.00016881, -0.00017039, \
                 -0.00017043, -0.00017127, -0.00017152\");
        }
      }";
        let (_, _) = unnamed_group_parser(input).unwrap();
    }

    #[test]
    fn test_group_3() {
        let input = "      timing() { 
        related_pin : \"CK\" ; 
        sdf_cond : \"ENABLE_B_AND_NOT_SE === 1'b1\" ; 
        timing_type : hold_rising ; 
        when : \"B&!SE\" ; 

        fall_constraint(cnst_ctin_rtin_5x5) { 
          index_1(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          index_2(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          values(\"-0.057255, -0.024717, 0.060606, 0.17141, 0.28867\",\\
                 \"-0.087193, -0.058049, 0.028245, 0.13813, 0.25745\",\\
                 \"-0.19269, -0.16383, -0.083466, 0.025617, 0.14229\",\\
                 \"-0.31589, -0.2968, -0.23804, -0.1371, -0.025855\",\\
                 \"-0.44037, -0.43037, -0.40215, -0.3206, -0.21397\");
        }

        rise_constraint(cnst_ctin_rtin_5x5) { 
          index_1(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          index_2(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          values(\"-0.07729, -0.05616, -0.010551, 0.042665, 0.094022\",\\
                 \"-0.10526, -0.085407, -0.039446, 0.013681, 0.064491\",\\
                 \"-0.19972, -0.1786, -0.13543, -0.080764, -0.030721\",\\
                 \"-0.31012, -0.29114, -0.25219, -0.20107, -0.15284\",\\
                 \"-0.42141, -0.40086, -0.36784, -0.32407, -0.27785\");
        }
      }";
        let (_, _) = unnamed_group_parser(input).unwrap();
    }

    #[test]
    fn test_group_4() {
        let input = "        fall_constraint(cnst_ctin_rtin_5x5) { 
          index_1(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          index_2(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          values(\"-0.057255, -0.024717, 0.060606, 0.17141, 0.28867\",\\
                 \"-0.087193, -0.058049, 0.028245, 0.13813, 0.25745\",\\
                 \"-0.19269, -0.16383, -0.083466, 0.025617, 0.14229\",\\
                 \"-0.31589, -0.2968, -0.23804, -0.1371, -0.025855\",\\
                 \"-0.44037, -0.43037, -0.40215, -0.3206, -0.21397\");
        }";
        let (_, _) = named_group_parser(input).unwrap();
    }

    #[test]
    fn test_group_5() {
        let input = "    pin(E) { 
      clock_gate_enable_pin : true ;
      capacitance : 0.00043968 ; 
      direction : input ; 
      fall_capacitance : 0.00043968 ; 
      input_voltage : default ; 
      related_ground_pin : VSS ; 
      related_power_pin : VDD ; 
      rise_capacitance : 0.00046435 ; 

      internal_power() { 
        related_pg_pin : \"VDD\" ; 
        when : \"!CK&!TE\" ; 

        fall_power(pwr_tin_7) { 
          index_1(\"0.01, 0.043638, 0.16172, 0.3872, 0.73849, \\
                  1.2314, 1.88\");
          values(\"0.00033606, 0.00033708, 0.0003372, 0.00033836, \\
                 0.00033909, 0.00033908, 0.0003391\");
        }

        rise_power(pwr_tin_7) { 
          index_1(\"0.01, 0.043638, 0.16172, 0.3872, 0.73849, \\
                  1.2314, 1.88\");
          values(\"-0.00016751, -0.00017852, -0.00018868, -0.00019433, \\
                 -0.00019602, -0.00019693, -0.00019761\");
        }
      }

      internal_power() { 
        related_pg_pin : \"VNW\" ; 
        when : \"!CK&!TE\" ; 

        fall_power(pwr_tin_7) { 
          index_1(\"0.01, 0.043638, 0.16172, 0.3872, 0.73849, \\
                  1.2314, 1.88\");
          values(\"-6.8348e-05, -6.8781e-05, -6.8928e-05, -6.8762e-05, \\
                 -6.9344e-05, -6.9253e-05, -6.915e-05\");
        }

        rise_power(pwr_tin_7) { 
          index_1(\"0.01, 0.043638, 0.16172, 0.3872, 0.73849, \\
                  1.2314, 1.88\");
          values(\"-3.9319e-05, -3.105e-05, -2.3885e-05, -2.1282e-05, \\
                 -2.0053e-05, -1.9181e-05, -1.8641e-05\");
        }
      }



      timing() { 
        related_pin : \"CK\" ; 
        sdf_cond : \"ENABLE_NOT_TE === 1'b1\" ; 
        timing_type : hold_falling ; 
        when : \"!TE\" ; 

        fall_constraint(cnst_ctin_rtin_5x5) { 
          index_1(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          index_2(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          values(\"-0.13742, -0.11015, -0.056337, 0.008532, 0.074933\",\\
                 \"-0.16798, -0.14027, -0.089483, -0.025991, 0.044863\",\\
                 \"-0.28422, -0.25514, -0.20081, -0.1377, -0.070156\",\\
                 \"-0.45499, -0.4239, -0.36478, -0.29839, -0.22975\",\\
                 \"-0.64895, -0.61816, -0.55728, -0.48904, -0.41512\");
        }

        rise_constraint(cnst_ctin_rtin_5x5) { 
          index_1(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          index_2(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          values(\"-0, 0.037948, 0.14417, 0.27124, 0.41193\",\\
                 \"-0.029062, 0.0059007, 0.10776, 0.23679, 0.38088\",\\
                 \"-0.12892, -0.090057, 0.017739, 0.14903, 0.29162\",\\
                 \"-0.2552, -0.21428, -0.10301, 0.0342, 0.18384\",\\
                 \"-0.38821, -0.35143, -0.23499, -0.092728, 0.059553\");
        }
      }

      timing() { 
        related_pin : \"CK\" ; 
        sdf_cond : \"ENABLE_NOT_TE === 1'b1\" ; 
        timing_type : setup_falling ; 
        when : \"!TE\" ; 

        fall_constraint(cnst_ctin_rtin_5x5) { 
          index_1(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          index_2(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          values(\"0.16032, 0.13235, 0.076675, 0.0072385, -0.061123\",\\
                 \"0.19477, 0.16162, 0.10715, 0.042809, -0.029932\",\\
                 \"0.3071, 0.27586, 0.21827, 0.1554, 0.084428\",\\
                 \"0.4865, 0.45506, 0.38968, 0.31854, 0.24704\",\\
                 \"0.69163, 0.65776, 0.58832, 0.51352, 0.43843\");
        }

        rise_constraint(cnst_ctin_rtin_5x5) { 
          index_1(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          index_2(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          values(\"0.011446, -0.02444, -0.13174, -0.2604, -0.40245\",\\
                 \"0.046986, 0.0092402, -0.092647, -0.22564, -0.36612\",\\
                 \"0.14954, 0.11071, 0, -0.13509, -0.28051\",\\
                 \"0.2867, 0.24567, 0.12776, -0.014949, -0.16666\",\\
                 \"0.43541, 0.39545, 0.27174, 0.12427, -0.032135\");
        }
      }
    }";
        let (_, _) = named_group_parser(input).unwrap();
    }

    #[test]
    fn test_group_6() {
        let input = "      timing() { 
        related_pin : \"CK\" ; 
        sdf_cond : \"ENABLE_NOT_TE === 1'b1\" ; 
        timing_type : hold_falling ; 
        when : \"!TE\" ; 

        fall_constraint(cnst_ctin_rtin_5x5) { 
          index_1(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          index_2(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          values(\"-0.13742, -0.11015, -0.056337, 0.008532, 0.074933\",\\
                 \"-0.16798, -0.14027, -0.089483, -0.025991, 0.044863\",\\
                 \"-0.28422, -0.25514, -0.20081, -0.1377, -0.070156\",\\
                 \"-0.45499, -0.4239, -0.36478, -0.29839, -0.22975\",\\
                 \"-0.64895, -0.61816, -0.55728, -0.48904, -0.41512\");
        }

        rise_constraint(cnst_ctin_rtin_5x5) { 
          index_1(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          index_2(\"0.01, 0.096347, 0.39944, 0.97825, 1.88\");
          values(\"-0, 0.037948, 0.14417, 0.27124, 0.41193\",\\
                 \"-0.029062, 0.0059007, 0.10776, 0.23679, 0.38088\",\\
                 \"-0.12892, -0.090057, 0.017739, 0.14903, 0.29162\",\\
                 \"-0.2552, -0.21428, -0.10301, 0.0342, 0.18384\",\\
                 \"-0.38821, -0.35143, -0.23499, -0.092728, 0.059553\");
        }
      }";
        let (_, _) = unnamed_group_parser(input).unwrap();
    }

    #[test]
    fn test_group_7() {
        let input = "  wire_load(\"Zero\") {
    resistance : 0.00397143;
    capacitance : 0.000206;
    area : 0;
    slope : 0.0;
    fanout_length (1, 0.0);
  }";
        let (_, _) = header_group_parser(input).unwrap();
    }

    #[test]
    fn test_group_8() {
        let input = "      internal_power() { 
        when : \"!B&!CK&!SE&!SI\" ; 

        fall_power(pwr_tin_7) { 
          index_1(\"0.0021, 0.0242957, 0.102208, 0.250991, 0.48279, \
                  0.808029, 1.236\");
          values(\"0.000375434, 0.000376121, 0.000385726, 0.000389842, \
                 0.000392275, 0.000392902, 0.000393199\");
        }

        rise_power(pwr_tin_7) { 
          index_1(\"0.0021, 0.0242957, 0.102208, 0.250991, 0.48279, \
                  0.808029, 1.236\");
          values(\"-0.000107745, -0.000110585, -0.000111607, \
                 -0.000112191, -0.000112566, -0.000112543, -0.000112614\");
        }
      }";
        let (_, _) = unnamed_group_parser(input).unwrap();
    }
}
