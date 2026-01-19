<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import { doeStore } from '$lib/stores/doe.svelte';
  import { FactorConfigTable, ResponseConfigForm } from '$lib/components/doe';
  import type { ExperimentFactor, ResponseVariable, OAData, AnalysisSettings } from '$lib/types';
  import { DEFAULT_ANALYSIS_SETTINGS } from '$lib/types';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  // Local array state (populated on mount)
  let array = $state<OAData | null>(null);
  let loading = $state(true);

  // Load array on mount
  onMount(() => {
    const found = arrayStore.get(data.arrayId);
    if (found) {
      array = found;
    }
    loading = false;
  });

  // Check if design is saturated (no error degrees of freedom)
  const designInfo = $derived.by(() => {
    if (!array) return { isSaturated: false, errorDf: 0, minReplicates: 1 };

    const totalDf = array.runs - 1;
    // Sum of (levels - 1) for each factor
    const factorDf = array.levels.reduce((sum, levels) => sum + (levels - 1), 0);
    const errorDf = totalDf - factorDf;

    // If error_df <= 0, design is saturated and needs replicates
    const isSaturated = errorDf <= 0;
    // Minimum replicates needed: at least 2 for error estimation in saturated designs
    const minReplicates = isSaturated ? 2 : 1;

    return { isSaturated, errorDf, minReplicates };
  });

  // Initialize draft when array is available, with correct minimum replicates
  $effect(() => {
    if (array && !doeStore.draftConfig) {
      doeStore.startDraft(array.id, array.factors, array.levels);
      // Set minimum replicates for saturated designs
      if (designInfo.minReplicates > 1) {
        doeStore.updateDraft({ replicates: designInfo.minReplicates });
      }
    }
  });

  const draft = $derived(doeStore.draftConfig);

  // Validation
  const isValid = $derived(() => {
    if (!draft) return false;
    if (!draft.name?.trim()) return false;
    if (!draft.factors?.every(f => f.name.trim())) return false;
    if (!draft.responses?.length) return false;
    if (!draft.responses?.every(r => r.name.trim())) return false;
    if (draft.responses?.some(r =>
      r.optimizationType === 'nominal-is-best' && r.targetValue === undefined
    )) return false;
    return true;
  });

  function handleFactorUpdate(factorId: string, updates: Partial<ExperimentFactor>) {
    doeStore.updateDraftFactor(factorId, updates);
  }

  function handleResponseUpdate(updates: Partial<ResponseVariable>) {
    if (draft?.responses?.[0]) {
      doeStore.updateDraftResponse(draft.responses[0].id, updates);
    }
  }

  function handleReplicatesChange(value: number) {
    doeStore.updateDraft({ replicates: Math.max(1, Math.min(10, value)) });
  }

  function handleNameChange(value: string) {
    doeStore.updateDraft({ name: value });
  }

  function handleAnalysisSettingsChange(updates: Partial<AnalysisSettings>) {
    const current = draft?.analysisSettings ?? DEFAULT_ANALYSIS_SETTINGS;
    doeStore.updateDraft({
      analysisSettings: { ...current, ...updates }
    });
  }

  // State for collapsible advanced settings
  let showAdvanced = $state(false);

  async function handleCreate() {
    const config = await doeStore.finalizeDraft();
    if (config && array) {
      // Set up results with correct number of runs
      await doeStore.setupResultsForArray(config.id, array.runs);
      // Navigate to results entry
      await goto(`/doe/${config.id}/results`);
    }
  }

  function handleCancel() {
    doeStore.cancelDraft();
    goto(`/viewer/${data.arrayId}`);
  }
</script>

<svelte:head>
  <title>Configure Experiment | Taguchi DOE</title>
</svelte:head>

