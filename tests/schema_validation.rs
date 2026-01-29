use git_atomic::config::Config;
use schemars::schema_for;
use std::fs;

/// Validate all TOML fixtures against the JSON Schema derived from Config.
/// Catches schema drift between fixture files and the actual config types.
#[test]
fn fixtures_match_config_schema() {
    let schema = schema_for!(Config);
    let schema_value = serde_json::to_value(&schema).expect("schema serializes to JSON");
    let compiled = jsonschema::validator_for(&schema_value).expect("schema compiles");

    let mut checked = 0;
    for entry in glob::glob("tests/fixtures/**/*.toml").expect("valid glob") {
        let path = entry.expect("readable dir entry");
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{}: {e}", path.display()));

        // Parse TOML → serde_json::Value for schema validation
        let toml_value: toml::Value = toml::from_str(&content)
            .unwrap_or_else(|e| panic!("{}: invalid TOML: {e}", path.display()));
        let json_value = serde_json::to_value(&toml_value)
            .unwrap_or_else(|e| panic!("{}: TOML→JSON conversion failed: {e}", path.display()));

        if let Err(e) = compiled.validate(&json_value) {
            panic!("{} failed schema validation: {e}", path.display());
        }

        // Also verify it deserializes into Config (catches serde issues the schema misses)
        let _config: Config = toml::from_str(&content)
            .unwrap_or_else(|e| panic!("{}: valid schema but serde error: {e}", path.display()));

        checked += 1;
    }

    assert!(checked > 0, "no fixtures found in tests/fixtures/");
}

/// The sample config from `Config::sample()` must round-trip through
/// serialization → deserialization without loss, ensuring `init` output
/// is always valid.
#[test]
fn sample_config_round_trips() {
    let sample = Config::sample();
    let toml_str = toml::to_string_pretty(&sample).expect("sample serializes");
    let parsed: Config = toml::from_str(&toml_str).expect("sample round-trips");

    assert_eq!(parsed.settings.base_branch, sample.settings.base_branch);
    assert_eq!(parsed.settings.branch_template, sample.settings.branch_template);
    assert_eq!(parsed.components.len(), sample.components.len());
    for (i, comp) in sample.components.iter().enumerate() {
        let p = &parsed.components[i];
        assert_eq!(p.name, comp.name);
        assert_eq!(p.globs, comp.globs);
        assert_eq!(p.commit_type, comp.commit_type);
        assert_eq!(p.branch, comp.branch);
    }
}

/// The sample config must also pass JSON Schema validation.
#[test]
fn sample_config_passes_schema() {
    let schema = schema_for!(Config);
    let schema_value = serde_json::to_value(&schema).expect("schema serializes");
    let compiled = jsonschema::validator_for(&schema_value).expect("schema compiles");

    let sample = Config::sample();
    let toml_str = toml::to_string_pretty(&sample).expect("sample serializes");
    let toml_value: toml::Value = toml::from_str(&toml_str).expect("valid TOML");
    let json_value = serde_json::to_value(&toml_value).expect("TOML→JSON");

    if let Err(e) = compiled.validate(&json_value) {
        panic!("Config::sample() fails its own schema: {e}");
    }
}
