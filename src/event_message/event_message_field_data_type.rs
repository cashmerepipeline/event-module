#[derive(Debug, Clone)]
pub enum EventMessageFieldDataType {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Float32,
    Float64,
    Text,
    Time,
    // Array,
    // Map,
}
