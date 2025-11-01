use ::flatbuffers::FlatBufferBuilder;
use prost::Message;
use serde::{Deserialize, Serialize};

pub mod proto_data {
    include!("proto_data.rs");
}

pub mod flat_data {
    include!("flat_data.rs");
}

#[derive(Serialize, Deserialize)]
pub struct TestData {
    pub id: u64,
    pub name: String,
    pub values: Vec<f64>,
    pub metadata: std::collections::HashMap<String, String>,
    pub timestamp: i64,
}

// Pure JSON serialization/deserialization
pub mod json {
    use super::TestData;

    pub fn serialize(
        id: u64,
        name: &str,
        values: &[f64],
        metadata: &std::collections::HashMap<String, String>,
        timestamp: i64,
    ) -> Result<Vec<u8>, serde_json::Error> {
        let data = TestData {
            id,
            name: name.to_string(),
            values: values.to_vec(),
            metadata: metadata.clone(),
            timestamp,
        };
        serde_json::to_vec(&data)
    }

    pub fn deserialize(
        bytes: &[u8],
    ) -> Result<
        (
            u64,
            String,
            Vec<f64>,
            std::collections::HashMap<String, String>,
            i64,
        ),
        serde_json::Error,
    > {
        let data: TestData = serde_json::from_slice(bytes)?;
        Ok((
            data.id,
            data.name,
            data.values,
            data.metadata,
            data.timestamp,
        ))
    }
}

// Pure MessagePack serialization/deserialization
pub mod msgpack {
    use super::TestData;
    use rmp_serde;

    pub fn serialize(
        id: u64,
        name: &str,
        values: &[f64],
        metadata: &std::collections::HashMap<String, String>,
        timestamp: i64,
    ) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        let data = TestData {
            id,
            name: name.to_string(),
            values: values.to_vec(),
            metadata: metadata.clone(),
            timestamp,
        };
        rmp_serde::to_vec(&data)
    }

    pub fn deserialize(
        bytes: &[u8],
    ) -> Result<
        (
            u64,
            String,
            Vec<f64>,
            std::collections::HashMap<String, String>,
            i64,
        ),
        rmp_serde::decode::Error,
    > {
        let data: TestData = rmp_serde::from_slice(bytes)?;
        Ok((
            data.id,
            data.name,
            data.values,
            data.metadata,
            data.timestamp,
        ))
    }
}

// Pure Protobuf serialization/deserialization
pub mod protobuf {
    use super::*;

    pub fn serialize(
        id: u64,
        name: &str,
        values: &[f64],
        metadata: &std::collections::HashMap<String, String>,
        timestamp: i64,
    ) -> Result<Vec<u8>, prost::EncodeError> {
        let proto_data = proto_data::TestData {
            id,
            name: name.to_string(),
            values: values.to_vec(),
            metadata: metadata.clone(),
            timestamp,
        };
        Ok(proto_data.encode_to_vec())
    }

    pub fn deserialize(
        bytes: &[u8],
    ) -> Result<
        (
            u64,
            String,
            Vec<f64>,
            std::collections::HashMap<String, String>,
            i64,
        ),
        prost::DecodeError,
    > {
        let proto_data = proto_data::TestData::decode(bytes)?;
        Ok((
            proto_data.id,
            proto_data.name,
            proto_data.values,
            proto_data.metadata,
            proto_data.timestamp,
        ))
    }
}

// Pure FlatBuffers serialization/deserialization (building structure from scratch)
pub mod flatbuffers {
    use super::*;
    use flat_data::flat_data::{KeyValueArgs, TestDataArgs};

    pub fn serialize(
        id: u64,
        name: &str,
        values: &[f64],
        metadata: &std::collections::HashMap<String, String>,
        timestamp: i64,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut builder = FlatBufferBuilder::new();

        let name_str = builder.create_string(name);

        let values_vector = builder.create_vector(values);

        let mut metadata_offsets = Vec::new();
        for (key, value) in metadata {
            let key_str = builder.create_string(key);
            let value_str = builder.create_string(value);

            let kv_args = KeyValueArgs {
                key: Some(key_str),
                value: Some(value_str),
            };
            let kv = flat_data::flat_data::KeyValue::create(&mut builder, &kv_args);
            metadata_offsets.push(kv);
        }
        let metadata_vector = builder.create_vector(&metadata_offsets);

        let args = TestDataArgs {
            id,
            name: Some(name_str),
            values: Some(values_vector),
            metadata: Some(metadata_vector),
            timestamp,
        };

        let finished_data = flat_data::flat_data::TestData::create(&mut builder, &args);
        builder.finish(finished_data, None);

        Ok(builder.finished_data().to_vec())
    }

