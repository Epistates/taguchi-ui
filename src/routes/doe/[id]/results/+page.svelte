<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { doeStore } from '$lib/stores/doe.svelte';
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import { ResultsGrid } from '$lib/components/doe';
  import type { PageData } from './$types';
  import type { DOEConfig, OAData, ExperimentResults } from '$lib/types';

  let { data }: { data: PageData } = $props();

  // Local state for data
  let config = $state<DOEConfig | null>(null);
  let array = $state<OAData | null>(null);
  let loading = $state(true);

  // Track measurement updates for reactivity
  let measurementVersion = $state(0);

  // Debug state for error display
  let debugInfo = $state<string | null>(null);

  onMount(async () => {
    const foundConfig = doeStore.getConfig(data.configId);

    if (foundConfig) {
      config = foundConfig;
      const foundArray = arrayStore.get(foundConfig.arrayId);

      if (foundArray) {
        array = foundArray;

        // Verify measurements are properly initialized
        // Expected: runs × replicates × responses
        const expectedCount = foundArray.runs * foundConfig.replicates * foundConfig.responses.length;
        const results = doeStore.getResults(data.configId);
        const actualCount = results?.measurements.length ?? 0;

        if (actualCount !== expectedCount) {
          console.log(`Measurements mismatch: expected ${expectedCount}, got ${actualCount}. Re-initializing...`);
          await doeStore.setupResultsForArray(foundConfig.id, foundArray.runs);
          measurementVersion++; // Trigger reactivity
        }
      } else {
        debugInfo = `Array ID "${foundConfig.arrayId}" not found in store.`;
      }
    } else {
      debugInfo = `Config ID "${data.configId}" not found`;
    }
    loading = false;
  });

  // Get results reactively (re-check when measurementVersion changes)
  const results = $derived.by(() => {
    // This dependency forces re-evaluation
    void measurementVersion;
    return doeStore.getResults(data.configId);
  });

  // Progress tracking
  const progress = $derived.by(() => {
    if (!results) return { complete: 0, total: 0, percent: 0 };
    const complete = results.measurements.filter(m => m.value !== null).length;
    const total = results.measurements.length;
    return { complete, total, percent: total > 0 ? (complete / total) * 100 : 0 };
  });

  const canAnalyze = $derived(progress.complete === progress.total && progress.total > 0);

  // Wrapper to trigger reactivity on measurement updates
  function onMeasurementUpdate() {
    measurementVersion++;
  }

  // CSV import state
  let showImport = $state(false);
  let csvInput = $state('');
  let importError = $state<string | null>(null);

  async function handleImportCSV() {
    if (!config || !array) return;
    importError = null;

    // Capture config for use in callback (TypeScript narrowing)
    const currentConfig = config;

    try {
      const lines = csvInput.trim().split('\n').filter(l => l.trim());
      if (lines.length === 0) {
        importError = 'No data found';
        return;
      }

      const responseId = currentConfig.responses[0]?.id;
      if (!responseId) {
        importError = 'No response variable configured';
        return;
      }

      // Parse CSV - expecting: run, rep1, rep2, ... (or just values for single replicate)
      const measurements: Array<{ runIndex: number; replicateIndex: number; responseId: string; value: number | null }> = [];

      lines.forEach((line, lineIdx) => {
        const values = line.split(',').map(v => v.trim());

        // Skip header row if present
        if (lineIdx === 0 && isNaN(parseFloat(values[0]))) {
          return;
        }

        // Determine if first column is run number or value
        const hasRunColumn = values.length > currentConfig.replicates;
        const runIdx = hasRunColumn ? parseInt(values[0]) - 1 : lineIdx;
        const valueStart = hasRunColumn ? 1 : 0;

        for (let repIdx = 0; repIdx < currentConfig.replicates; repIdx++) {
          const rawValue = values[valueStart + repIdx];
          const value = rawValue ? parseFloat(rawValue) : null;
          measurements.push({
            runIndex: runIdx,
            replicateIndex: repIdx,
            responseId,
            value: isNaN(value as number) ? null : value,
          });
        }
      });

      await doeStore.bulkUpdateMeasurements(currentConfig.id, measurements);
      measurementVersion++;
      showImport = false;
      csvInput = '';
    } catch (e) {
      importError = String(e);
    }
  }

  async function handleAnalyze() {
    if (!config || !array) return;

    try {
      await doeStore.runAnalysis(config.id, array.data);
      goto(`/doe/${config.id}/analysis`);
    } catch (e) {
      console.error('Analysis failed:', e);
    }
  }
