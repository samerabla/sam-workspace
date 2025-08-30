-- This migration script creates the initial tables for the application.
-- Ensure you have the sqlx CLI installed and configured for your database.

-- Create a new migration file with the following command:
-- >> sqlx migrate add create_x_table

-- To run the migration, use the command:
-- >> sqlx migrate run

-- To update the query cache
-- >> cargo sqlx prepare

CREATE TYPE user_role AS ENUM (
    'super_admin', 
    'admin', 
    'user'
);

CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password Text NOT NULL,
    role user_role NOT NULL DEFAULT 'user',
    attributes JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE pending_users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    verification_token TEXT NOT NULL
);

CREATE TABLE password_reset_tokens (
    id Serial Not Null UNIQUE,
    email TEXT NOT NULL,
    token TEXT NOT NULL
);

create table errors (
    id Serial Not Null UNIQUE,
    module VARCHAR(255),
    file VARCHAR(255),
    line INT,
    message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE languages (
    id Serial PRIMARY KEY,
    code TEXT Not Null UNIQUE,
    name TEXT NOT NULL UNIQUE,
    flag TEXT NOT NULL,
    active BOOLEAN NOT NULL
);

CREATE TABLE categories (
    -- This id will be the standard name for the category and will be created manually.
    -- e.g. "property", "property_for_sale", etc.
    id TEXT PRIMARY KEY,
    parent_id TEXT REFERENCES categories(id)
);

CREATE TABLE categories_names (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    language_id INTEGER  NOT NULL REFERENCES languages(id),
    category_id TEXT NOT NULL REFERENCES categories(id),
    slug TEXT NOT NULL UNIQUE,
    UNIQUE(language_id, category_id), -- One name per language per category
    UNIQUE(language_id, slug) -- Unique slug per language
);

CREATE TYPE field_data_type AS ENUM (
    'string', 
    'integer', 
    'decimal',
    'select', 
    'multiselect',
    'boolean',
    'date',
    'file',
    'location'
);

-- Fields definition table
CREATE TABLE fields (
    id TEXT PRIMARY KEY,
    data_type field_data_type NOT NULL DEFAULT 'string',
    is_required BOOLEAN NOT NULL DEFAULT FALSE,
    is_searchable BOOLEAN NOT NULL DEFAULT TRUE,
    is_filterable BOOLEAN NOT NULL DEFAULT FALSE,
    validation_rules JSONB, -- e.g., {"min": 0, "max": 1000000, "pattern": "^[0-9]+$"}
    sort_order INTEGER NOT NULL DEFAULT 0
);

-- Field names (multilingual field names)
CREATE TABLE fields_names (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL,
    placeholder TEXT,
    language_id INTEGER  NOT NULL REFERENCES languages(id) ON DELETE CASCADE,
    field_id TEXT NOT NULL REFERENCES fields(id) ON DELETE CASCADE,
    UNIQUE(language_id, field_id)
);

-- Field options for select/multiselect fields
CREATE TABLE field_options (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    field_id TEXT NOT NULL REFERENCES fields(id) ON DELETE CASCADE,
    option_key TEXT NOT NULL, -- internal key like 'new', 'used', 'refurbished'
    UNIQUE(field_id, option_key)
);

-- Multilingual option names
CREATE TABLE field_options_names (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    option_id INTEGER NOT NULL REFERENCES field_options(id) ON DELETE CASCADE,
    language_id INTEGER  NOT NULL REFERENCES languages(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    UNIQUE(option_id, language_id)
);

-- Junction table: Categories to Fields mapping
CREATE TABLE categories_fields (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    category_id TEXT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    field_id TEXT NOT NULL REFERENCES fields(id) ON DELETE CASCADE,
    UNIQUE(category_id, field_id)
);

-- Listings table (main content table)
CREATE TABLE listings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    category_id TEXT NOT NULL REFERENCES categories(id),
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'draft', -- draft, published, sold, expired
    featured BOOLEAN NOT NULL DEFAULT FALSE,
    views_count INTEGER NOT NULL DEFAULT 0,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Dynamic field values for listings
CREATE TABLE listing_field_values (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    listing_id UUID NOT NULL REFERENCES listings(id) ON DELETE CASCADE,
    field_id TEXT NOT NULL REFERENCES fields(id) ON DELETE CASCADE,
    value_text TEXT, -- for string, text, url, email, phone
    value_integer INTEGER, -- for integer
    value_decimal DOUBLE PRECISION, -- for decimal
    value_boolean BOOLEAN, -- for boolean
    value_date DATE, -- for date
    value_datetime TIMESTAMPTZ, -- for datetime
    value_json JSONB, -- for complex data like location, multiselect
    UNIQUE(listing_id, field_id)
);

CREATE INDEX idx_listing_field_values_json ON listing_field_values USING GIN(value_json);


-- updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_listings_updated_at BEFORE UPDATE ON listings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();