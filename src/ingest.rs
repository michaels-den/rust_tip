use sqlx::PgPool;
use uuid::Uuid;

// TODO: offload this and other intel-importer functionality into a separate crate
pub async fn run_binary_defense_ingest(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
let path = "resources/ip-blocklist.txt";
let content = tokio::fs::read_to_string(path).await
    .map_err(|e| format!("Failed to read local file at {}: {}", path, e))?;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let stix_pattern = format!("[ipv4-addr:value = '{}']", trimmed);
        let id = format!("indicator--{}", Uuid::new_v4());

        sqlx::query!(
            r#"
            INSERT INTO indicators (id, pattern, pattern_type, source)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (pattern) DO NOTHING
            "#,
            id,                    // id
            stix_pattern,          // pattern
            "stix",                // pattern_type
            "binarydefense.com"    // source
        )
        .execute(pool)
        .await?;
    }

    println!("Successfully processed Binary Defense feed.");
    Ok(())
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit() {
        let result = 1;
        assert_eq!(result, 1);
    }
}