<script lang="ts">
  import { goto } from '$app/navigation';
  import { Button } from '$lib/components/form';
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import { experimentsStore } from '$lib/stores/experiments.svelte';
  import type { OAData, ImportValidation } from '$lib/types';
  import { nanoid } from 'nanoid';

  // Import state
  let importedData = $state<number[][] | null>(null);
  let importValidation = $state<ImportValidation | null>(null);
  let importFileName = $state<string | null>(null);
  let importError = $state<string | null>(null);

  // Comparison state
  let compareMode = $state(false);
  let selectedForCompare = $state<Set<string>>(new Set());

  // Get all arrays in session
  let arrays = $derived(arrayStore.list);

  // Import CSV handler
  async function handleImportCSV() {
    importError = null;
    importedData = null;
    importValidation = null;

    try {
      const result = await experimentsStore.importCSV();
      if (result) {
        importedData = result.data;
        importFileName = result.path.split('/').pop() ?? 'imported.csv';
        importValidation = await experimentsStore.validateImport(result.data);
      }
    } catch (e) {
      importError = String(e);
    }
  }

  // Import JSON handler
  async function handleImportJSON() {
    importError = null;
    importedData = null;
    importValidation = null;

    try {
      const oa = await experimentsStore.importJSON();
      if (oa) {
        arrayStore.add(oa);
        arrayStore.setCurrent(oa.id);
        goto(`/viewer/${oa.id}`);
      }
    } catch (e) {
      importError = String(e);
    }
  }

  // Accept imported data and add to arrays
  function acceptImport() {
    if (!importedData || !importValidation) return;

    const oa: OAData = {
      id: nanoid(),
      runs: importValidation.runs,
      factors: importValidation.factors,
      levels: importValidation.levels,
      strength: importValidation.estimatedStrength,
      data: importedData,
      metadata: {
        name: importFileName?.replace(/\.[^/.]+$/, '') ?? 'Imported Array',
        algorithm: 'Imported',
        createdAt: new Date().toISOString(),
        notes: `Imported from ${importFileName}`,
      },
    };

    arrayStore.add(oa);
    arrayStore.setCurrent(oa.id);

    // Reset import state
    importedData = null;
    importValidation = null;
    importFileName = null;

    goto(`/viewer/${oa.id}`);
  }

  // Cancel import
  function cancelImport() {
    importedData = null;
    importValidation = null;
    importFileName = null;
    importError = null;
  }

  // Toggle compare selection
  function toggleCompare(id: string) {
    const newSet = new Set(selectedForCompare);
    if (newSet.has(id)) {
      newSet.delete(id);
    } else if (newSet.size < 2) {
      newSet.add(id);
    }
    selectedForCompare = newSet;
  }

  // View array
  function viewArray(id: string) {
    goto(`/viewer/${id}`);
  }

  // Analyze array
  function analyzeArray(id: string) {
    goto(`/analysis/${id}`);
  }

  // Remove array from session
  function removeArray(id: string) {
    arrayStore.remove(id);
    selectedForCompare.delete(id);
    selectedForCompare = new Set(selectedForCompare);
  }

  // Compare selected arrays
  let comparedArrays = $derived(
    Array.from(selectedForCompare).map((id) => arrayStore.get(id)).filter(Boolean) as OAData[]
  );
</script>

