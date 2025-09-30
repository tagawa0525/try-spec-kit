-- Initial schema for document path management database
-- Migration: 001_initial_schema
-- Date: 2025-09-30

-- Departments table
CREATE TABLE IF NOT EXISTS departments (
    code CHAR(1) PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Sections table
CREATE TABLE IF NOT EXISTS sections (
    code CHAR(1) PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    department_code CHAR(1) NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (department_code) REFERENCES departments(code)
);

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    department_code CHAR(1) NOT NULL,
    section_code CHAR(1) NOT NULL,
    can_create INTEGER NOT NULL DEFAULT 1,
    can_update INTEGER NOT NULL DEFAULT 1,
    can_delete INTEGER NOT NULL DEFAULT 0,
    can_read INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (department_code) REFERENCES departments(code),
    FOREIGN KEY (section_code) REFERENCES sections(code)
);

-- Business tasks table
CREATE TABLE IF NOT EXISTS business_tasks (
    id TEXT PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    department_code CHAR(1),
    section_code CHAR(1),
    active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (department_code) REFERENCES departments(code),
    FOREIGN KEY (section_code) REFERENCES sections(code)
);

-- Document types table
CREATE TABLE IF NOT EXISTS document_types (
    code TEXT PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    root_directory TEXT NOT NULL,
    generation_rule_id INTEGER,
    active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (generation_rule_id) REFERENCES generation_rules(id)
);

-- Path generation rules table
CREATE TABLE IF NOT EXISTS generation_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    components TEXT NOT NULL,  -- JSON array of RuleComponent enum
    separators TEXT NOT NULL,  -- JSON array of separator characters
    counter_scope TEXT NOT NULL,  -- TypeOnly, TypeAndYear, etc.
    counter_digits INTEGER NOT NULL DEFAULT 3,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Documents table (main entity)
CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY NOT NULL,
    document_number TEXT UNIQUE NOT NULL,
    document_type_code TEXT NOT NULL,
    department_code CHAR(1) NOT NULL,
    section_code CHAR(1) NOT NULL,
    business_task_id TEXT,
    user_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    generated INTEGER NOT NULL DEFAULT 1,  -- 1=auto-generated, 0=manual
    deleted INTEGER NOT NULL DEFAULT 0,    -- logical deletion flag
    FOREIGN KEY (document_type_code) REFERENCES document_types(code),
    FOREIGN KEY (department_code) REFERENCES departments(code),
    FOREIGN KEY (section_code) REFERENCES sections(code),
    FOREIGN KEY (business_task_id) REFERENCES business_tasks(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Counters table for auto-increment document numbers
CREATE TABLE IF NOT EXISTS counters (
    scope_key TEXT PRIMARY KEY NOT NULL,
    current_value INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
