<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { onMount, onDestroy } from 'svelte';
  import { Button, Input } from '$lib/components/form';
  import { MatrixDisplay } from '$lib/components/data';
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import { experimentsStore } from '$lib/stores/experiments.svelte';
  import { context } from '$lib/stores/app.svelte';
  import type { OAData } from '$lib/types';

  // Current array
  let array = $state<OAData | null>(null);
  let loading = $state(true);

  // View options
  let showHeaders = $state(true);
  let compactView = $state(false);

  // Selection state
  let selectedRow = $state<number | null>(null);
  let selectedCol = $state<number | null>(null);

  // Export feedback
  let exportMessage = $state<string | null>(null);

  // Edit mode
  let isEditing = $state(false);
  let editName = $state('');
  let editNotes = $state('');

  // Clean up context when leaving the page
  onDestroy(() => {
    context.clear();
  });

  // Update context panel when array or selection changes
  $effect(() => {
    if (array) {
      context.set('array-details', {
        array,
        selectedRow,
        selectedCol,
        onEdit: startEditing,
      });
    }
  });

  // Load array on mount
  onMount(() => {
    const id = $page.params.id;
    if (!id) {
      loading = false;
      return;
    }

    const found = arrayStore.get(id);
    if (found) {
      array = found;
      arrayStore.setCurrent(id);
    }
    loading = false;
  });

  // Handle cell selection
  function handleCellClick(row: number, col: number, _value: number) {
    selectedRow = selectedRow === row ? null : row;
    selectedCol = selectedCol === col ? null : col;
  }

  // Show temporary message
  function showMessage(msg: string) {
    exportMessage = msg;
    setTimeout(() => exportMessage = null, 3000);
  }

  // Export handlers
  async function exportCSV() {
    if (!array) return;
    try {
      const path = await experimentsStore.exportCSV(array);
      if (path) {
        showMessage(`Exported to ${path.split('/').pop()}`);
      }
    } catch (e) {
      showMessage(`Export failed: ${e}`);
    }
  }

  async function exportJSON() {
    if (!array) return;
    try {
      const path = await experimentsStore.exportJSON(array);
      if (path) {
        showMessage(`Exported to ${path.split('/').pop()}`);
      }
    } catch (e) {
      showMessage(`Export failed: ${e}`);
    }
  }

  async function exportLaTeX() {
    if (!array) return;
    try {
      await experimentsStore.copyLaTeXToClipboard(array);
      showMessage('LaTeX copied to clipboard');
    } catch (e) {
      showMessage(`Export failed: ${e}`);
    }
  }

  // Navigate to array verification
  function goToVerification() {
    if (array) {
      goto(`/analysis/${array.id}`);
    }
  }

  // Start editing
  function startEditing() {
    if (array) {
      editName = array.metadata.name || '';
      editNotes = array.metadata.notes || '';
      isEditing = true;
    }
  }

  // Save edits
  async function saveEdits() {
    if (array) {
      const updated = await arrayStore.updateMetadata(array.id, {
        name: editName.trim() || undefined,
        notes: editNotes.trim() || undefined,
      });
      if (updated) {
        array = updated;
      }
      isEditing = false;
      showMessage('Changes saved');
    }
  }

  // Cancel editing
  function cancelEditing() {
    isEditing = false;
  }

  // Computed notation
  let notation = $derived(
    array
      ? `OA(${array.runs}, ${array.factors}, ${array.levels[0]}, ${array.strength})`
      : ''
  );

  // Format date
  function formatDate(isoString: string): string {
    try {
      return new Date(isoString).toLocaleString();
    } catch {
      return isoString;
    }
  }
</script>

