export interface Department {
  code: string;
  name: string;
  sections: string[];
}

export interface DocumentType {
  code: string;
  description: string;
  root_directory: string | null;
  active: boolean;
}

export async function fetchDepartments(): Promise<Department[]> {
  const res = await fetch('/api/departments');
  if (!res.ok) throw new Error('Failed to fetch departments');
  return res.json();
}

export async function fetchDocumentTypes(): Promise<DocumentType[]> {
  const res = await fetch('/api/document-types');
  if (!res.ok) throw new Error('Failed to fetch document types');
  return res.json();
}
