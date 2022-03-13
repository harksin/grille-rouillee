use schema_registry_converter::async_impl::schema_registry::SrSettings;

pub fn get_sr_settings() -> SrSettings {
    let sr_url = String::from("http://localhost:8181");
    SrSettings::new(sr_url)
}
