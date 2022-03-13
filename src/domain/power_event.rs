use schema_registry_converter::async_impl::schema_registry::{post_schema, SrSettings};
use schema_registry_converter::error::SRCError;
use schema_registry_converter::schema_registry_common::{
    RegisteredSchema, SchemaType, SuppliedSchema,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PowerEvent {
    pub origin: String,
    pub volume: String,
    pub ts: String,
    pub power_type: String,
}

impl PowerEvent {
    pub fn get_schema() -> &'static str {
        r#"
        {
            "type": "record",
            "name": "PowerEvent",
            "fields": [
                {"name": "origin", "type": "string"},
                {"name": "volume", "type": "string"},
                {"name": "ts", "type": "string"},
                {"name": "power_type", "type": "string"}
            ]
        }
    "#
    }

    pub async fn publish_schema(
        sr_settings: &SrSettings,
        subject: String,
    ) -> Result<RegisteredSchema, SRCError> {
        let schema = SuppliedSchema {
            name: Some(String::from("io.polytech.PowerEvent")),
            schema_type: SchemaType::Avro,
            schema: String::from(PowerEvent::get_schema()),
            references: vec![],
        };

        post_schema(&sr_settings, subject.clone(), schema).await
    }
}
