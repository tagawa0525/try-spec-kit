<script lang="ts">
  import { onMount } from 'svelte';
  import { getAllDocuments } from '$lib/api/documents';
  import type { DocumentPath } from '$lib/api/documents';
  import DocumentList from '$lib/components/DocumentList.svelte';
  import SearchBar from '$lib/components/SearchBar.svelte';

  let documents: DocumentPath[] = [];
  let filteredDocuments: DocumentPath[] = [];
  let loading = true;
  let error: string | null = null;
  let showDeleted = false;
  let isSearching = false;

  onMount(async () => {
    await loadDocuments();
  });

  async function loadDocuments() {
    loading = true;
    error = null;
    try {
      documents = await getAllDocuments();
      filteredDocuments = documents;
      isSearching = false;
    } catch (e) {
      error = e instanceof Error ? e.message : '文書の読み込みに失敗しました';
    } finally {
      loading = false;
    }
  }

  function handleSearch(results: DocumentPath[]) {
    filteredDocuments = results;
    isSearching = true;
  }

  function handleDocumentDeleted(id: string) {
    // Update the document in the list to mark it as deleted
    documents = documents.map(doc => 
      doc.id === id ? { ...doc, deleted: true } : doc
    );
    filteredDocuments = filteredDocuments.map(doc => 
      doc.id === id ? { ...doc, deleted: true } : doc
    );
  }

  function clearSearch() {
    filteredDocuments = documents;
    isSearching = false;
  }

  $: displayedDocuments = showDeleted
    ? filteredDocuments
    : filteredDocuments.filter(d => !d.deleted);
</script>

<div class="documents-page">
  <header class="page-header">
    <div>
      <h1>文書一覧</h1>
      <p class="subtitle">
        {displayedDocuments.length} 件の文書
        {#if isSearching}
          （検索結果）
        {/if}
      </p>
    </div>
    <div class="header-actions">
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={showDeleted} />
        削除済みを表示
      </label>
      <a href="/" class="btn btn-secondary">ダッシュボードに戻る</a>
    </div>
  </header>

  <div class="search-section">
    <SearchBar onSearch={handleSearch} />
    {#if isSearching}
      <button class="btn btn-secondary" on:click={clearSearch}>
        検索をクリア
      </button>
    {/if}
  </div>

  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      文書を読み込み中...
    </div>
  {:else if error}
    <div class="error-container">
      <div class="error-message">{error}</div>
      <button class="btn btn-primary" on:click={loadDocuments}>再試行</button>
    </div>
  {:else}
    <DocumentList documents={displayedDocuments} onDelete={handleDocumentDeleted} {showDeleted} />
  {/if}
</div>

<style>
  .documents-page {
    max-width: 1400px;
    margin: 0 auto;
    padding: 24px;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 32px;
  }

  h1 {
    margin: 0 0 8px 0;
    font-size: 2rem;
    color: #111827;
  }

  .subtitle {
    margin: 0;
    color: #6b7280;
    font-size: 0.875rem;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    font-size: 0.875rem;
    color: #374151;
    user-select: none;
  }

  .checkbox-label input[type='checkbox'] {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }

  .search-section {
    margin-bottom: 24px;
    display: flex;
    gap: 12px;
    align-items: flex-start;
  }

  .btn {
    padding: 10px 20px;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
    text-decoration: none;
    display: inline-block;
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
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 80px 24px;
    color: #6b7280;
    font-size: 1.125rem;
    gap: 16px;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #e5e7eb;
    border-top-color: #2563eb;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .error-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    padding: 48px 24px;
  }

  .error-message {
    padding: 12px 24px;
    background: #fee2e2;
    color: #991b1b;
    border-radius: 4px;
    max-width: 600px;
    text-align: center;
  }

  @media (max-width: 768px) {
    .page-header {
      flex-direction: column;
      gap: 16px;
    }

    .header-actions {
      width: 100%;
      flex-direction: column;
      align-items: stretch;
    }
  }
</style>
