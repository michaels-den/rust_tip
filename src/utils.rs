use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::fs;

// 1. The top-level structure of enterprise-attack.json is a STIX Bundle
#[derive(Debug, Serialize, Deserialize)]
pub struct AttackBundle {
    pub objects: Vec<AttackObject>,
}

// 2. Individual objects (Techniques, Software, Groups) inside the bundle
#[derive(Debug, Serialize, Deserialize)]
pub struct AttackObject {
    pub id: String,
    #[serde(rename = "type")] // 'type' is a reserved keyword in Rust
    pub object_type: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub external_references: Option<Vec<ExternalReference>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalReference {
    pub source_name: String,
    pub external_id: Option<String>,
    pub url: Option<String>,
}

/// Loads the Mitre ATT&CK enterprise file from the local resources directory
pub async fn load_attack_data() -> Result<AttackBundle, Box<dyn Error>> {
    let path = "resources/enterprise-attack.json";
    let content = fs::read_to_string(path).await
        .map_err(|e| format!("Failed to find {} - ensure it is in the resources folder: {}", path, e))?;


    let bundle: AttackBundle = serde_json::from_str(&content)?;

    // print the whole bundle for debugging
    println!("{:#?}", bundle);
    println!("Successfully loaded {} MITRE ATT&CK objects", bundle.objects.len());
    Ok(bundle)
}

// unit test?
#[test]
fn test_load_attack_data() {
    load_attack_data();
}