<div class="viewer-page">
  {#if loading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>Loading array...</p>
    </div>
  {:else if !array}
    <div class="error-state">
      <h2>Array Not Found</h2>
      <p>The requested array could not be found in the current session.</p>
      <Button variant="primary" onclick={() => goto('/catalogue')}>
        {#snippet children()}
          Browse Catalogue
        {/snippet}
      </Button>
    </div>
  {:else}
    <!-- Export Message Toast -->
    {#if exportMessage}
      <div class="export-toast">
        {exportMessage}
      </div>
    {/if}

    <!-- Header -->
    <header class="page-header">
      <div class="header-main">
        {#if isEditing}
          <input
            type="text"
            class="edit-title-input"
            bind:value={editName}
            placeholder="Array name..."
          />
        {:else}
          <h1 class="page-title">
            {array.metadata.name || 'Orthogonal Array'}
          </h1>
        {/if}
        <span class="array-notation">{notation}</span>
      </div>
      <div class="header-actions">
        {#if isEditing}
          <Button variant="primary" onclick={saveEdits}>
            {#snippet children()}
              Save
            {/snippet}
          </Button>
          <Button variant="ghost" onclick={cancelEditing}>
            {#snippet children()}
              Cancel
            {/snippet}
          </Button>
        {:else}
          <Button variant="ghost" onclick={startEditing}>
            {#snippet children()}
              Edit
            {/snippet}
          </Button>
          <Button variant="ghost" onclick={goToVerification}>
            {#snippet children()}
              Verify
            {/snippet}
          </Button>
          <div class="export-dropdown">
            <Button variant="secondary" loading={experimentsStore.loading}>
              {#snippet children()}
                Export
              {/snippet}
            </Button>
            <div class="dropdown-menu">
              <button class="dropdown-item" onclick={exportCSV}>CSV</button>
              <button class="dropdown-item" onclick={exportJSON}>JSON</button>
              <button class="dropdown-item" onclick={exportLaTeX}>LaTeX (Copy)</button>
            </div>
          </div>
        {/if}
      </div>
    </header>

    <div class="viewer-layout">
      <!-- Main Matrix View -->
      <main class="matrix-section">
        <div class="view-controls">
          <label class="control-toggle">
            <input type="checkbox" bind:checked={showHeaders} />
            <span>Show Headers</span>
          </label>
          <label class="control-toggle">
            <input type="checkbox" bind:checked={compactView} />
            <span>Compact View</span>
          </label>
        </div>

        <MatrixDisplay
          data={array}
          {showHeaders}
          compact={compactView}
          selectable={true}
          highlightRow={selectedRow}
          highlightCol={selectedCol}
          onCellClick={handleCellClick}
        />
      </main>
    </div>
  {/if}
</div>

<style>
  .viewer-page {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: var(--space-6);
    flex-shrink: 0;
  }

  .header-main {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .page-title {
    font-size: var(--text-3xl);
    font-weight: var(--font-bold);
    color: var(--color-text-primary);
  }

  .array-notation {
    font-family: var(--font-mono);
    font-size: var(--text-lg);
    color: var(--color-text-muted);
  }

  .header-actions {
    display: flex;
    gap: var(--space-2);
  }

  .export-dropdown {
    position: relative;
  }

  .dropdown-menu {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: var(--space-1);
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-lg);
    min-width: 120px;
    opacity: 0;
    visibility: hidden;
    transform: translateY(-4px);
    transition: all var(--duration-fast);
    z-index: 10;
  }

  .export-dropdown:hover .dropdown-menu,
  .export-dropdown:focus-within .dropdown-menu {
    opacity: 1;
    visibility: visible;
    transform: translateY(0);
  }

  .dropdown-item {
    display: block;
    width: 100%;
    padding: var(--space-2) var(--space-3);
    text-align: left;
    background: none;
    border: none;
    color: var(--color-text-primary);
    cursor: pointer;
    transition: background-color var(--duration-fast);
  }

  .dropdown-item:hover {
    background-color: var(--color-bg-secondary);
  }

  .dropdown-item:first-child {
    border-radius: var(--radius-md) var(--radius-md) 0 0;
  }

  .dropdown-item:last-child {
    border-radius: 0 0 var(--radius-md) var(--radius-md);
  }

  /* Layout */
  .viewer-layout {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
  }

  /* Matrix Section */
  .matrix-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    min-height: 0;
    overflow: auto;
  }

  .view-controls {
    display: flex;
    gap: var(--space-4);
    padding: var(--space-3);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
    flex-shrink: 0;
  }

  .control-toggle {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    cursor: pointer;
  }

  .control-toggle input[type="checkbox"] {
    width: 16px;
    height: 16px;
    accent-color: var(--color-accent);
  }

  /* States */
  .loading-state,
  .error-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--space-4);
    padding: var(--space-12);
    color: var(--color-text-muted);
    flex: 1;
  }

  .error-state h2 {
    font-size: var(--text-xl);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  /* Export Toast */
  .export-toast {
    position: fixed;
    bottom: var(--space-6);
    right: var(--space-6);
    padding: var(--space-3) var(--space-5);
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-accent);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-lg);
    font-size: var(--text-sm);
    color: var(--color-text-primary);
    z-index: 100;
    animation: slideIn 0.2s ease-out;
  }

  @keyframes slideIn {
    from {
      transform: translateY(20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  /* Edit Mode Styles */
  .edit-title-input {
    font-size: var(--text-3xl);
    font-weight: var(--font-bold);
    color: var(--color-text-primary);
    background-color: var(--color-bg-secondary);
    border: 2px solid var(--color-accent);
    border-radius: var(--radius-md);
    padding: var(--space-2) var(--space-3);
    width: 100%;
    max-width: 400px;
    outline: none;
  }

  .notes-input {
    width: 100%;
    padding: var(--space-2) var(--space-3);
    font-size: var(--text-sm);
    font-family: inherit;
    color: var(--color-text-primary);
    background-color: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    resize: vertical;
    min-height: 80px;
  }

  .notes-input:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: var(--shadow-focus);
  }

  .action-buttons {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    margin-top: var(--space-3);
  }

  .delete-confirm {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-3);
    background-color: var(--color-error-subtle);
    border-radius: var(--radius-md);
  }

  .delete-warning {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-error);
    text-align: center;
  }

  .delete-actions {
    display: flex;
    gap: var(--space-2);
    justify-content: center;
  }
</style>
