# Start a cluster with 2 nodes

# create a topic :

./bin/kafka-topics.sh --bootstrap-server localhost:9092 --create --topic equilibrium --partitions 15
--replication-factor 1

# extract actual partition table

bin/kafka-reassign-partitions.sh --bootstrap-server localhost:9092 --broker-list "1,1002" --topics-to-move-json-file
./topic.json --generate

Current partition replica assignment

```json
{
  "version": 1,
  "partitions": [
    {
      "topic": "equilibrium",
      "partition": 0,
      "replicas": [
        1,
        1001
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 1,
      "replicas": [
        1001,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 2,
      "replicas": [
        1,
        1001
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 3,
      "replicas": [
        1001,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 4,
      "replicas": [
        1,
        1001
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 5,
      "replicas": [
        1001,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 6,
      "replicas": [
        1,
        1001
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 7,
      "replicas": [
        1001,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 8,
      "replicas": [
        1,
        1001
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 9,
      "replicas": [
        1001,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 10,
      "replicas": [
        1,
        1001
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 11,
      "replicas": [
        1001,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 12,
      "replicas": [
        1,
        1001
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 13,
      "replicas": [
        1001,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 14,
      "replicas": [
        1,
        1001
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    }
  ]
}
```

Current partition replica assignment
```json
{
  "version": 1,
  "partitions": [
    {
      "topic": "equilibrium",
      "partition": 0,
      "replicas": [
        1002,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 1,
      "replicas": [
        1,
        1002
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 2,
      "replicas": [
        1002,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 3,
      "replicas": [
        1,
        1002
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 4,
      "replicas": [
        1002,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 5,
      "replicas": [
        1,
        1002
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 6,
      "replicas": [
        1002,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 7,
      "replicas": [
        1,
        1002
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 8,
      "replicas": [
        1002,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 9,
      "replicas": [
        1,
        1002
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 10,
      "replicas": [
        1002,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 11,
      "replicas": [
        1,
        1002
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 12,
      "replicas": [
        1002,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 13,
      "replicas": [
        1,
        1002
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    },
    {
      "topic": "equilibrium",
      "partition": 14,
      "replicas": [
        1002,
        1
      ],
      "log_dirs": [
        "any",
        "any"
      ]
    }
  ]
}
```

In this case node 1001 is dead and node 1002 is the new target.


# to execute the reassignment 
bin/kafka-reassign-partitions.sh --bootstrap-server localhost:9092 --reassignment-json-file ./new-assignment.json --execute

# to check the result
bin/kafka-reassign-partitions.sh --bootstrap-server localhost:9092 --reassignment-json-file ./new-assignment.json --verify