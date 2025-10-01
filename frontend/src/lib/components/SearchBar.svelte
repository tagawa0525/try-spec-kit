<script lang="ts">
  import { searchDocuments } from '$lib/api/documents';
  import type { DocumentPath } from '$lib/api/documents';

  export let onSearch: ((results: DocumentPath[]) => void) | undefined = undefined;

  let query = '';
  let loading = false;
  let error: string | null = null;

  async function handleSearch() {
    if (!query.trim()) {
      error = '検索キーワードを入力してください';
      return;
    }

    loading = true;
    error = null;

    try {
      const results = await searchDocuments(query.trim());
      if (onSearch) {
        onSearch(results);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : '検索に失敗しました';
    } finally {
      loading = false;
    }
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      handleSearch();
    }
  }

  function handleClear() {
    query = '';
    error = null;
    if (onSearch) {
      onSearch([]);
    }
  }
</script>

<div class="search-bar">
  <div class="search-input-wrapper">
    <input
      type="text"
      bind:value={query}
      on:keydown={handleKeyDown}
      placeholder="文書番号、種類、部門、課で検索..."
      class="search-input"
      disabled={loading}
    />
    {#if query}
      <button
        class="clear-btn"
        on:click={handleClear}
        type="button"
        aria-label="クリア"
      >
        ✕
      </button>
    {/if}
  </div>

  <button
    class="search-btn"
    on:click={handleSearch}
    disabled={loading || !query.trim()}
  >
    {#if loading}
      <span class="spinner"></span>
      検索中...
    {:else}
      🔍 検索
    {/if}
  </button>

  {#if error}
    <div class="error-message">{error}</div>
  {/if}
</div>

<style>
  .search-bar {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-bottom: 24px;
  }

  .search-input-wrapper {
    position: relative;
    flex: 1;
    display: flex;
    align-items: center;
  }

  .search-input {
    width: 100%;
    padding: 12px 40px 12px 16px;
    border: 1px solid #d1d5db;
    border-radius: 8px;
    font-size: 1rem;
    transition: all 0.2s;
  }

  .search-input:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }

  .search-input:disabled {
    background: #f3f4f6;
    cursor: not-allowed;
  }

  .clear-btn {
    position: absolute;
    right: 12px;
    width: 24px;
    height: 24px;
    border: none;
    background: #e5e7eb;
    border-radius: 50%;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    color: #6b7280;
    transition: all 0.2s;
  }

  .clear-btn:hover {
    background: #d1d5db;
    color: #374151;
  }

  .search-btn {
    padding: 12px 24px;
    border: none;
    background: #2563eb;
    color: white;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
  }

  .search-btn:hover:not(:disabled) {
    background: #1d4ed8;
  }

  .search-btn:disabled {
    background: #9ca3af;
    cursor: not-allowed;
  }

  .spinner {
    display: inline-block;
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .error-message {
    padding: 12px;
    background: #fee2e2;
    color: #991b1b;
    border-radius: 4px;
    font-size: 0.875rem;
  }

  @media (min-width: 640px) {
    .search-bar {
      flex-direction: row;
      align-items: flex-start;
    }

    .search-btn {
      white-space: nowrap;
    }
  }
</style>
