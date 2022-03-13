# startup sequence
create topics :
```
lyon-bt
west-tht
nuc-001-ht
power-request
```

start city process

``cargo run --bin city-simple -- --prom-port 9902  --input-topic lyon-bt -n lyon``

start plan process

``cargo run --bin nuclear-plant``

start transfo 1 process

``cargo run --bin transfo-simple -- --prom-port 9903  --input-topic nuc-001-ht  --output-topic west-tht --output-power tht``

start transfo 2 process

``cargo run --bin transfo-simple -- --prom-port 9901  --input-topic west-tht  --output-topic lyon-bt --output-power bt``

#druid

``druid-avro-extensions`` to `druid.extensions.loadList` common config


create a data source :
set a custom imput format
```json
      "inputFormat": {
        "type": "avro_stream",
        "binaryAsString": false,
        "avroBytesDecoder": {
          "type": "schema_registry",
          "url": "http://localhost:8181"
        }
      }
```

cast volume to lon
```json
          {
            "type": "expression",
            "name": "volume",
            "expression": "cast('volume','Long')"
          }
```