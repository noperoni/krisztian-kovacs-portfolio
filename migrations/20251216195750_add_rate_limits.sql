-- Rate limiting table for contact form submissions
-- Tracks submission attempts per IP hash with time windows

CREATE TABLE contact_rate_limits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ip_hash VARCHAR(64) NOT NULL,
    attempt_count INTEGER NOT NULL DEFAULT 1,
    window_start TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_attempt TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique constraint for upsert operations
    CONSTRAINT unique_ip_hash UNIQUE (ip_hash)
);

-- Index for cleanup queries (removing old rate limit records)
CREATE INDEX idx_rate_limits_window_start ON contact_rate_limits(window_start);

-- Comment for documentation
COMMENT ON TABLE contact_rate_limits IS 'Tracks contact form submission attempts for rate limiting (3 per hour per IP)';
