<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { getDocumentById } from '$lib/api/documents';
  import type { DocumentPath } from '$lib/api/documents';
  import DocumentDetails from '$lib/components/DocumentDetails.svelte';

  let document: DocumentPath | null = null;
  let loading = true;
  let error: string | null = null;

  $: documentId = $page.params.id;

  onMount(async () => {
    await loadDocument();
  });

  async function loadDocument() {
    if (!documentId) {
      error = '文書IDが指定されていません';
      loading = false;
      return;
    }

    loading = true;
    error = null;

    try {
      document = await getDocumentById(documentId);
    } catch (e) {
      error = e instanceof Error ? e.message : '文書の読み込みに失敗しました';
    } finally {
      loading = false;
    }
  }

  function handleUpdate(updatedDoc: DocumentPath) {
    document = updatedDoc;
  }

  function handleDelete() {
    // Navigate back to documents list after deletion
    goto('/documents');
  }
</script>

<div class="document-detail-page">
  <nav class="breadcrumb">
    <a href="/">ダッシュボード</a>
    <span class="separator">/</span>
    <a href="/documents">文書一覧</a>
    <span class="separator">/</span>
    <span class="current">
      {#if document}
        {document.document_number}
      {:else}
        文書詳細
      {/if}
    </span>
  </nav>

  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      文書を読み込み中...
    </div>
  {:else if error}
    <div class="error-container">
      <div class="error-message">{error}</div>
      <div class="error-actions">
        <button class="btn btn-primary" on:click={loadDocument}>再試行</button>
        <a href="/documents" class="btn btn-secondary">文書一覧に戻る</a>
      </div>
    </div>
  {:else if document}
    <DocumentDetails {document} onUpdate={handleUpdate} onDelete={handleDelete} />
  {:else}
    <div class="error-container">
      <div class="error-message">文書が見つかりませんでした</div>
      <a href="/documents" class="btn btn-secondary">文書一覧に戻る</a>
    </div>
  {/if}
</div>

<style>
  .document-detail-page {
    max-width: 1200px;
    margin: 0 auto;
    padding: 24px;
  }

  .breadcrumb {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 24px;
    font-size: 0.875rem;
    color: #6b7280;
  }

  .breadcrumb a {
    color: #2563eb;
    text-decoration: none;
  }

  .breadcrumb a:hover {
    text-decoration: underline;
  }

  .breadcrumb .separator {
    color: #d1d5db;
  }

  .breadcrumb .current {
    color: #111827;
    font-weight: 500;
  }

  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 120px 24px;
    color: #6b7280;
    font-size: 1.125rem;
    gap: 16px;
  }

  .spinner {
    width: 48px;
    height: 48px;
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
    gap: 24px;
    padding: 80px 24px;
  }

  .error-message {
    padding: 16px 32px;
    background: #fee2e2;
    color: #991b1b;
    border-radius: 8px;
    max-width: 600px;
    text-align: center;
    font-size: 1.125rem;
  }

  .error-actions {
    display: flex;
    gap: 12px;
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
</style>
