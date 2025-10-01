<script lang="ts">
  import { createDocumentAuto, createDocumentManual } from '$lib/api/documents';
  import type { DocumentPath } from '$lib/api/documents';

  export let onSuccess: ((doc: DocumentPath) => void) | undefined = undefined;

  let mode: 'auto' | 'manual' = 'auto';
  let typeCode = '';
  let userId = '';
  let taskId = '';
  let documentNumber = '';
  let filePath = '';

  let loading = false;
  let error: string | null = null;
  let success = false;

  async function handleSubmit() {
    error = null;
    success = false;

    // Validation
    if (!typeCode.trim() || !userId.trim()) {
      error = '文書種類とユーザーIDは必須です';
      return;
    }

    if (mode === 'manual') {
      if (!documentNumber.trim() || !filePath.trim()) {
        error = '手動モードでは文書番号とファイルパスが必須です';
        return;
      }
    }

    loading = true;

    try {
      let doc: DocumentPath;

      if (mode === 'auto') {
        doc = await createDocumentAuto({
          type_code: typeCode.trim(),
          user_id: userId.trim(),
          task_id: taskId.trim() || undefined,
        });
      } else {
        doc = await createDocumentManual({
          document_number: documentNumber.trim(),
          type_code: typeCode.trim(),
          file_path: filePath.trim(),
          user_id: userId.trim(),
          task_id: taskId.trim() || undefined,
        });
      }

      success = true;
      resetForm();

      if (onSuccess) {
        onSuccess(doc);
      }
    } catch (e) {
      error = e instanceof Error ? e.message : '文書の作成に失敗しました';
    } finally {
      loading = false;
    }
  }

  function resetForm() {
    typeCode = '';
    userId = '';
    taskId = '';
    documentNumber = '';
    filePath = '';
  }
</script>

<div class="document-form">
  <div class="form-header">
    <h2>文書作成</h2>
    <div class="mode-toggle">
      <button
        class="mode-btn"
        class:active={mode === 'auto'}
        on:click={() => (mode = 'auto')}
      >
        自動生成
      </button>
      <button
        class="mode-btn"
        class:active={mode === 'manual'}
        on:click={() => (mode = 'manual')}
      >
        手動登録
      </button>
    </div>
  </div>

  <form on:submit|preventDefault={handleSubmit}>
    {#if error}
      <div class="alert alert-error">{error}</div>
    {/if}

    {#if success}
      <div class="alert alert-success">文書を作成しました</div>
    {/if}

    {#if mode === 'manual'}
      <div class="form-group">
        <label for="documentNumber">文書番号 *</label>
        <input
          id="documentNumber"
          type="text"
          bind:value={documentNumber}
          placeholder="例: AGI-2509001"
          required
        />
      </div>
    {/if}

    <div class="form-group">
      <label for="typeCode">文書種類コード *</label>
      <input
        id="typeCode"
        type="text"
        bind:value={typeCode}
        placeholder="例: A, りん議, 教育"
        required
      />
      <span class="help-text">1-12文字（マルチバイト対応）</span>
    </div>

    {#if mode === 'manual'}
      <div class="form-group">
        <label for="filePath">ファイルパス *</label>
        <input
          id="filePath"
          type="text"
          bind:value={filePath}
          placeholder="例: /docs/contracts/AGI-2509001"
          required
        />
        <span class="help-text">絶対パスのみ（Windows UNCパス対応）</span>
      </div>
    {/if}

    <div class="form-group">
      <label for="userId">ユーザーID *</label>
      <input
        id="userId"
        type="text"
        bind:value={userId}
        placeholder="例: user001"
        required
      />
    </div>

    <div class="form-group">
      <label for="taskId">業務タスクID（オプション）</label>
      <input
        id="taskId"
        type="text"
        bind:value={taskId}
        placeholder="例: task001"
      />
    </div>

    <div class="form-actions">
      <button type="submit" class="btn btn-primary" disabled={loading}>
        {#if loading}
          作成中...
        {:else if mode === 'auto'}
          自動生成して作成
        {:else}
          手動登録
        {/if}
      </button>
      <button type="button" class="btn btn-secondary" on:click={resetForm}>
        クリア
      </button>
    </div>
  </form>
</div>

<style>
  .document-form {
    background: white;
    padding: 24px;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    max-width: 600px;
  }

  .form-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
  }

  h2 {
    margin: 0;
    font-size: 1.5rem;
    color: #111827;
  }

  .mode-toggle {
    display: flex;
    gap: 8px;
  }

  .mode-btn {
    padding: 6px 12px;
    border: 1px solid #d1d5db;
    background: white;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
    transition: all 0.2s;
  }

  .mode-btn:hover {
    background: #f9fafb;
  }

  .mode-btn.active {
    background: #2563eb;
    color: white;
    border-color: #2563eb;
  }

  .form-group {
    margin-bottom: 20px;
  }

  label {
    display: block;
    margin-bottom: 6px;
    font-weight: 500;
    color: #374151;
  }

  input {
    width: 100%;
    padding: 10px 12px;
    border: 1px solid #d1d5db;
    border-radius: 4px;
    font-size: 1rem;
    transition: border-color 0.2s;
  }

  input:focus {
    outline: none;
    border-color: #2563eb;
    box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
  }

  .help-text {
    display: block;
    margin-top: 4px;
    font-size: 0.875rem;
    color: #6b7280;
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

  .alert-success {
    background: #d1fae5;
    color: #065f46;
  }

  .form-actions {
    display: flex;
    gap: 12px;
    margin-top: 24px;
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

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