    pub fn deserialize(
        bytes: &[u8],
    ) -> Result<
        (
            u64,
            String,
            Vec<f64>,
            std::collections::HashMap<String, String>,
            i64,
        ),
        Box<dyn std::error::Error>,
    > {
        let fb_data = flat_data::flat_data::root_as_test_data(bytes)?;

        let mut metadata = std::collections::HashMap::new();
        if let Some(metadata_vec) = fb_data.metadata() {
            for kv in metadata_vec.iter() {
                if let (Some(key), Some(value)) = (kv.key(), kv.value()) {
                    metadata.insert(key.to_string(), value.to_string());
                }
            }
        }

        let values = fb_data
            .values()
            .map(|vec| vec.iter().collect::<Vec<_>>())
            .unwrap_or_default();

        Ok((
            fb_data.id(),
            fb_data.name().unwrap_or("").to_string(),
            values,
            metadata,
            fb_data.timestamp(),
        ))
    }
}

// Ergonomic FlatBuffers serialization/deserialization using TestData as middleware
pub mod flatbuffers_wrapped {
    use super::*;
    use flat_data::flat_data::{KeyValueArgs, TestDataArgs};

    pub fn serialize(
        id: u64,
        name: &str,
        values: &[f64],
        metadata: &std::collections::HashMap<String, String>,
        timestamp: i64,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // First create TestData struct as middleware
        let test_data = TestData {
            id,
            name: name.to_string(),
            values: values.to_vec(),
            metadata: metadata.clone(),
            timestamp,
        };

        // Then serialize using FlatBuffers
        let mut builder = FlatBufferBuilder::new();

        let name_str = builder.create_string(&test_data.name);

        let values_vector = builder.create_vector(&test_data.values);

        let mut metadata_offsets = Vec::new();
        for (key, value) in &test_data.metadata {
            let key_str = builder.create_string(key);
            let value_str = builder.create_string(value);

            let kv_args = KeyValueArgs {
                key: Some(key_str),
                value: Some(value_str),
            };
            let kv = flat_data::flat_data::KeyValue::create(&mut builder, &kv_args);
            metadata_offsets.push(kv);
        }
        let metadata_vector = builder.create_vector(&metadata_offsets);

        let args = TestDataArgs {
            id: test_data.id,
            name: Some(name_str),
            values: Some(values_vector),
            metadata: Some(metadata_vector),
            timestamp: test_data.timestamp,
        };

        let finished_data = flat_data::flat_data::TestData::create(&mut builder, &args);
        builder.finish(finished_data, None);

        Ok(builder.finished_data().to_vec())
    }

    pub fn deserialize(
        bytes: &[u8],
    ) -> Result<
        (
            u64,
            String,
            Vec<f64>,
            std::collections::HashMap<String, String>,
            i64,
        ),
        Box<dyn std::error::Error>,
    > {
        let fb_data = flat_data::flat_data::root_as_test_data(bytes)?;

        // First deserialize into TestData as middleware
        let mut metadata = std::collections::HashMap::new();
        if let Some(metadata_vec) = fb_data.metadata() {
            for kv in metadata_vec.iter() {
                if let (Some(key), Some(value)) = (kv.key(), kv.value()) {
                    metadata.insert(key.to_string(), value.to_string());
                }
            }
        }

        let values = fb_data
            .values()
            .map(|vec| vec.iter().collect::<Vec<_>>())
            .unwrap_or_default();

        let test_data = TestData {
            id: fb_data.id(),
            name: fb_data.name().unwrap_or("").to_string(),
            values,
            metadata,
            timestamp: fb_data.timestamp(),
        };

        // Then extract fields from TestData
        Ok((
            test_data.id,
            test_data.name,
            test_data.values,
            test_data.metadata,
            test_data.timestamp,
        ))
    }
}

pub fn create_test_data() -> (
    u64,
    String,
    Vec<f64>,
    std::collections::HashMap<String, String>,
    i64,
) {
    let mut values = Vec::with_capacity(1000);
    for i in 0..1000 {
        values.push(i as f64 * 1.5);
    }

    let mut metadata = std::collections::HashMap::new();
    metadata.insert("version".to_string(), "1.0".to_string());
    metadata.insert("type".to_string(), "benchmark".to_string());

    (12345, "test_data".to_string(), values, metadata, 1640995200)
}
