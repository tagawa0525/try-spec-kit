import { writable } from 'svelte/store';
import type { Department, DocumentType } from '$lib/api/metadata';
import { fetchDepartments, fetchDocumentTypes } from '$lib/api/metadata';

export const departments = writable<Department[]>([]);
export const documentTypes = writable<DocumentType[]>([]);
export const metadataLoading = writable<boolean>(false);

export async function loadMetadata() {
  metadataLoading.set(true);
  try {
    const [deps, types] = await Promise.all([fetchDepartments(), fetchDocumentTypes()]);
    departments.set(deps);
    documentTypes.set(types);
  } finally {
    metadataLoading.set(false);
  }
}
