use criterion::{Criterion, black_box, criterion_group, criterion_main};
use serial_bench::{create_test_data, flatbuffers, flatbuffers_wrapped, json, msgpack, protobuf};

fn bench_serialization(c: &mut Criterion) {
    let (id, name, values, metadata, timestamp) = create_test_data();

    c.bench_function("json_serialize", |b| {
        b.iter(|| {
            let _result = json::serialize(
                black_box(id),
                black_box(&name),
                black_box(&values),
                black_box(&metadata),
                black_box(timestamp),
            )
            .unwrap();
        });
    });

    c.bench_function("msgpack_serialize", |b| {
        b.iter(|| {
            let _result = msgpack::serialize(
                black_box(id),
                black_box(&name),
                black_box(&values),
                black_box(&metadata),
                black_box(timestamp),
            )
            .unwrap();
        });
    });

    c.bench_function("protobuf_serialize", |b| {
        b.iter(|| {
            let _result = protobuf::serialize(
                black_box(id),
                black_box(&name),
                black_box(&values),
                black_box(&metadata),
                black_box(timestamp),
            )
            .unwrap();
        });
    });

    c.bench_function("flatbuffers_serialize", |b| {
        b.iter(|| {
            let _result = flatbuffers::serialize(
                black_box(id),
                black_box(&name),
                black_box(&values),
                black_box(&metadata),
                black_box(timestamp),
            )
            .unwrap();
        });
    });

    c.bench_function("flatbuffers_wrapped_serialize", |b| {
        b.iter(|| {
            let _result = flatbuffers_wrapped::serialize(
                black_box(id),
                black_box(&name),
                black_box(&values),
                black_box(&metadata),
                black_box(timestamp),
            )
            .unwrap();
        });
    });
}

fn bench_deserialization(c: &mut Criterion) {
    let (id, name, values, metadata, timestamp) = create_test_data();

    let json_bytes = json::serialize(id, &name, &values, &metadata, timestamp).unwrap();
    let msgpack_bytes = msgpack::serialize(id, &name, &values, &metadata, timestamp).unwrap();
    let protobuf_bytes = protobuf::serialize(id, &name, &values, &metadata, timestamp).unwrap();
    let flatbuffers_bytes =
        flatbuffers::serialize(id, &name, &values, &metadata, timestamp).unwrap();
    let flatbuffers_wrapped_bytes =
        flatbuffers_wrapped::serialize(id, &name, &values, &metadata, timestamp).unwrap();

    c.bench_function("json_deserialize", |b| {
        b.iter(|| {
            let _result = json::deserialize(black_box(&json_bytes)).unwrap();
        });
    });

    c.bench_function("msgpack_deserialize", |b| {
        b.iter(|| {
            let _result = msgpack::deserialize(black_box(&msgpack_bytes)).unwrap();
        });
    });

    c.bench_function("protobuf_deserialize", |b| {
        b.iter(|| {
            let _result = protobuf::deserialize(black_box(&protobuf_bytes)).unwrap();
        });
    });

    c.bench_function("flatbuffers_deserialize", |b| {
        b.iter(|| {
            let _result = flatbuffers::deserialize(black_box(&flatbuffers_bytes)).unwrap();
        });
    });

    c.bench_function("flatbuffers_wrapped_deserialize", |b| {
        b.iter(|| {
            let _result =
                flatbuffers_wrapped::deserialize(black_box(&flatbuffers_wrapped_bytes)).unwrap();
        });
    });
}

fn bench_roundtrip(c: &mut Criterion) {
    let (id, name, values, metadata, timestamp) = create_test_data();

    c.bench_function("json_roundtrip", |b| {
        b.iter(|| {
            let bytes = json::serialize(
                black_box(id),
                black_box(&name),
                black_box(&values),
                black_box(&metadata),
                black_box(timestamp),
            )
            .unwrap();
            let _result = json::deserialize(black_box(&bytes)).unwrap();
        });
    });

    c.bench_function("msgpack_roundtrip", |b| {
        b.iter(|| {
            let bytes = msgpack::serialize(
                black_box(id),
                black_box(&name),
                black_box(&values),
                black_box(&metadata),
                black_box(timestamp),
            )
            .unwrap();
            let _result = msgpack::deserialize(black_box(&bytes)).unwrap();
        });
    });

    c.bench_function("protobuf_roundtrip", |b| {
        b.iter(|| {
            let bytes = protobuf::serialize(
                black_box(id),
                black_box(&name),
                black_box(&values),
                black_box(&metadata),
                black_box(timestamp),
            )
            .unwrap();
            let _result = protobuf::deserialize(black_box(&bytes)).unwrap();
        });
    });

    c.bench_function("flatbuffers_roundtrip", |b| {
        b.iter(|| {
            let bytes = flatbuffers::serialize(
                black_box(id),
                black_box(&name),
                black_box(&values),
                black_box(&metadata),
                black_box(timestamp),
            )
            .unwrap();
            let _result = flatbuffers::deserialize(black_box(&bytes)).unwrap();
        });
    });

    c.bench_function("flatbuffers_wrapped_roundtrip", |b| {
        b.iter(|| {
            let bytes = flatbuffers_wrapped::serialize(
                black_box(id),
                black_box(&name),
                black_box(&values),
                black_box(&metadata),
                black_box(timestamp),
            )
            .unwrap();
            let _result = flatbuffers_wrapped::deserialize(black_box(&bytes)).unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_serialization,
    bench_deserialization,
    bench_roundtrip
);
criterion_main!(benches);
