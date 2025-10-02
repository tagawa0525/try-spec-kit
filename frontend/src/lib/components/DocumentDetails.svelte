<script lang="ts">
  import type { DocumentPath } from '$lib/api/documents';
  import { updateDocumentPath, deleteDocument } from '$lib/api/documents';

  export let document: DocumentPath;
  export let onUpdate: ((doc: DocumentPath) => void) | undefined = undefined;
  export let onDelete: (() => void) | undefined = undefined;

  import { onMount } from 'svelte';
  import { departments, documentTypes, loadMetadata } from '$lib/stores/metadata';

  // Fallback maps
  const TYPE_NAME_MAP: Record<string, string> = {
    A: 'AGI (契約)',
    'りん議': 'りん議',
    教育: '教育',
  };

  const DEPT_NAME_MAP: Record<string, string> = {
    G: '総務',
    K: '解析',
  };

  let depsList: any[] = [];
  let typesList: any[] = [];
  const unsubscribeDeps = departments.subscribe((v) => (depsList = v));
  const unsubscribeTypes = documentTypes.subscribe((v) => (typesList = v));

  onMount(() => {
    if (depsList.length === 0 || typesList.length === 0) {
      loadMetadata().catch((e) => console.error('Failed to load metadata', e));
    }
    return () => {
      unsubscribeDeps();
      unsubscribeTypes();
    };
  });

  function displayType(typeCode: string) {
    const found = typesList.find((t) => t.code === typeCode);
    if (found) return `${found.code} (${found.description})`;
    return TYPE_NAME_MAP[typeCode] ?? typeCode;
  }

  function displayDept(deptCode: string) {
    const found = depsList.find((d) => d.code === deptCode);
    if (found) return found.name;
    return DEPT_NAME_MAP[deptCode] ?? deptCode;
  }

  let isEditing = false;
  let newPath = document.file_path;
  let loading = false;
  let error: string | null = null;

  function formatDate(dateStr: string): string {
    return new Date(dateStr).toLocaleString('ja-JP', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  }

  async function handleUpdatePath() {
    if (!newPath.trim()) {
      error = 'パスを入力してください';
      return;
    }

    loading = true;
    error = null;

    try {
      const updated = await updateDocumentPath(document.id, newPath.trim());
      document = updated;
      isEditing = false;
      if (onUpdate) {
        onUpdate(updated);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : 'パスの更新に失敗しました';
    } finally {
      loading = false;
    }
  }

  async function handleDelete() {
    if (!confirm('この文書を削除しますか？（論理削除）')) {
      return;
    }

    loading = true;
    error = null;

    try {
      await deleteDocument(document.id);
      if (onDelete) {
        onDelete();
      }
    } catch (e) {
      error = e instanceof Error ? e.message : '削除に失敗しました';
    } finally {
      loading = false;
    }
  }

  function cancelEdit() {
    isEditing = false;
    newPath = document.file_path;
    error = null;
  }
</script>

<div class="document-details">
  <div class="details-header">
    <h2>{document.document_number}</h2>
    <div class="badges">
      {#if document.deleted}
        <span class="badge deleted-badge">削除済</span>
      {:else if document.generated}
        <span class="badge generated-badge">自動生成</span>
      {:else}
        <span class="badge manual-badge">手動登録</span>
      {/if}
    </div>
  </div>

  {#if error}
    <div class="alert alert-error">{error}</div>
  {/if}

  <div class="details-grid">
    <div class="detail-item">
      <span class="label">文書ID</span>
      <span class="value monospace">{document.id}</span>
    </div>

    <div class="detail-item">
      <span class="label">文書種類</span>
      <span class="value">{displayType(document.document_type)}</span>
    </div>

    <div class="detail-item">
      <span class="label">部門</span>
      <span class="value">{displayDept(document.department)}</span>
    </div>

    <div class="detail-item">
      <span class="label">課</span>
      <span class="value">{document.section}</span>
    </div>

    <div class="detail-item">
      <span class="label">ユーザーID</span>
      <span class="value">{document.user_id}</span>
    </div>

    {#if document.business_task}
      <div class="detail-item">
        <span class="label">業務タスクID</span>
        <span class="value">{document.business_task}</span>
      </div>
    {/if}

    <div class="detail-item full-width">
      <span class="label">ファイルパス</span>
      {#if isEditing}
        <div class="edit-path">
          <input
            type="text"
            bind:value={newPath}
            class="path-input"
            disabled={loading}
          />
          <div class="edit-actions">
            <button
              class="btn btn-primary btn-sm"
              on:click={handleUpdatePath}
              disabled={loading}
            >
              保存
            </button>
            <button class="btn btn-secondary btn-sm" on:click={cancelEdit}>
              キャンセル
            </button>
          </div>
        </div>
      {:else}
        <div class="path-display">
          <span class="value monospace">{document.file_path}</span>
          {#if !document.deleted}
            <button class="btn-edit" on:click={() => (isEditing = true)}>
              編集
            </button>
          {/if}
        </div>
      {/if}
    </div>

    <div class="detail-item">
      <span class="label">作成日時</span>
      <span class="value">{formatDate(document.created_at)}</span>
    </div>

    <div class="detail-item">
      <span class="label">更新日時</span>
      <span class="value">{formatDate(document.updated_at)}</span>
    </div>
  </div>

  {#if !document.deleted}
    <div class="actions">
      <button
        class="btn btn-danger"
        on:click={handleDelete}
        disabled={loading}
      >
        文書を削除
      </button>
    </div>
  {/if}
</div>

<style>
  .document-details {
    background: white;
    padding: 24px;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .details-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
    padding-bottom: 16px;
    border-bottom: 1px solid #e5e7eb;
  }

  h2 {
    margin: 0;
    font-size: 1.5rem;
    color: #111827;
  }

  .badges {
    display: flex;
    gap: 8px;
  }

  .badge {
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .generated-badge {
    background: #dbeafe;
    color: #1e40af;
  }

  .manual-badge {
    background: #fef3c7;
    color: #92400e;
  }

  .deleted-badge {
    background: #fee2e2;
    color: #991b1b;
  }

  .alert {
    padding: 12px;
    border-radius: 4px;
    margin-bottom: 16px;
  }

  .alert-error {
    background: #fee2e2;
    color: #991b1b;
  }

  .details-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 20px;
    margin-bottom: 24px;
  }

  .detail-item {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .detail-item.full-width {
    grid-column: 1 / -1;
  }

  .label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #6b7280;
  }

  .value {
    font-size: 1rem;
    color: #111827;
  }

  .monospace {
    font-family: monospace;
    font-size: 0.9rem;
  }

  .path-display {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .btn-edit {
    padding: 4px 12px;
    border: 1px solid #d1d5db;
    background: white;
    border-radius: 4px;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-edit:hover {
    background: #f9fafb;
    border-color: #2563eb;
    color: #2563eb;
  }

  .edit-path {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .path-input {
    width: 100%;
    padding: 10px 12px;
    border: 1px solid #d1d5db;
    border-radius: 4px;
    font-family: monospace;
    font-size: 0.9rem;
  }

  .path-input:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }

  .edit-actions {
    display: flex;
    gap: 8px;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    padding-top: 16px;
    border-top: 1px solid #e5e7eb;
  }

  .btn {
    padding: 10px 20px;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-sm {
    padding: 6px 12px;
    font-size: 0.875rem;
  }

  .btn-primary {
    background: #2563eb;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #1d4ed8;
  }

  .btn-secondary {
    background: #f3f4f6;
    color: #374151;
  }

  .btn-secondary:hover {
    background: #e5e7eb;
  }

  .btn-danger {
    background: #ef4444;
    color: white;
  }

  .btn-danger:hover:not(:disabled) {
    background: #dc2626;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
