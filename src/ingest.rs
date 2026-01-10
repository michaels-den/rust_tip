use sqlx::PgPool;
use uuid::Uuid;

pub async fn run_binary_defense_ingest(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.binarydefense.com/banlist.txt";
    let response_data = reqwest::get(url).await?;
    let content = response_data.text().await?;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let stix_pattern = format!("[ipv4-addr:value = '{}']", trimmed);
        let id = format!("indicator--{}", Uuid::new_v4());

        sqlx::query!(
            r#"
            INSERT INTO indicators (id, pattern, pattern_type, name)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (pattern) DO NOTHING
            "#,
            id,              // id
            stix_pattern,    // pattern
            "stix",          // pattern_type
            "Binary Defense" // name
        )
        .execute(pool)
        .await?;
    }

    println!("Successfully processed Binary Defense feed.");
    Ok(())
}