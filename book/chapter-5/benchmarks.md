# Benchmarks

In terms of benchmarks, the following metrics can be used to evaluate a serialization/deserialization process:
1. **Speed** - how fast is the serialization/deserialization process is compared to other serializers/deserializers
2. **Size** - how much space is needed to store the serialized data (in particular for a schema-less serializer, like FlatMessage, the size needed to store the metadata is relevant)

Additionally, the for `FlatMessage` another relevant metrics are:
1. **Safe** vs **Unsafe** deserialization (and their performance implications)
2. **Zero-copy** deserialization (and its performance implications)
3. **FlatMessageStruct** vs **FlatMessagePacked** for nested structures (and their performance implications in terms of speed and size)