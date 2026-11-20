use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::AssertSqlSafe;

use crate::middleware::ApiError;

#[derive(Debug, Clone)]
pub struct Trade {
    pub start_time: DateTime<Utc>,
    pub duration_seconds: i32,
    pub delivery_area_id: String,
    pub quantity: Decimal,
    pub price: Decimal,
    pub trade_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct GroupedTrades {
    pub quantity: Decimal,
    pub amount: Decimal,
    pub delivery_area_id: String,
}

pub enum SourceType {
    Public,
    Private,
}

pub trait TradeSource {
    async fn fetch_trades(&self) -> Result<Vec<Trade>, ApiError>;
}

impl SourceType {
    fn table_name(&self) -> &'static str {
        match self {
            SourceType::Public => "public_trades",
            SourceType::Private => "private_trades",
        }
    }
}

pub fn trades_query(source_type: &SourceType) -> AssertSqlSafe<String> {
    AssertSqlSafe(format!(
        "SELECT start_time, duration_seconds, delivery_area_id, quantity, \
         price, trade_time, side \
         FROM dbo.{} \
         WHERE start_time >= $1 AND start_time < $2 AND delivery_area_id = $3",
        source_type.table_name()
    ))
}
