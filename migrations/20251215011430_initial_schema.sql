-- Portfolio v2 Initial Schema
-- Tables: contact_submissions, page_views, analytics_events
-- Using PostgreSQL 18's native uuidv7() for time-ordered UUIDs (better indexing)

-- Contact form submissions
CREATE TABLE contact_submissions (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    subject VARCHAR(500),
    message TEXT NOT NULL,

    -- Spam protection metadata
    honeypot_filled BOOLEAN DEFAULT FALSE,
    ip_hash VARCHAR(64),  -- Hashed IP for rate limiting (privacy-first)
    user_agent TEXT,

    -- Status tracking
    status VARCHAR(50) DEFAULT 'pending',  -- pending, read, replied, spam
    read_at TIMESTAMPTZ,
    replied_at TIMESTAMPTZ,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- Page views (privacy-first analytics - no cookies, no PII)
CREATE TABLE page_views (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    -- Page info
    path VARCHAR(500) NOT NULL,
    title VARCHAR(255),

    -- Anonymized visitor info (no PII)
    session_hash VARCHAR(64),  -- Daily rotating hash, not trackable across days
    referrer VARCHAR(1000),
    referrer_domain VARCHAR(255),

    -- Device/browser info (non-identifying)
    device_type VARCHAR(50),  -- desktop, mobile, tablet
    browser_family VARCHAR(100),
    os_family VARCHAR(100),
    country_code VARCHAR(2),  -- From IP geolocation, IP not stored

    -- Engagement
    time_on_page_seconds INTEGER,
    scroll_depth_percent INTEGER,

    -- Timestamp
    viewed_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- Analytics events (button clicks, interactions, etc.)
CREATE TABLE analytics_events (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    -- Event info
    event_name VARCHAR(255) NOT NULL,  -- e.g., 'contact_form_submit', 'project_click', 'theme_toggle'
    event_category VARCHAR(100),       -- e.g., 'engagement', 'navigation', 'settings'
    event_data JSONB,                  -- Flexible payload for event-specific data

    -- Page context
    page_path VARCHAR(500),

    -- Session link (same daily rotating hash as page_views)
    session_hash VARCHAR(64),

    -- Timestamp
    occurred_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- Indexes for common queries
CREATE INDEX idx_contact_submissions_created_at ON contact_submissions(created_at DESC);
CREATE INDEX idx_contact_submissions_status ON contact_submissions(status);

CREATE INDEX idx_page_views_path ON page_views(path);
CREATE INDEX idx_page_views_viewed_at ON page_views(viewed_at DESC);
CREATE INDEX idx_page_views_referrer_domain ON page_views(referrer_domain);

CREATE INDEX idx_analytics_events_name ON analytics_events(event_name);
CREATE INDEX idx_analytics_events_occurred_at ON analytics_events(occurred_at DESC);
CREATE INDEX idx_analytics_events_category ON analytics_events(event_category);

-- Trigger for updated_at on contact_submissions
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_contact_submissions_updated_at
    BEFORE UPDATE ON contact_submissions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
