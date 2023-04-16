use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Features {
    #[serde(rename = "0")]
    pub n0: N0,
    #[serde(rename = "5")]
    pub n5: N5,
    #[serde(rename = "7")]
    pub n7: N7,
    #[serde(rename = "9")]
    pub n9: N9,
    #[serde(rename = "12")]
    pub n12: N12,
    #[serde(rename = "14")]
    pub n14: N14,
    #[serde(rename = "17")]
    pub n17: N17,
    #[serde(rename = "23")]
    pub n23: N23,
    #[serde(rename = "27")]
    pub n27: N27,
    #[serde(rename = "31")]
    pub n31: N31,
    #[serde(rename = "45")]
    pub n45: N45,
    #[serde(rename = "2023")]
    pub n2023: N2023,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N0 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N5 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N7 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N9 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N12 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N14 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N17 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N23 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N27 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N31 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N45 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct N2023 {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}