<main class="config-page">
  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading array...</p>
    </div>
  {:else if !array}
    <div class="empty-state">
      <h2>Array Not Found</h2>
      <p>The selected array could not be found. Please select an array first.</p>
      <p class="debug-info">Array ID: {data.arrayId}</p>
      <button class="btn btn-primary" onclick={() => goto('/catalogue')}>
        Browse Catalogue
      </button>
    </div>
  {:else if !draft}
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading configuration...</p>
    </div>
  {:else}
    <header class="page-header">
      <div class="header-content">
        <h1>Configure Experiment</h1>
        <p class="subtitle">
          Set up your DOE experiment using {array.metadata.name || `${array.runs}-run array`}
        </p>
      </div>
      <div class="array-badge">
        <span class="badge badge-info">
          {array.runs} runs &times; {array.factors} factors
        </span>
      </div>
    </header>

    <div class="config-sections">
      <!-- Experiment Name -->
      <section class="config-section">
        <h2 class="section-title">Experiment Name</h2>
        <input
          type="text"
          class="form-input name-input"
          value={draft.name ?? ''}
          placeholder="Enter experiment name"
          oninput={(e) => handleNameChange(e.currentTarget.value)}
        />
      </section>

      <!-- Factor Configuration -->
      <section class="config-section">
        <h2 class="section-title">Factor Configuration</h2>
        <p class="section-desc">
          Define names and actual level values for each factor in your experiment.
        </p>
        {#if draft.factors}
          <FactorConfigTable
            factors={draft.factors}
            numLevels={array.levels}
            onUpdate={handleFactorUpdate}
          />
        {/if}
      </section>

      <!-- Response Variable -->
      <section class="config-section">
        <h2 class="section-title">Response Variable</h2>
        <p class="section-desc">
          Define the output you will measure and how it should be optimized.
        </p>
        {#if draft.responses?.[0]}
          <ResponseConfigForm
            response={draft.responses[0]}
            onUpdate={handleResponseUpdate}
          />
        {/if}
      </section>

      <!-- Replicates -->
      <section class="config-section">
        <h2 class="section-title">Replicates</h2>
        <p class="section-desc">
          Number of times to repeat each experimental run. More replicates improve statistical confidence.
        </p>
        <div class="replicates-control">
          <button
            class="btn btn-secondary btn-icon"
            onclick={() => handleReplicatesChange((draft.replicates ?? 1) - 1)}
            disabled={(draft.replicates ?? 1) <= designInfo.minReplicates}
            aria-label="Decrease replicates"
          >
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M3 8h10"/>
            </svg>
          </button>
          <span class="replicates-value">{draft.replicates ?? 1}</span>
          <button
            class="btn btn-secondary btn-icon"
            onclick={() => handleReplicatesChange((draft.replicates ?? 1) + 1)}
            disabled={(draft.replicates ?? 1) >= 10}
            aria-label="Increase replicates"
          >
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M8 3v10M3 8h10"/>
            </svg>
          </button>
          <span class="replicates-label">
            {(draft.replicates ?? 1) === 1 ? 'replicate' : 'replicates'}
            ({array.runs * (draft.replicates ?? 1)} total measurements)
          </span>
        </div>
        {#if designInfo.isSaturated}
          <p class="info-note">
            Saturated design — minimum {designInfo.minReplicates} replicates required for error estimation.
          </p>
        {/if}
      </section>

      <!-- Advanced Analysis Settings -->
      <section class="config-section">
        <button
          class="section-toggle"
          onclick={() => showAdvanced = !showAdvanced}
          aria-expanded={showAdvanced}
        >
          <h2 class="section-title">Advanced Analysis Settings</h2>
          <svg
            class="toggle-icon"
            class:rotated={showAdvanced}
            width="20"
            height="20"
            viewBox="0 0 20 20"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path d="M6 8l4 4 4-4"/>
          </svg>
        </button>

        {#if showAdvanced}
          {@const settings = draft?.analysisSettings ?? DEFAULT_ANALYSIS_SETTINGS}
          <div class="advanced-settings">
            <!-- Confidence Level -->
            <div class="setting-row">
              <div class="setting-info">
                <label for="confidence-level" class="setting-label">Confidence Level</label>
                <span class="setting-hint">Statistical confidence for intervals (higher = wider intervals)</span>
              </div>
              <select
                id="confidence-level"
                class="setting-select"
                value={settings.confidenceLevel}
                onchange={(e) => handleAnalysisSettingsChange({ confidenceLevel: parseFloat(e.currentTarget.value) })}
              >
                <option value={0.80}>80%</option>
                <option value={0.90}>90%</option>
                <option value={0.95}>95%</option>
                <option value={0.99}>99%</option>
              </select>
            </div>

            <!-- Enable Pooling -->
            <div class="setting-row">
              <div class="setting-info">
                <span class="setting-label">Factor Pooling</span>
                <span class="setting-hint">Pool insignificant factors into error term for better estimates</span>
              </div>
              <label class="toggle-switch">
                <input
                  type="checkbox"
                  checked={settings.enablePooling}
                  onchange={(e) => handleAnalysisSettingsChange({ enablePooling: e.currentTarget.checked })}
                />
                <span class="toggle-slider"></span>
              </label>
            </div>

            {#if settings.enablePooling}
              <!-- Pooling Threshold -->
              <div class="setting-row indented">
                <div class="setting-info">
                  <label for="pooling-threshold" class="setting-label">Pooling Threshold</label>
                  <span class="setting-hint">F-ratio below which factors are pooled (typical: 2.0)</span>
                </div>
                <input
                  id="pooling-threshold"
                  type="number"
                  class="setting-input"
                  min="0.5"
                  max="10"
                  step="0.5"
                  value={settings.poolingThreshold}
                  oninput={(e) => handleAnalysisSettingsChange({ poolingThreshold: parseFloat(e.currentTarget.value) || 2.0 })}
                />
              </div>

              <!-- Min Unpooled Factors -->
              <div class="setting-row indented">
                <div class="setting-info">
                  <label for="min-unpooled" class="setting-label">Minimum Unpooled Factors</label>
                  <span class="setting-hint">Always keep at least this many factors unpooled</span>
                </div>
                <input
                  id="min-unpooled"
                  type="number"
                  class="setting-input"
                  min="0"
                  max={array?.factors ?? 10}
                  step="1"
                  value={settings.minUnpooledFactors}
                  oninput={(e) => handleAnalysisSettingsChange({ minUnpooledFactors: parseInt(e.currentTarget.value) || 1 })}
                />
              </div>
            {/if}
          </div>
        {/if}
      </section>
    </div>

    <!-- Actions -->
    <footer class="page-actions">
      <button class="btn btn-ghost" onclick={handleCancel}>
        Cancel
      </button>
      <button
        class="btn btn-primary"
        disabled={!isValid()}
        onclick={handleCreate}
      >
        Continue to Results Entry
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M6 3l5 5-5 5"/>
        </svg>
      </button>
    </footer>
  {/if}
</main>

<style>
  .config-page {
    max-width: 900px;
    margin: 0 auto;
    padding: var(--space-6);
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: var(--space-8);
    padding-bottom: var(--space-6);
    border-bottom: 1px solid var(--color-border);
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

  .config-sections {
    display: flex;
    flex-direction: column;
    gap: var(--space-8);
  }

  .config-section {
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-6);
  }

  .section-title {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-2);
  }

  .section-desc {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin-bottom: var(--space-4);
  }

  .name-input {
    max-width: 400px;
    padding: var(--space-2) var(--space-3);
    font-size: var(--text-base);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-bg-primary);
    color: var(--color-text-primary);
  }

  .name-input:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: var(--shadow-focus);
  }

  .replicates-control {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .replicates-value {
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
    color: var(--color-text-primary);
    min-width: 40px;
    text-align: center;
    font-family: var(--font-mono);
  }

  .replicates-label {
    color: var(--color-text-secondary);
    font-size: var(--text-sm);
  }

  .info-note {
    margin-top: var(--space-2);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .page-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    margin-top: var(--space-8);
    padding-top: var(--space-6);
    border-top: 1px solid var(--color-border);
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

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  /* Advanced Settings Section */
  .section-toggle {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 0;
    background: none;
    border: none;
    cursor: pointer;
    color: inherit;
  }

  .section-toggle .section-title {
    margin: 0;
  }

  .toggle-icon {
    color: var(--color-text-muted);
    transition: transform 0.2s ease;
  }

  .toggle-icon.rotated {
    transform: rotate(180deg);
  }

  .advanced-settings {
    margin-top: var(--space-4);
    padding-top: var(--space-4);
    border-top: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
  }

  .setting-row.indented {
    padding-left: var(--space-6);
    opacity: 0.9;
  }

  .setting-info {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    flex: 1;
  }

  .setting-label {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
  }

  .setting-hint {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .setting-select {
    padding: var(--space-2) var(--space-3);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-bg-primary);
    color: var(--color-text-primary);
    font-size: var(--text-sm);
    min-width: 100px;
  }

  .setting-select:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  .setting-input {
    width: 80px;
    padding: var(--space-2) var(--space-2);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-bg-primary);
    color: var(--color-text-primary);
    font-size: var(--text-sm);
    text-align: center;
    font-family: var(--font-mono);
  }

  .setting-input:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  /* Toggle Switch */
  .toggle-switch {
    position: relative;
    width: 44px;
    height: 24px;
    flex-shrink: 0;
  }

  .toggle-switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .toggle-slider {
    position: absolute;
    cursor: pointer;
    inset: 0;
    background-color: var(--color-bg-tertiary);
    border-radius: 24px;
    transition: 0.2s;
  }

  .toggle-slider::before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    border-radius: 50%;
    transition: 0.2s;
  }

  .toggle-switch input:checked + .toggle-slider {
    background-color: var(--color-accent);
  }

  .toggle-switch input:checked + .toggle-slider::before {
    transform: translateX(20px);
  }

  .toggle-switch input:focus + .toggle-slider {
    box-shadow: var(--shadow-focus);
  }
</style>
