-- Create indexes for performance optimization
-- Migration: 002_indexes
-- Date: 2025-09-30

-- Index on document_number for fast lookup by number
CREATE INDEX IF NOT EXISTS idx_documents_number 
ON documents(document_number) 
WHERE deleted = 0;

-- Index on document_type for filtering by type
CREATE INDEX IF NOT EXISTS idx_documents_type 
ON documents(document_type_code) 
WHERE deleted = 0;

-- Index on department for filtering by department
CREATE INDEX IF NOT EXISTS idx_documents_department 
ON documents(department_code) 
WHERE deleted = 0;

-- Index on section for filtering by section
CREATE INDEX IF NOT EXISTS idx_documents_section 
ON documents(section_code) 
WHERE deleted = 0;

-- Index on business_task for filtering by task
CREATE INDEX IF NOT EXISTS idx_documents_task 
ON documents(business_task_id) 
WHERE deleted = 0 AND business_task_id IS NOT NULL;

-- Index on deleted flag for efficient filtering
CREATE INDEX IF NOT EXISTS idx_documents_deleted 
ON documents(deleted);

-- Index on created_at for chronological queries
CREATE INDEX IF NOT EXISTS idx_documents_created 
ON documents(created_at DESC);

-- Composite index for common query patterns
CREATE INDEX IF NOT EXISTS idx_documents_type_dept_section 
ON documents(document_type_code, department_code, section_code) 
WHERE deleted = 0;
