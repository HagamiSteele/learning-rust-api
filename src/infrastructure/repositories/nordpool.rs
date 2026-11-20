use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::Row;

use crate::infrastructure::repositories::trades::*;
use crate::middleware::ApiError;

pub struct NordpoolSource {
    pub source_type: SourceType,
    pub pool: sqlx::PgPool,
    pub area: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

// fn get_area_map() -> HashMap<i32, &str>{
//     let mut area_map = HashMap::new();
//     area_map.insert(1, "DK1");
// }

struct AreaMapper;

impl AreaMapper {
    const MAPPINGS: &[(&str, i32)] = &[
        ("DE", 1),
        ("FR", 2),
        ("NL", 3),
        ("BE", 4),
        ("AT", 5),
        ("CH", 6),
        ("PL", 7),
        ("CZ", 8),
        ("DK", 9),
        ("SE", 10),
        ("NO", 11),
        ("FI", 12),
        ("IT", 13),
        ("ES", 14),
        ("PT", 15),
        ("GB", 16),
        ("IE", 17),
        ("GR", 18),
        ("HU", 19),
        ("RO", 20),
    ];

    pub fn get_code(name: &str) -> Option<i32> {
        Self::MAPPINGS
            .iter()
            .find(|&&(c, _)| c == name)
            .map(|&(_, id)| id)
    }

    pub fn get_name(code: i32) -> Option<&'static str> {
        Self::MAPPINGS
            .iter()
            .find(|&&(_, i)| i == code)
            .map(|&(code, _)| code)
    }
}

impl TradeSource for NordpoolSource {
    async fn fetch_trades(&self) -> Result<Vec<Trade>, ApiError> {
        let safe_query = trades_query(&self.source_type);

        let area_id = AreaMapper::get_code(&self.area).ok_or_else(|| {
            ApiError::BadRequest(format!(
                "Could not convert '{}' to a the right area id for the Nordpool Database",
                self.area
            ))
        })?;

        let rows = sqlx::query(safe_query)
            .bind(self.start)
            .bind(self.end)
            .bind(&area_id)
            .fetch_all(&self.pool)
            .await?;

        let mut np_trades = Vec::new();
        for row in rows {
            let side: String = row.get("side");
            let raw_quantity: Decimal = row.get("quantity");

            let quantity = if side == "Buy" {
                raw_quantity
            } else {
                -raw_quantity
            };

            let row_area: i32 = row.get("delivery_area_id");
            let area = AreaMapper::get_name(row_area).ok_or_else(|| {
                ApiError::BadRequest(format!("Failed to map from db_area '{}'", &row_area))
            })?;

            np_trades.push(Trade {
                start_time: row.get("start_time"),
                duration_seconds: row.get("duration_seconds"),
                delivery_area_id: area.to_string(),
                quantity: quantity,
                price: row.get("price"),
                trade_time: row.get("trade_time"),
            })
        }

        return Ok(np_trades);
    }
}
