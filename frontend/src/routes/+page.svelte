<script lang="ts">
  import { onMount } from 'svelte';
  import { getAllDocuments } from '$lib/api/documents';
  import type { DocumentPath } from '$lib/api/documents';
  import DocumentList from '$lib/components/DocumentList.svelte';
  import DocumentForm from '$lib/components/DocumentForm.svelte';

  let documents: DocumentPath[] = [];
  let loading = true;
  let error: string | null = null;
  let showCreateForm = false;

  onMount(async () => {
    await loadDocuments();
  });

  async function loadDocuments() {
    loading = true;
    error = null;
    try {
      documents = await getAllDocuments();
    } catch (e) {
      error = e instanceof Error ? e.message : '文書の読み込みに失敗しました';
    } finally {
      loading = false;
    }
  }

  function handleDocumentCreated(doc: DocumentPath) {
    documents = [doc, ...documents];
    showCreateForm = false;
  }

  function handleDocumentDeleted(id: string) {
    // Update the document in the list to mark it as deleted
    documents = documents.map(doc => 
      doc.id === id ? { ...doc, deleted: true } : doc
    );
  }

  $: activeCount = documents.filter(d => !d.deleted).length;
  $: deletedCount = documents.filter(d => d.deleted).length;
</script>

<div class="dashboard">
  <header class="dashboard-header">
    <h1>文書パス管理システム</h1>
    <button class="btn btn-primary" on:click={() => (showCreateForm = !showCreateForm)}>
      {showCreateForm ? '閉じる' : '+ 新規文書作成'}
    </button>
  </header>

  <div class="stats">
    <div class="stat-card">
      <div class="stat-value">{activeCount}</div>
      <div class="stat-label">アクティブ文書</div>
    </div>
    <div class="stat-card">
      <div class="stat-value">{deletedCount}</div>
      <div class="stat-label">削除済み文書</div>
    </div>
    <div class="stat-card">
      <div class="stat-value">{documents.length}</div>
      <div class="stat-label">総文書数</div>
    </div>
  </div>

  {#if showCreateForm}
    <div class="create-form-section">
      <DocumentForm onSuccess={handleDocumentCreated} />
    </div>
  {/if}

  <div class="documents-section">
    <div class="section-header">
      <h2>文書一覧</h2>
      <nav class="quick-links">
        <a href="/documents">すべて表示</a>
      </nav>
    </div>

    {#if loading}
      <div class="loading">文書を読み込み中...</div>
    {:else if error}
      <div class="error-message">{error}</div>
      <button class="btn btn-secondary" on:click={loadDocuments}>再試行</button>
    {:else}
      <DocumentList {documents} onDelete={handleDocumentDeleted} />
    {/if}
  </div>
</div>

<style>
  .dashboard {
    max-width: 1400px;
    margin: 0 auto;
    padding: 24px;
  }

  .dashboard-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 32px;
  }

  h1 {
    margin: 0;
    font-size: 2rem;
    color: #111827;
  }

  h2 {
    margin: 0;
    font-size: 1.25rem;
    color: #111827;
  }

  .stats {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 20px;
    margin-bottom: 32px;
  }

  .stat-card {
    background: white;
    padding: 24px;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .stat-value {
    font-size: 2.5rem;
    font-weight: 700;
    color: #2563eb;
    margin-bottom: 8px;
  }

  .stat-label {
    font-size: 0.875rem;
    color: #6b7280;
    font-weight: 500;
  }

  .create-form-section {
    margin-bottom: 32px;
  }

  .documents-section {
    background: white;
    padding: 24px;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .quick-links a {
    color: #2563eb;
    text-decoration: none;
    font-weight: 500;
  }

  .quick-links a:hover {
    text-decoration: underline;
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

  .btn-primary {
    background: #2563eb;
    color: white;
  }

  .btn-primary:hover {
    background: #1d4ed8;
  }

  .btn-secondary {
    background: #f3f4f6;
    color: #374151;
  }

  .btn-secondary:hover {
    background: #e5e7eb;
  }

  .loading {
    text-align: center;
    padding: 48px;
    color: #6b7280;
    font-size: 1.125rem;
  }

  .error-message {
    padding: 12px;
    background: #fee2e2;
    color: #991b1b;
    border-radius: 4px;
    margin-bottom: 16px;
  }
</style>
