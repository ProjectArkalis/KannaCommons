fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = tonic_build::configure();
    let conf = conf.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    conf.compile(&["protos/arkalis.proto"], &[""]).unwrap();
    Ok(())
}
