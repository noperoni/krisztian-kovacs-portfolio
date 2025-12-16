-- GitHub repository cache for portfolio
-- Implements stale-while-revalidate caching pattern

CREATE TABLE github_repos_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Repository data from GitHub API
    github_id BIGINT NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    description TEXT,
    html_url VARCHAR(500) NOT NULL,
    language VARCHAR(100),
    stargazers_count INTEGER NOT NULL DEFAULT 0,
    forks_count INTEGER NOT NULL DEFAULT 0,
    open_issues_count INTEGER NOT NULL DEFAULT 0,
    topics JSONB DEFAULT '[]'::jsonb,

    -- Timestamps from GitHub
    github_created_at TIMESTAMPTZ,
    github_updated_at TIMESTAMPTZ,
    github_pushed_at TIMESTAMPTZ,

    -- Cache metadata
    cached_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,

    -- Soft delete for repos that disappear from GitHub
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Cache metadata table for tracking refresh state
CREATE TABLE github_cache_metadata (
    id INTEGER PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    last_successful_fetch TIMESTAMPTZ,
    last_fetch_attempt TIMESTAMPTZ,
    fetch_error_count INTEGER NOT NULL DEFAULT 0,
    last_error_message TEXT,
    rate_limit_remaining INTEGER,
    rate_limit_reset TIMESTAMPTZ
);

-- Insert singleton metadata row
INSERT INTO github_cache_metadata (id) VALUES (1);

-- Indexes
CREATE INDEX idx_github_repos_cached_at ON github_repos_cache(cached_at DESC);
CREATE INDEX idx_github_repos_expires_at ON github_repos_cache(expires_at);
CREATE INDEX idx_github_repos_stars ON github_repos_cache(stargazers_count DESC);
CREATE INDEX idx_github_repos_active ON github_repos_cache(is_active) WHERE is_active = TRUE;

COMMENT ON TABLE github_repos_cache IS 'Cached GitHub repository data with SWR pattern (5min fresh, 1hr stale)';
COMMENT ON TABLE github_cache_metadata IS 'Singleton table tracking GitHub API fetch state and rate limits';
