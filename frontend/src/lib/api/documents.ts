/**
 * API Client for Document Path Management
 * Communicates with Rust backend API (axum)
 */

export interface DocumentPath {
  id: string;
  document_number: string;
  document_type: string;
  department: string;
  section: string;
  business_task: string | null;
  user_id: string;
  file_path: string;
  created_at: string;
  updated_at: string;
  generated: boolean;
  deleted: boolean;
}

export interface CreateDocumentRequest {
  type_code: string;
  user_id: string;
  task_id?: string;
}

export interface CreateDocumentManualRequest {
  document_number: string;
  type_code: string;
  file_path: string;
  user_id: string;
  task_id?: string;
}

export interface UpdatePathRequest {
  new_path: string;
}

/**
 * Create document with auto-generated number
 */
export async function createDocumentAuto(
  request: CreateDocumentRequest
): Promise<DocumentPath> {
  const response = await fetch('/api/documents', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message || 'Failed to create document');
  }

  return response.json();
}

/**
 * Create document with manual number
 */
export async function createDocumentManual(
  request: CreateDocumentManualRequest
): Promise<DocumentPath> {
  const response = await fetch('/api/documents/manual', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message || 'Failed to create document');
  }

  return response.json();
}

/**
 * Get document by ID
 */
export async function getDocumentById(id: string): Promise<DocumentPath> {
  const response = await fetch(`/api/documents/${id}`);

  if (!response.ok) {
    throw new Error('Document not found');
  }

  return response.json();
}

/**
 * Get document by number
 */
export async function getDocumentByNumber(number: string): Promise<DocumentPath> {
  const response = await fetch(`/api/documents/number/${encodeURIComponent(number)}`);

  if (!response.ok) {
    throw new Error('Document not found');
  }

  return response.json();
}

/**
 * Update document path
 */
export async function updateDocumentPath(
  id: string,
  newPath: string
): Promise<DocumentPath> {
  const response = await fetch(`/api/documents/${id}/path`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ new_path: newPath }),
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message || 'Failed to update document path');
  }

  return response.json();
}

/**
 * Delete document (logical deletion)
 */
export async function deleteDocument(id: string): Promise<DocumentPath> {
  const response = await fetch(`/api/documents/${id}`, {
    method: 'DELETE',
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message || 'Failed to delete document');
  }

  return response.json();
}

/**
 * Search documents
 */
export async function searchDocuments(query: string): Promise<DocumentPath[]> {
  const response = await fetch(
    `/api/documents/search?q=${encodeURIComponent(query)}`
  );

  if (!response.ok) {
    throw new Error('Search failed');
  }

  return response.json();
}

/**
 * Get all documents
 */
export async function getAllDocuments(): Promise<DocumentPath[]> {
  const response = await fetch('/api/documents');

  if (!response.ok) {
    throw new Error('Failed to fetch documents');
  }

  return response.json();
}
