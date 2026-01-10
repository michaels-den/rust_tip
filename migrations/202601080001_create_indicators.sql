-- migrations/202601080001_create_indicators.sql

CREATE TABLE indicators (
    -- STIX Common Properties
    id TEXT PRIMARY KEY,                       -- e.g., 'indicator--8e2e...'
    type TEXT NOT NULL DEFAULT 'indicator',
    spec_version TEXT NOT NULL DEFAULT '2.1',
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Indicator Specific Properties (Required)
    pattern TEXT NOT NULL,                     -- e.g., "[ipv4-addr:value = '1.1.1.1']"
    pattern_type TEXT NOT NULL,                -- e.g., 'stix', 'sigma', 'snort'
    valid_from TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Optional Properties
    name TEXT,
    confidence INTEGER CHECK (confidence >= 0 AND confidence <= 100)
);

-- Indexing for performance as your feed grows
CREATE UNIQUE INDEX idx_indicators_pattern ON indicators(pattern);
