CREATE TABLE notices (
    id BIGSERIAL PRIMARY KEY,

    organization_name VARCHAR(200) NOT NULL,
    source_key VARCHAR(255) NOT NULL,
    source_url TEXT NOT NULL,

    title TEXT NOT NULL,
    author VARCHAR(200),
    category VARCHAR(200),
    body_text TEXT,
    is_pinned BOOLEAN NOT NULL DEFAULT FALSE,
    published_on DATE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notices_source_key
    ON notices (source_key);

CREATE INDEX idx_notices_published_on
    ON notices (published_on);

CREATE INDEX idx_notices_organization_name
    ON notices (organization_name);


CREATE TABLE attachments (
    id BIGSERIAL PRIMARY KEY,
    notice_id BIGINT NOT NULL,

    source_key VARCHAR(255) NOT NULL,
    file_name TEXT NOT NULL,
    download_url TEXT NOT NULL,
    display_order INTEGER NOT NULL DEFAULT 0,

    content_type VARCHAR(255),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_attachments_notice
        FOREIGN KEY (notice_id)
        REFERENCES notices (id)
        ON DELETE CASCADE,

    CONSTRAINT uq_attachments_notice_source_key
        UNIQUE (notice_id, source_key)
);


CREATE TABLE images (
    id BIGSERIAL PRIMARY KEY,
    notice_id BIGINT NOT NULL,

    source_url TEXT NOT NULL,
    alt_text TEXT,
    display_order INTEGER NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_images_notice
        FOREIGN KEY (notice_id)
        REFERENCES notices (id)
        ON DELETE CASCADE,

    CONSTRAINT uq_images_notice_display_order
        UNIQUE (notice_id, display_order)
);
