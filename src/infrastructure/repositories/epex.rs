use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::Row;

use crate::infrastructure::repositories::trades::*;
use crate::middleware::ApiError;

pub struct EpexSource {
    pub source_type: SourceType,
    pub pool: sqlx::PgPool,
    pub area: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl TradeSource for EpexSource {
    async fn fetch_trades(&self) -> Result<Vec<Trade>, ApiError> {
        let safe_query = trades_query(&self.source_type);

        let rows = sqlx::query(safe_query)
            .bind(self.start)
            .bind(self.end)
            .bind(&self.area)
            .fetch_all(&self.pool)
            .await?;

        let mut trades = Vec::new();
        for row in rows {
            let side: String = row.get("side");
            let raw_quantity: Decimal = row.get("quantity");

            let quantity = if side == "Buy" {
                raw_quantity
            } else {
                -raw_quantity
            };

            trades.push(Trade {
                start_time: row.get("start_time"),
                duration_seconds: row.get("duration_seconds"),
                delivery_area_id: row.get("delivery_area_id"),
                quantity: quantity,
                price: row.get("price"),
                trade_time: row.get("trade_time"),
            })
        }

        return Ok(trades);
    }
}
