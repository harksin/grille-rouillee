use schema_registry_converter::async_impl::schema_registry::{post_schema, SrSettings};
use schema_registry_converter::error::SRCError;
use schema_registry_converter::schema_registry_common::{
    RegisteredSchema, SchemaType, SuppliedSchema,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PowerRequest {
    pub origin: String,
    pub volume: String,
}

impl PowerRequest {
    pub fn get_schema() -> &'static str {
        r#"
        {
            "type": "record",
            "name": "PowerRequest",
            "fields": [
                {"name": "origin", "type": "string"},
                {"name": "volume", "type": "string"}
            ]
        }
    "#
    }

    pub async fn publish_schema(
        sr_settings: &SrSettings,
        subject: String,
    ) -> Result<RegisteredSchema, SRCError> {
        let schema = SuppliedSchema {
            name: Some(String::from("io.polytech.PowerRequest")),
            schema_type: SchemaType::Avro,
            schema: String::from(PowerRequest::get_schema()),
            references: vec![],
        };

        post_schema(&sr_settings, subject.clone(), schema).await
    }
}
