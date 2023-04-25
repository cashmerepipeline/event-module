use std::collections::BTreeMap;

/// 信息值
#[derive(Debug, Clone, PartialEq)]
pub enum EventMessageFieldData {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Float32(f32),
    Float64(f64),
    Text(String),
    Time(u64),
    ArrayInt(Vec<i32>),
    ArrayFloat(Vec<f32>),
    ArrayString(Vec<String>),
    Map(BTreeMap<String, String>),
}
