<script lang="ts">
  import type { DocumentPath } from '$lib/api/documents';
  import { deleteDocument } from '$lib/api/documents';

  export let documents: DocumentPath[] = [];
  export let onDelete: ((id: string) => void) | undefined = undefined;
  export let showDeleted: boolean = false;

  let loading = false;
  let error: string | null = null;

  async function handleDelete(id: string) {
    if (!confirm('この文書を削除しますか？（論理削除）')) {
      return;
    }

    loading = true;
    error = null;

    try {
      await deleteDocument(id);
      if (onDelete) {
        onDelete(id);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : '削除に失敗しました';
    } finally {
      loading = false;
    }
  }

  function formatDate(dateStr: string): string {
    return new Date(dateStr).toLocaleString('ja-JP', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  $: filteredDocuments = showDeleted
    ? documents
    : documents.filter((doc) => !doc.deleted);
</script>

<div class="document-list">
  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if filteredDocuments.length === 0}
    <p class="no-documents">文書がありません</p>
  {:else}
    <table>
      <thead>
        <tr>
          <th>文書番号</th>
          <th>文書種類</th>
          <th>部門</th>
          <th>課</th>
          <th>ファイルパス</th>
          <th>作成日時</th>
          <th>状態</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody>
        {#each filteredDocuments as doc (doc.id)}
          <tr class:deleted={doc.deleted}>
            <td class="document-number">
              <a href="/documents/{doc.id}">{doc.document_number}</a>
            </td>
            <td>{doc.document_type}</td>
            <td>{doc.department}</td>
            <td>{doc.section}</td>
            <td class="file-path" title={doc.file_path}>
              {doc.file_path}
            </td>
            <td>{formatDate(doc.created_at)}</td>
            <td>
              {#if doc.deleted}
                <span class="badge deleted-badge">削除済</span>
              {:else if doc.generated}
                <span class="badge generated-badge">自動生成</span>
              {:else}
                <span class="badge manual-badge">手動登録</span>
              {/if}
            </td>
            <td>
              {#if !doc.deleted}
                <button
                  class="btn btn-danger btn-sm"
                  on:click={() => handleDelete(doc.id)}
                  disabled={loading}
                >
                  削除
                </button>
              {/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>

<style>
  .document-list {
    width: 100%;
    overflow-x: auto;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    background: white;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  th,
  td {
    padding: 12px;
    text-align: left;
    border-bottom: 1px solid #e5e7eb;
  }

  th {
    background: #f9fafb;
    font-weight: 600;
    color: #374151;
  }

  tr:hover:not(.deleted) {
    background: #f9fafb;
  }

  tr.deleted {
    opacity: 0.6;
    background: #fef2f2;
  }

  .document-number a {
    color: #2563eb;
    text-decoration: none;
    font-weight: 500;
  }

  .document-number a:hover {
    text-decoration: underline;
  }

  .file-path {
    max-width: 300px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family: monospace;
    font-size: 0.9em;
  }

  .badge {
    padding: 4px 8px;
    border-radius: 4px;
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

  .btn {
    padding: 6px 12px;
    border: none;
    border-radius: 4px;
    font-size: 0.875rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-sm {
    padding: 4px 8px;
    font-size: 0.75rem;
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

  .error {
    padding: 12px;
    background: #fee2e2;
    color: #991b1b;
    border-radius: 4px;
    margin-bottom: 16px;
  }

  .no-documents {
    text-align: center;
    padding: 48px;
    color: #6b7280;
  }
</style>