<div class="experiments-page">
  <header class="page-header">
    <div class="header-main">
      <h1 class="page-title">Experiments</h1>
      <p class="page-subtitle">Import, manage, and compare orthogonal arrays</p>
    </div>
    <div class="header-actions">
      <Button variant="secondary" onclick={handleImportCSV} loading={experimentsStore.loading}>
        {#snippet children()}
          Import CSV
        {/snippet}
      </Button>
      <Button variant="secondary" onclick={handleImportJSON} loading={experimentsStore.loading}>
        {#snippet children()}
          Import JSON
        {/snippet}
      </Button>
    </div>
  </header>

  {#if importError}
    <div class="error-banner">
      <p>{importError}</p>
      <Button variant="ghost" size="sm" onclick={() => importError = null}>
        {#snippet children()}
          Dismiss
        {/snippet}
      </Button>
    </div>
  {/if}

  <!-- Import Preview Panel -->
  {#if importedData && importValidation}
    <section class="import-preview card">
      <h2 class="card-title">Import Preview</h2>

      <div class="import-details">
        <div class="detail-grid">
          <div class="detail">
            <span class="detail-label">File</span>
            <span class="detail-value">{importFileName}</span>
          </div>
          <div class="detail">
            <span class="detail-label">Runs</span>
            <span class="detail-value">{importValidation.runs}</span>
          </div>
          <div class="detail">
            <span class="detail-label">Factors</span>
            <span class="detail-value">{importValidation.factors}</span>
          </div>
          <div class="detail">
            <span class="detail-label">Levels</span>
            <span class="detail-value">
              {importValidation.isMixed
                ? importValidation.levels.join(', ')
                : importValidation.levels[0]}
            </span>
          </div>
          <div class="detail">
            <span class="detail-label">Est. Strength</span>
            <span class="detail-value">{importValidation.estimatedStrength}</span>
          </div>
          <div class="detail">
            <span class="detail-label">Type</span>
            <span class="detail-value">{importValidation.isMixed ? 'Mixed' : 'Symmetric'}</span>
          </div>
        </div>

        {#if importValidation.warnings.length > 0}
          <div class="warnings">
            <h4 class="warnings-title">Warnings</h4>
            <ul class="warnings-list">
              {#each importValidation.warnings as warning}
                <li>{warning}</li>
              {/each}
            </ul>
          </div>
        {/if}

        <!-- Data Preview -->
        <div class="data-preview">
          <h4 class="preview-title">Data Preview (first 5 rows)</h4>
          <div class="preview-table-wrapper">
            <table class="preview-table">
              <thead>
                <tr>
                  <th>#</th>
                  {#each Array(importValidation.factors) as _, i}
                    <th>F{i + 1}</th>
                  {/each}
                </tr>
              </thead>
              <tbody>
                {#each importedData.slice(0, 5) as row, i}
                  <tr>
                    <td class="row-num">{i + 1}</td>
                    {#each row as cell}
                      <td>{cell}</td>
                    {/each}
                  </tr>
                {/each}
                {#if importedData.length > 5}
                  <tr class="more-rows">
                    <td colspan={importValidation.factors + 1}>
                      ... and {importedData.length - 5} more rows
                    </td>
                  </tr>
                {/if}
              </tbody>
            </table>
          </div>
        </div>

        <div class="import-actions">
          <Button variant="ghost" onclick={cancelImport}>
            {#snippet children()}
              Cancel
            {/snippet}
          </Button>
          <Button variant="primary" onclick={acceptImport}>
            {#snippet children()}
              Accept &amp; View
            {/snippet}
          </Button>
        </div>
      </div>
    </section>
  {/if}

  <!-- Session Arrays -->
  <section class="arrays-section">
    <div class="section-header">
      <h2 class="section-title">Session Arrays ({arrays.length})</h2>
      {#if arrays.length >= 2}
        <Button
          variant={compareMode ? 'primary' : 'ghost'}
          size="sm"
          onclick={() => {
            compareMode = !compareMode;
            if (!compareMode) selectedForCompare = new Set();
          }}
        >
          {#snippet children()}
            {compareMode ? 'Exit Compare' : 'Compare'}
          {/snippet}
        </Button>
      {/if}
    </div>

    {#if arrays.length === 0}
      <div class="empty-state">
        <p>No arrays in current session.</p>
        <p class="hint">Build arrays in the Builder, select from the Catalogue, or import files.</p>
      </div>
    {:else}
      <div class="arrays-grid">
        {#each arrays as oa}
          <div
            class="array-card"
            class:selected={selectedForCompare.has(oa.id)}
            class:compare-mode={compareMode}
          >
            {#if compareMode}
              <button class="compare-checkbox" onclick={() => toggleCompare(oa.id)}>
                {#if selectedForCompare.has(oa.id)}
                  <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                    <path d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"/>
                  </svg>
                {:else}
                  <svg width="16" height="16" viewBox="0 0 16 16" stroke="currentColor" fill="none">
                    <rect x="2" y="2" width="12" height="12" rx="2" stroke-width="1.5"/>
                  </svg>
                {/if}
              </button>
            {/if}

            <div class="card-header">
              <h3 class="card-name">{oa.metadata.name || 'Unnamed Array'}</h3>
              <span class="card-notation">
                OA({oa.runs}, {oa.factors}, {oa.levels[0]}, {oa.strength})
              </span>
            </div>

            <div class="card-meta">
              <span class="meta-item">{oa.metadata.algorithm}</span>
              <span class="meta-item">{new Date(oa.metadata.createdAt).toLocaleDateString()}</span>
            </div>

            <div class="card-actions">
              <Button variant="ghost" size="sm" onclick={() => viewArray(oa.id)}>
                {#snippet children()}
                  View
                {/snippet}
              </Button>
              <Button variant="ghost" size="sm" onclick={() => analyzeArray(oa.id)}>
                {#snippet children()}
                  Analyze
                {/snippet}
              </Button>
              <Button variant="ghost" size="sm" onclick={() => removeArray(oa.id)}>
                {#snippet children()}
                  Remove
                {/snippet}
              </Button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </section>

  <!-- Comparison Panel -->
  {#if compareMode && comparedArrays.length === 2}
    <section class="compare-section card">
      <h2 class="card-title">Comparison</h2>

      <div class="compare-grid">
        {#each comparedArrays as oa, idx}
          <div class="compare-column">
            <h3 class="compare-name">{oa.metadata.name || `Array ${idx + 1}`}</h3>
            <dl class="compare-props">
              <div class="prop-row">
                <dt>Runs</dt>
                <dd>{oa.runs}</dd>
              </div>
              <div class="prop-row">
                <dt>Factors</dt>
                <dd>{oa.factors}</dd>
              </div>
              <div class="prop-row">
                <dt>Levels</dt>
                <dd>{oa.levels[0]}</dd>
              </div>
              <div class="prop-row">
                <dt>Strength</dt>
                <dd>{oa.strength}</dd>
              </div>
              <div class="prop-row">
                <dt>Algorithm</dt>
                <dd>{oa.metadata.algorithm}</dd>
              </div>
            </dl>
          </div>
        {/each}
      </div>

      {#if comparedArrays[0] && comparedArrays[1]}
        <div class="compare-summary">
          <h4>Differences</h4>
          <ul class="diff-list">
            {#if comparedArrays[0].runs !== comparedArrays[1].runs}
              <li>Runs differ: {comparedArrays[0].runs} vs {comparedArrays[1].runs}</li>
            {/if}
            {#if comparedArrays[0].factors !== comparedArrays[1].factors}
              <li>Factors differ: {comparedArrays[0].factors} vs {comparedArrays[1].factors}</li>
            {/if}
            {#if comparedArrays[0].levels[0] !== comparedArrays[1].levels[0]}
              <li>Levels differ: {comparedArrays[0].levels[0]} vs {comparedArrays[1].levels[0]}</li>
            {/if}
            {#if comparedArrays[0].strength !== comparedArrays[1].strength}
              <li>Strength differs: {comparedArrays[0].strength} vs {comparedArrays[1].strength}</li>
            {/if}
            {#if comparedArrays[0].runs === comparedArrays[1].runs &&
                 comparedArrays[0].factors === comparedArrays[1].factors &&
                 comparedArrays[0].levels[0] === comparedArrays[1].levels[0] &&
                 comparedArrays[0].strength === comparedArrays[1].strength}
              <li class="same">Arrays have identical parameters</li>
            {/if}
          </ul>
        </div>
      {/if}
    </section>
  {/if}
</div>

<style>
  .experiments-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .page-title {
    font-size: var(--text-3xl);
    font-weight: var(--font-bold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-1);
  }

  .page-subtitle {
    font-size: var(--text-lg);
    color: var(--color-text-secondary);
  }

  .header-actions {
    display: flex;
    gap: var(--space-2);
  }

  .error-banner {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-3) var(--space-4);
    background-color: var(--color-error-subtle);
    border-radius: var(--radius-md);
    color: var(--color-error);
  }

  /* Card */
  .card {
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-5);
  }

  .card-title {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-4);
  }

  /* Import Preview */
  .import-details {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .detail-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: var(--space-3);
  }

  .detail {
    padding: var(--space-2);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
  }

  .detail-label {
    display: block;
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    margin-bottom: 2px;
  }

  .detail-value {
    font-weight: var(--font-semibold);
    font-family: var(--font-mono);
    color: var(--color-text-primary);
  }

  .warnings {
    padding: var(--space-3);
    background-color: var(--color-warning-subtle);
    border-radius: var(--radius-md);
  }

  .warnings-title {
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-2);
  }

  .warnings-list {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    padding-left: var(--space-4);
  }

  .data-preview {
    border-top: 1px solid var(--color-border);
    padding-top: var(--space-4);
  }

  .preview-title {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-secondary);
    margin-bottom: var(--space-2);
  }

  .preview-table-wrapper {
    overflow-x: auto;
  }

  .preview-table {
    width: 100%;
    border-collapse: collapse;
    font-family: var(--font-mono);
    font-size: var(--text-sm);
  }

  .preview-table th,
  .preview-table td {
    padding: var(--space-2);
    text-align: center;
    border: 1px solid var(--color-border-light);
  }

  .preview-table th {
    background-color: var(--color-bg-secondary);
    font-weight: var(--font-semibold);
    color: var(--color-text-secondary);
  }

  .preview-table .row-num {
    background-color: var(--color-bg-secondary);
    color: var(--color-text-muted);
  }

  .preview-table .more-rows td {
    background-color: var(--color-bg-secondary);
    color: var(--color-text-muted);
    font-style: italic;
  }

  .import-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-2);
    padding-top: var(--space-4);
    border-top: 1px solid var(--color-border);
  }

  /* Arrays Section */
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--space-4);
  }

  .section-title {
    font-size: var(--text-xl);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .empty-state {
    text-align: center;
    padding: var(--space-8);
    color: var(--color-text-muted);
  }

  .empty-state .hint {
    font-size: var(--text-sm);
    margin-top: var(--space-2);
  }

  .arrays-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: var(--space-4);
  }

  .array-card {
    position: relative;
    padding: var(--space-4);
    background-color: var(--color-bg-elevated);
    border: 2px solid var(--color-border);
    border-radius: var(--radius-lg);
    transition: all var(--duration-fast);
  }

  .array-card:hover {
    border-color: var(--color-accent);
  }

  .array-card.compare-mode {
    cursor: pointer;
  }

  .array-card.selected {
    border-color: var(--color-accent);
    background-color: var(--color-accent-subtle);
  }

  .compare-checkbox {
    position: absolute;
    top: var(--space-3);
    right: var(--space-3);
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    color: var(--color-accent);
    cursor: pointer;
  }

  .card-header {
    margin-bottom: var(--space-2);
  }

  .card-name {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    margin-bottom: 2px;
  }

  .card-notation {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--color-text-muted);
  }

  .card-meta {
    display: flex;
    gap: var(--space-3);
    margin-bottom: var(--space-3);
  }

  .meta-item {
    font-size: var(--text-xs);
    color: var(--color-text-secondary);
  }

  .card-actions {
    display: flex;
    gap: var(--space-1);
    padding-top: var(--space-3);
    border-top: 1px solid var(--color-border-light);
  }

  /* Compare Section */
  .compare-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-4);
  }

  .compare-column {
    padding: var(--space-4);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
  }

  .compare-name {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-accent);
    margin-bottom: var(--space-3);
  }

  .compare-props {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .prop-row {
    display: flex;
    justify-content: space-between;
  }

  .prop-row dt {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
  }

  .prop-row dd {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    font-family: var(--font-mono);
    color: var(--color-text-primary);
  }

  .compare-summary {
    margin-top: var(--space-4);
    padding-top: var(--space-4);
    border-top: 1px solid var(--color-border);
  }

  .compare-summary h4 {
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    color: var(--color-text-secondary);
    margin-bottom: var(--space-2);
  }

  .diff-list {
    font-size: var(--text-sm);
    color: var(--color-text-primary);
    padding-left: var(--space-4);
  }

  .diff-list li {
    margin-bottom: var(--space-1);
  }

  .diff-list .same {
    color: var(--color-success);
  }
</style>
