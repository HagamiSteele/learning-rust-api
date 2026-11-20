CREATE DATABASE "NordpoolIntraday";

CREATE SCHEMA dbo;

CREATE TABLE dbo.private_trades (
    start_time timestamptz(0) NOT NULL,
    duration_seconds integer NOT NULL CHECK (duration_seconds IN (3600, 900)),
    delivery_area_id varchar(2) NOT NULL,
    quantity numeric(12, 1) NOT NULL,
    price numeric(12, 2) NOT NULL,
    side varchar(4) NOT NULL CHECK (side IN ('BUY', 'SELL')),
    trade_time timestamptz(3) NOT NULL
);

CREATE TABLE dbo.public_trades (
    start_time timestamptz(0) NOT NULL,
    duration_seconds integer NOT NULL CHECK (duration_seconds IN (3600, 900)),
    delivery_area_id varchar(2) NOT NULL,
    quantity numeric(12, 1) NOT NULL,
    price numeric(12, 2) NOT NULL,
    side varchar(4) NOT NULL CHECK (side IN ('BUY', 'SELL')),
    trade_time timestamptz(3) NOT NULL
);

WITH trades AS (
    SELECT
        n,
        ((n - 1) % 96) AS slot,
        (ARRAY[
            'DE', 'FR', 'NL', 'BE', 'AT', 'CH', 'PL', 'CZ', 'DK', 'SE',
            'NO', 'FI', 'IT', 'ES', 'PT', 'GB', 'IE', 'GR', 'HU', 'RO'
        ])[((n - 1) / 96) + 1] AS delivery_area_id,
        (1 + (n % 1000))::numeric / 10 AS quantity,
        (-50 + (n % 5000)::numeric / 100) AS price
    FROM generate_series(1, 1920) AS n
)
INSERT INTO dbo.private_trades
    (start_time, duration_seconds, delivery_area_id, quantity, price, side, trade_time)
SELECT
    timestamptz '2026-11-12 23:00:00+00' + slot * interval '15 minutes',
    CASE WHEN slot % 4 = 0 THEN 3600 ELSE 900 END,
    delivery_area_id,
    quantity,
    price,
    side,
    timestamptz '2026-11-12 23:00:00+00' + (n % 86400) * interval '1 second'
FROM trades
CROSS JOIN (VALUES ('BUY'), ('SELL')) AS sides(side);

INSERT INTO dbo.public_trades
    (start_time, duration_seconds, delivery_area_id, quantity, price, side, trade_time)
SELECT
    timestamptz '2026-11-12 23:00:00+00' + slot * interval '15 minutes',
    CASE WHEN slot % 4 = 0 THEN 3600 ELSE 900 END,
    (ARRAY[
        'DE', 'FR', 'NL', 'BE', 'AT', 'CH', 'PL', 'CZ', 'DK', 'SE',
        'NO', 'FI', 'IT', 'ES', 'PT', 'GB', 'IE', 'GR', 'HU', 'RO'
    ])[area_index],
    (1 + (n % 1000))::numeric / 10,
    (-50 + (n % 5000)::numeric / 100),
    CASE WHEN n % 2 = 0 THEN 'BUY' ELSE 'SELL' END,
    timestamptz '2026-11-12 23:00:00+00' + ((n - 1) % 86400) * interval '1 second'
FROM (
    SELECT
        n,
        ((n - 1) % 96) AS slot,
        (((n - 1) / 96) % 20) + 1 AS area_index
    FROM generate_series(1, 1000000) AS n
) AS trades;

CREATE INDEX ON dbo.private_trades (start_time, delivery_area_id);
CREATE INDEX ON dbo.private_trades (trade_time);
CREATE INDEX ON dbo.public_trades (start_time, delivery_area_id);
CREATE INDEX ON dbo.public_trades (trade_time);

\connect "NordpoolIntraday"

CREATE SCHEMA dbo;

CREATE TABLE dbo.private_trades (
    start_time timestamptz(0) NOT NULL,
    duration_seconds integer NOT NULL CHECK (duration_seconds IN (3600, 900)),
    delivery_area_id integer NOT NULL CHECK (delivery_area_id BETWEEN 1 AND 20),
    quantity numeric(12, 1) NOT NULL,
    price numeric(12, 2) NOT NULL,
    side varchar(4) NOT NULL CHECK (side IN ('Buy', 'Sell')),
    trade_time timestamptz(3) NOT NULL
);

CREATE TABLE dbo.public_trades (
    start_time timestamptz(0) NOT NULL,
    duration_seconds integer NOT NULL CHECK (duration_seconds IN (3600, 900)),
    delivery_area_id integer NOT NULL CHECK (delivery_area_id BETWEEN 1 AND 20),
    quantity numeric(12, 1) NOT NULL,
    price numeric(12, 2) NOT NULL,
    side varchar(4) NOT NULL CHECK (side IN ('Buy', 'Sell')),
    trade_time timestamptz(3) NOT NULL
);

WITH trades AS (
    SELECT
        n,
        ((n - 1) % 96) AS slot,
        (((n - 1) / 96) + 1)::integer AS delivery_area_id,
        (1 + (n % 1000))::numeric / 10 AS quantity,
        (-50 + (n % 5000)::numeric / 100) AS price
    FROM generate_series(1, 1920) AS n
)
INSERT INTO dbo.private_trades
    (start_time, duration_seconds, delivery_area_id, quantity, price, side, trade_time)
SELECT
    timestamptz '2026-11-12 23:00:00+00' + slot * interval '15 minutes',
    CASE WHEN slot % 4 = 0 THEN 3600 ELSE 900 END,
    delivery_area_id,
    quantity,
    price,
    side,
    timestamptz '2026-11-12 23:00:00+00' + (n % 86400) * interval '1 second'
FROM trades
CROSS JOIN (VALUES ('Buy'), ('Sell')) AS sides(side);

INSERT INTO dbo.public_trades
    (start_time, duration_seconds, delivery_area_id, quantity, price, side, trade_time)
SELECT
    timestamptz '2026-11-12 23:00:00+00' + slot * interval '15 minutes',
    CASE WHEN slot % 4 = 0 THEN 3600 ELSE 900 END,
    area_index::integer,
    (1 + (n % 1000))::numeric / 10,
    (-50 + (n % 5000)::numeric / 100),
    CASE WHEN n % 2 = 0 THEN 'Buy' ELSE 'Sell' END,
    timestamptz '2026-11-12 23:00:00+00' + ((n - 1) % 86400) * interval '1 second'
FROM (
    SELECT
        n,
        ((n - 1) % 96) AS slot,
        (((n - 1) / 96) % 20) + 1 AS area_index
    FROM generate_series(1, 1000000) AS n
) AS trades;

CREATE INDEX ON dbo.private_trades (start_time, delivery_area_id);
CREATE INDEX ON dbo.private_trades (trade_time);
CREATE INDEX ON dbo.public_trades (start_time, delivery_area_id);
CREATE INDEX ON dbo.public_trades (trade_time);