</script>

<svelte:head>
  <title>Enter Results | {config?.name ?? 'Experiment'} | Taguchi DOE</title>
</svelte:head>

<main class="results-page">
  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading experiment...</p>
    </div>
  {:else if !config}
    <div class="empty-state">
      <h2>Experiment Not Found</h2>
      <p>The experiment configuration could not be found.</p>
      {#if debugInfo}
        <p class="debug-info">{debugInfo}</p>
      {/if}
      <button class="btn btn-primary" onclick={() => goto('/doe')}>
        View Experiments
      </button>
    </div>
  {:else if !array}
    <div class="empty-state">
      <h2>Array Not Found</h2>
      <p>The orthogonal array for this experiment could not be found.</p>
      <p class="debug-info">Array ID: {config.arrayId}</p>
      {#if debugInfo}
        <p class="debug-info">{debugInfo}</p>
      {/if}
      <button class="btn btn-primary" onclick={() => goto('/doe')}>
        View Experiments
      </button>
    </div>
  {:else}
    <header class="page-header">
      <div class="header-content">
        <button class="back-btn" onclick={() => goto('/doe')}>
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M10 3L5 8l5 5"/>
          </svg>
        </button>
        <div>
          <h1>{config.name}</h1>
          <p class="subtitle">
            Enter experimental results for {array.runs} runs
            {#if config.replicates > 1}
              ({config.replicates} replicates each)
            {/if}
          </p>
        </div>
      </div>

      <div class="header-actions">
        <button class="btn btn-secondary" onclick={() => showImport = true}>
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M8 12V3M4 7l4-4 4 4"/>
            <path d="M2 12v2h12v-2"/>
          </svg>
          Import CSV
        </button>
        <button
          class="btn btn-primary"
          disabled={!canAnalyze || doeStore.loading}
          onclick={handleAnalyze}
        >
          {#if doeStore.loading}
            <span class="spinner"></span>
            Analyzing...
          {:else}
            Analyze Results
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M6 3l5 5-5 5"/>
            </svg>
          {/if}
        </button>
      </div>
    </header>

    <!-- Progress Bar -->
    <div class="progress-section">
      <div class="progress-info">
        <span class="progress-label">
          {progress.complete} of {progress.total} measurements entered
        </span>
        {#if canAnalyze}
          <span class="badge badge-success">Ready for Analysis</span>
        {/if}
      </div>
      <div class="progress-bar">
        <div
          class="progress-fill"
          class:complete={canAnalyze}
          style="width: {progress.percent}%"
        ></div>
      </div>
    </div>

    <!-- Response Info -->
    <div class="response-info">
      <span class="response-label">
        Response: <strong>{config.responses[0]?.name ?? 'Value'}</strong>
        {#if config.responses[0]?.unit}
          ({config.responses[0].unit})
        {/if}
      </span>
      <span class="opt-type">
        Goal:
        {#if config.responses[0]?.optimizationType === 'larger-is-better'}
          Maximize
        {:else if config.responses[0]?.optimizationType === 'smaller-is-better'}
          Minimize
        {:else if config.responses[0]?.optimizationType === 'nominal-is-best'}
          Target = {config.responses[0]?.targetValue}
        {/if}
      </span>
    </div>

    <!-- Results Grid -->
    <ResultsGrid {config} {array} onUpdate={onMeasurementUpdate} />

    {#if doeStore.error}
      <div class="error-message">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="8" cy="8" r="6"/>
          <path d="M8 5v3M8 10v1"/>
        </svg>
        {doeStore.error}
        <button class="close-btn" onclick={() => doeStore.clearError()}>
          <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M4 4l8 8M12 4l-8 8"/>
          </svg>
        </button>
      </div>
    {/if}

    <!-- Import Modal -->
    {#if showImport}
      <div class="modal-backdrop" onclick={() => showImport = false}>
        <div class="modal" onclick={(e) => e.stopPropagation()}>
          <div class="modal-header">
            <h2>Import CSV Data</h2>
            <button class="close-btn" onclick={() => showImport = false}>
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M4 4l8 8M12 4l-8 8"/>
              </svg>
            </button>
          </div>

          <div class="modal-body">
            <p class="import-hint">
              Paste CSV data with one row per run. Format:
              {#if config.replicates > 1}
                <code>run, rep1, rep2, ...</code> or just <code>rep1, rep2, ...</code>
              {:else}
                <code>run, value</code> or just <code>value</code>
              {/if}
            </p>

            <textarea
              class="csv-input"
              placeholder={config.replicates > 1
                ? "1, 85.2, 84.1, 86.0\n2, 91.5, 92.0, 90.8\n..."
                : "1, 85.2\n2, 91.5\n..."}
              bind:value={csvInput}
              rows="10"
            ></textarea>

            {#if importError}
              <p class="import-error">{importError}</p>
            {/if}
          </div>

          <div class="modal-footer">
            <button class="btn btn-ghost" onclick={() => showImport = false}>
              Cancel
            </button>
            <button
              class="btn btn-primary"
              disabled={!csvInput.trim()}
              onclick={handleImportCSV}
            >
              Import
            </button>
          </div>
        </div>
      </div>
    {/if}
  {/if}
</main>

<style>
  .results-page {
    max-width: 1200px;
    margin: 0 auto;
    padding: var(--space-6);
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: var(--space-6);
  }

  .header-content {
    display: flex;
    align-items: flex-start;
    gap: var(--space-3);
  }

  .back-btn {
    padding: var(--space-2);
    border-radius: var(--radius-md);
    color: var(--color-text-muted);
    transition: all var(--duration-fast);
  }

  .back-btn:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .header-content h1 {
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-1);
  }

  .subtitle {
    color: var(--color-text-secondary);
    font-size: var(--text-base);
  }

  .header-actions {
    display: flex;
    gap: var(--space-3);
  }

  .progress-section {
    margin-bottom: var(--space-4);
  }

  .progress-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--space-2);
  }

  .progress-label {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
  }

  .progress-bar {
    height: 8px;
    background-color: var(--color-bg-tertiary);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background-color: var(--color-accent);
    transition: width var(--duration-normal);
  }

  .progress-fill.complete {
    background-color: var(--color-success);
  }

  .response-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-3) var(--space-4);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
    margin-bottom: var(--space-4);
    font-size: var(--text-sm);
  }

  .response-label {
    color: var(--color-text-secondary);
  }

  .response-label strong {
    color: var(--color-text-primary);
  }

  .opt-type {
    color: var(--color-text-muted);
  }

  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-12);
    gap: var(--space-4);
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .empty-state {
    text-align: center;
    padding: var(--space-12);
  }

  .empty-state h2 {
    font-size: var(--text-xl);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-2);
  }

  .empty-state p {
    color: var(--color-text-secondary);
    margin-bottom: var(--space-6);
  }

  .debug-info {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    background-color: var(--color-bg-tertiary);
    padding: var(--space-2);
    border-radius: var(--radius-sm);
    word-break: break-all;
  }

  .error-message {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-3) var(--space-4);
    margin-top: var(--space-4);
    background-color: var(--color-error-subtle);
    border: 1px solid var(--color-error);
    border-radius: var(--radius-md);
    color: var(--color-error);
    font-size: var(--text-sm);
  }

  .error-message .close-btn {
    margin-left: auto;
    padding: var(--space-1);
    border-radius: var(--radius-sm);
  }

  .error-message .close-btn:hover {
    background-color: rgba(0, 0, 0, 0.1);
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid currentColor;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  /* Modal */
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal {
    background-color: var(--color-bg-elevated);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-xl);
    width: 100%;
    max-width: 500px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-4);
    border-bottom: 1px solid var(--color-border);
  }

  .modal-header h2 {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
  }

  .modal-body {
    padding: var(--space-4);
    overflow-y: auto;
  }

  .import-hint {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    margin-bottom: var(--space-3);
  }

  .import-hint code {
    background-color: var(--color-bg-tertiary);
    padding: var(--space-1);
    border-radius: var(--radius-sm);
    font-family: var(--font-mono);
    font-size: var(--text-xs);
  }

  .csv-input {
    width: 100%;
    padding: var(--space-3);
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-bg-primary);
    color: var(--color-text-primary);
    resize: vertical;
  }

  .csv-input:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: var(--shadow-focus);
  }

  .import-error {
    margin-top: var(--space-2);
    color: var(--color-error);
    font-size: var(--text-sm);
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    padding: var(--space-4);
    border-top: 1px solid var(--color-border);
  }

  .close-btn {
    padding: var(--space-1);
    border-radius: var(--radius-sm);
    color: var(--color-text-muted);
    transition: all var(--duration-fast);
  }

  .close-btn:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }
</style>
