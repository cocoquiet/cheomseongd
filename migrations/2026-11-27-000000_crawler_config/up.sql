CREATE TABLE crawler_configs (
    crawler_id VARCHAR(100) PRIMARY KEY,
    interval_seconds BIGINT NOT NULL CHECK (interval_seconds > 0),
    notice_count INTEGER NOT NULL CHECK (notice_count > 0)
);
