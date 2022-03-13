# startup sequence
create topics :

start city process

``cargo run --bin city-simple -- --prom-port 9902  --input-topic lyon-bt -n lyon``

start plan process

``cargo run --bin nuclear-plant``

start transfo 1 process

``cargo run --bin transfo-simple -- --prom-port 9900  --input-topic nuc-001-ht  --output-topic west-tht --output-power tht``

start transfo 2 process

``cargo run --bin transfo-simple -- --prom-port 9901  --input-topic west-tht  --output-topic lion-bt --output-power bt``


