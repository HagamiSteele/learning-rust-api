# trades-api

## Local PostgreSQL test server

The Docker Compose setup starts one PostgreSQL server with two databases:

- `EpexM7`: European two-letter delivery area codes and `BUY`/`SELL` sides.
- `NordpoolIntraday`: integer delivery area IDs from 1 to 20 and `Buy`/`Sell` sides.

Each database contains a `dbo` schema with `private_trades` and `public_trades`.
Private trades are inserted in matching buy/sell pairs with equal quantities,
so their net quantity is zero. Each `dbo.public_trades` table is populated with
1,000,000 deterministic sample rows to support performance testing. Both tables
have indexes on `(start_time, delivery_area_id)` and `trade_time`.

All sample trades use the UTC delivery window from 2026-11-12 23:00 inclusive
to 2026-11-13 23:00 exclusive. Trades are distributed across all 20 areas at
each quarter-hour slot, with multiple public trades per area and slot. The
`duration_seconds` column is `3600` for starts on the hour and `900` for starts
at quarter-hour boundaries (:15, :30, or :45); all start times align to a
quarter-hour boundary.

The `public_trades` table name refers to the type of sample market data; the
database is not exposed publicly. Port 5432 is published on `127.0.0.1` only
and is reachable from the local machine, not from other network hosts.

### Start

From the repository root, run:

```powershell
docker compose up -d
```

The first startup creates the databases, tables, indexes, and sample data; it
may take a few minutes. Check when PostgreSQL is ready with:

```powershell
docker compose ps
```

Connect using host `localhost`, port `5432`, username `trades`, and password
`trades`. For example:

```powershell
docker compose exec postgres psql -U trades -d EpexM7
docker compose exec postgres psql -U trades -d NordpoolIntraday
```

Schema and table names are lowercase, so no quoting is needed in SQL, for
example:

```sql
SELECT COUNT(*) FROM dbo.public_trades;
```

Stop the server but keep its data with:

```powershell
docker compose down
```

To remove the server and its persisted database data and recreate everything
from scratch, run:

```powershell
docker compose down -v
```

The configured credentials are for local development only. Initialization
scripts only run when the PostgreSQL data volume is empty.