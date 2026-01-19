<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount, onDestroy } from 'svelte';
  import { Button } from '$lib/components/form';
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import { context } from '$lib/stores/app.svelte';
  import { isPrimePower, isMultipleOf4, isOddPrime, formatPrimePower } from '$lib/utils';

  // Algorithm definitions
  const algorithms = [
    {
      id: 'bose',
      name: 'Bose',
      fullName: 'Bose Construction',
      formula: 'OA(s², s+1, s, 2)',
      tagline: 'Prime power levels, strength 2',
    },
    {
      id: 'bush',
      name: 'Bush',
      fullName: 'Bush Construction',
      formula: 'OA(sᵗ, t+1, s, t)',
      tagline: 'Prime power levels, variable strength',
    },
    {
      id: 'hadamard',
      name: 'Hadamard',
      fullName: 'Hadamard Construction',
      formula: 'OA(4n, 4n-1, 2, 2)',
      tagline: '2 levels only, many factors',
    },
    {
      id: 'addelman_kempthorne',
      name: 'Addelman-Kempthorne',
      fullName: 'Addelman-Kempthorne',
      formula: 'OA(2s², mixed, mixed, 2)',
      tagline: 'Mixed 2-level and s-level factors',
    },
  ];

  // State
  let selectedAlgorithm = $state(algorithms[0].id);
  let params = $state<Record<string, number>>({
    levels: 3,
    strength: 2,
    order: 8,
    twoLevelFactors: 3,
    sLevelFactors: 4,
    sLevels: 3,
  });
  let building = $state(false);
  let error = $state<string | null>(null);

  // Get current algorithm
  let currentAlgo = $derived(algorithms.find(a => a.id === selectedAlgorithm)!);

  // Validation state
  let validation = $derived((() => {
    const errors: string[] = [];
    const warnings: string[] = [];

    switch (selectedAlgorithm) {
      case 'bose': {
        const pp = isPrimePower(params.levels);
        if (!pp.valid) {
          errors.push(`${params.levels} is not a prime power`);
        }
        break;
      }
      case 'bush': {
        const pp = isPrimePower(params.levels);
        if (!pp.valid) {
          errors.push(`${params.levels} is not a prime power`);
        }
        if (params.strength < 2) {
          errors.push('Strength must be at least 2');
        }
        if (params.strength > 6) {
          warnings.push('High strength may result in very large arrays');
        }
        break;
      }
      case 'hadamard': {
        if (!isMultipleOf4(params.order)) {
          errors.push(`${params.order} is not a multiple of 4`);
        }
        if (params.order > 128) {
          warnings.push('Large order may be slow to construct');
        }
        break;
      }
      case 'addelman_kempthorne': {
        if (!isOddPrime(params.sLevels)) {
          errors.push(`${params.sLevels} is not an odd prime`);
        }
        if (params.twoLevelFactors + params.sLevelFactors < 2) {
          errors.push('Need at least 2 total factors');
        }
        break;
      }
    }

    return { valid: errors.length === 0, errors, warnings };
  })());

  // Preview computed array properties
  let preview = $derived((() => {
    try {
      switch (selectedAlgorithm) {
        case 'bose':
          return {
            runs: params.levels ** 2,
            factors: params.levels + 1,
            levels: params.levels,
            strength: 2,
          };
        case 'bush':
          return {
            runs: params.levels ** params.strength,
            factors: params.strength + 1,
            levels: params.levels,
            strength: params.strength,
          };
        case 'hadamard':
          return {
            runs: params.order,
            factors: params.order - 1,
            levels: 2,
            strength: 2,
          };
        case 'addelman_kempthorne':
          return {
            runs: 2 * params.sLevels ** 2,
            factors: params.twoLevelFactors + params.sLevelFactors,
            levels: 'Mixed',
            strength: 2,
          };
        default:
          return null;
      }
    } catch {
      return null;
    }
  })());

  // Notation string
  let notation = $derived(
    preview
      ? `OA(${preview.runs}, ${preview.factors}, ${preview.levels}, ${preview.strength})`
      : null
  );

  // Update context panel when algorithm changes
  $effect(() => {
    context.set('expert', { algorithm: selectedAlgorithm });
  });

  // Clean up on leave
  onDestroy(() => {
    context.clear();
  });

  // Set context on mount
  onMount(() => {
    context.set('expert', { algorithm: selectedAlgorithm });
  });

  // Build request
  function buildRequest() {
    switch (selectedAlgorithm) {
      case 'bose':
        return {
          levels: params.levels,
          factors: params.levels + 1,
          strength: 2,
          algorithm: 'Bose',
        };
      case 'bush':
        return {
          levels: params.levels,
          factors: params.strength + 1,
          strength: params.strength,
          algorithm: 'Bush',
        };
      case 'hadamard':
        return {
          levels: 2,
          factors: params.order - 1,
          strength: 2,
          algorithm: 'Hadamard',
        };
      case 'addelman_kempthorne':
        return {
          levels: [
            ...Array(params.twoLevelFactors).fill(2),
            ...Array(params.sLevelFactors).fill(params.sLevels),
          ],
          factors: params.twoLevelFactors + params.sLevelFactors,
          strength: 2,
          algorithm: 'Addelman-Kempthorne',
        };
      default:
        throw new Error('Unknown algorithm');
    }
  }

  // Build handler
  async function handleBuild() {
    if (!validation.valid) return;

    building = true;
    error = null;

    try {
      const request = buildRequest();
      const result = await arrayStore.build(request);
      goto(`/viewer/${result.id}`);
    } catch (e) {
      error = String(e);
    } finally {
      building = false;
    }
  }
</script>

<div class="expert-page">
  <header class="page-header">
    <div class="header-text">
      <h1 class="page-title">Expert Mode</h1>
      <p class="page-subtitle">Direct access to construction algorithms</p>
    </div>
    {#if preview && validation.valid}
      <div class="header-preview">
        <code class="notation">{notation}</code>
        <span class="runs-badge">{preview.runs} runs</span>
      </div>
    {/if}
  </header>

  <!-- Algorithm Tabs -->
  <nav class="algorithm-tabs">
    {#each algorithms as algo}
      <button
        class="algo-tab"
        class:active={selectedAlgorithm === algo.id}
        onclick={() => selectedAlgorithm = algo.id}
      >
        <span class="tab-name">{algo.name}</span>
        <span class="tab-tagline">{algo.tagline}</span>
      </button>
    {/each}
  </nav>

  <!-- Selected Algorithm Panel -->
  <section class="algorithm-panel">
    <div class="panel-header">
      <h2 class="algo-name">{currentAlgo.fullName}</h2>
      <code class="algo-formula">{currentAlgo.formula}</code>
    </div>

    <div class="panel-content">
      <!-- Parameters -->
      <div class="params-section">
        <h3 class="section-label">Parameters</h3>

        {#if selectedAlgorithm === 'bose'}
          <div class="param-row">
            <label class="param-label">
              <span class="param-name">Levels (s)</span>
              <span class="param-hint">Must be a prime power</span>
            </label>
            <div class="param-input-group">
              <input
                type="number"
                class="param-input"
                bind:value={params.levels}
                min={2}
                max={31}
              />
              <span class="param-validation" class:valid={isPrimePower(params.levels).valid} class:invalid={!isPrimePower(params.levels).valid}>
                {isPrimePower(params.levels).valid ? formatPrimePower(params.levels) : 'Not a prime power'}
              </span>
            </div>
          </div>
        {/if}

        {#if selectedAlgorithm === 'bush'}
          <div class="param-row">
            <label class="param-label">
              <span class="param-name">Levels (s)</span>
              <span class="param-hint">Must be a prime power</span>
            </label>
            <div class="param-input-group">
              <input
                type="number"
                class="param-input"
                bind:value={params.levels}
                min={2}
                max={31}
              />
              <span class="param-validation" class:valid={isPrimePower(params.levels).valid} class:invalid={!isPrimePower(params.levels).valid}>
                {isPrimePower(params.levels).valid ? formatPrimePower(params.levels) : 'Not a prime power'}
              </span>
            </div>
          </div>
          <div class="param-row">
            <label class="param-label">
              <span class="param-name">Strength (t)</span>
              <span class="param-hint">Orthogonality degree (usually 2-4)</span>
            </label>
            <div class="param-input-group">
              <input
                type="number"
                class="param-input"
                bind:value={params.strength}
                min={2}
                max={6}
              />
              <span class="param-validation valid">
                t = {params.strength} → {params.strength + 1} factors
              </span>
            </div>
          </div>
        {/if}

        {#if selectedAlgorithm === 'hadamard'}
          <div class="param-row">
            <label class="param-label">
              <span class="param-name">Hadamard Order</span>
              <span class="param-hint">Must be a multiple of 4</span>
            </label>
            <div class="param-input-group">
              <input
                type="number"
                class="param-input"
                bind:value={params.order}
                min={4}
                max={256}
                step={4}
              />
              <span class="param-validation" class:valid={isMultipleOf4(params.order)} class:invalid={!isMultipleOf4(params.order)}>
                {isMultipleOf4(params.order) ? `4 × ${params.order / 4} = ${params.order}` : 'Not a multiple of 4'}
              </span>
            </div>
          </div>
        {/if}

        {#if selectedAlgorithm === 'addelman_kempthorne'}
          <div class="param-row">
            <label class="param-label">
              <span class="param-name">2-Level Factors</span>
              <span class="param-hint">Binary factors (0/1)</span>
            </label>
            <div class="param-input-group">
              <input
                type="number"
                class="param-input"
                bind:value={params.twoLevelFactors}
                min={0}
                max={20}
              />
            </div>
          </div>
          <div class="param-row">
            <label class="param-label">
              <span class="param-name">s-Level Factors</span>
              <span class="param-hint">Factors with s levels</span>
            </label>
            <div class="param-input-group">
              <input
                type="number"
                class="param-input"
                bind:value={params.sLevelFactors}
                min={1}
                max={20}
              />
            </div>
          </div>
          <div class="param-row">
            <label class="param-label">
              <span class="param-name">s Value</span>
              <span class="param-hint">Must be an odd prime (3, 5, 7, ...)</span>
            </label>
            <div class="param-input-group">
              <input
                type="number"
                class="param-input"
                bind:value={params.sLevels}
                min={3}
                max={31}
                step={2}
              />
              <span class="param-validation" class:valid={isOddPrime(params.sLevels)} class:invalid={!isOddPrime(params.sLevels)}>
                {isOddPrime(params.sLevels) ? `${params.sLevels} is odd prime` : 'Not an odd prime'}
              </span>
            </div>
          </div>
        {/if}
      </div>

      <!-- Output Preview -->
      <div class="preview-section">
        <h3 class="section-label">Output</h3>

        {#if preview}
          <div class="preview-grid">
            <div class="preview-stat">
              <span class="stat-value">{preview.runs}</span>
              <span class="stat-label">Runs (N)</span>
            </div>
            <div class="preview-stat">
              <span class="stat-value">{preview.factors}</span>
              <span class="stat-label">Factors (k)</span>
            </div>
            <div class="preview-stat">
              <span class="stat-value">{preview.levels}</span>
              <span class="stat-label">Levels (s)</span>
            </div>
            <div class="preview-stat">
              <span class="stat-value">{preview.strength}</span>
              <span class="stat-label">Strength (t)</span>
            </div>
          </div>
        {/if}
      </div>
    </div>

    <!-- Validation Messages -->
    {#if validation.errors.length > 0}
      <div class="validation-errors">
        {#each validation.errors as err}
          <p class="error-text">{err}</p>
        {/each}
      </div>
    {/if}

    {#if validation.warnings.length > 0}
      <div class="validation-warnings">
        {#each validation.warnings as warn}
          <p class="warning-text">{warn}</p>
        {/each}
      </div>
    {/if}

    {#if error}
      <div class="build-error">
        <p>{error}</p>
      </div>
    {/if}
  </section>

  <!-- Build Action -->
  <section class="build-section">
    <Button
      variant="primary"
      size="lg"
      fullWidth
      disabled={!validation.valid || building}
      loading={building}
      onclick={handleBuild}
    >
      {#snippet children()}
        {#if notation && validation.valid}
          Build {notation}
        {:else}
          Build Array
        {/if}
      {/snippet}
    </Button>
  </section>
</div>

<style>
  .expert-page {
    max-width: 900px;
    margin: 0 auto;
  }

  /* Header */
  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: var(--space-6);
  }

  .page-title {
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-1);
  }

  .page-subtitle {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
  }

  .header-preview {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .notation {
    font-family: var(--font-mono);
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-accent);
  }

  .runs-badge {
    padding: var(--space-1) var(--space-3);
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    background-color: var(--color-accent-subtle);
    color: var(--color-accent);
    border-radius: var(--radius-full);
  }

  /* Algorithm Tabs */
  .algorithm-tabs {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: var(--space-2);
    margin-bottom: var(--space-4);
  }

  .algo-tab {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: var(--space-3) var(--space-4);
    background-color: var(--color-bg-elevated);
    border: 2px solid var(--color-border);
    border-radius: var(--radius-lg);
    cursor: pointer;
    transition: all var(--duration-fast);
    text-align: left;
  }

  .algo-tab:hover {
    border-color: var(--color-accent);
  }

  .algo-tab.active {
    border-color: var(--color-accent);
    background-color: var(--color-accent-subtle);
  }

  .tab-name {
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    font-size: var(--text-sm);
  }

  .tab-tagline {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .algo-tab.active .tab-name {
    color: var(--color-accent);
  }

  /* Algorithm Panel */
  .algorithm-panel {
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-5);
    margin-bottom: var(--space-4);
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-5);
    padding-bottom: var(--space-4);
    border-bottom: 1px solid var(--color-border);
  }

  .algo-name {
    font-size: var(--text-xl);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .algo-formula {
    font-family: var(--font-mono);
    font-size: var(--text-lg);
    color: var(--color-accent);
    padding: var(--space-2) var(--space-3);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
  }

  .panel-content {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-6);
  }

  .section-label {
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: var(--space-4);
  }

  /* Parameters */
  .params-section {
    display: flex;
    flex-direction: column;
  }

  .param-row {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    margin-bottom: var(--space-4);
  }

  .param-row:last-child {
    margin-bottom: 0;
  }

  .param-label {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .param-name {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
  }

  .param-hint {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .param-input-group {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .param-input {
    width: 80px;
    height: 40px;
    padding: 0 var(--space-3);
    font-size: var(--text-base);
    font-family: var(--font-mono);
    background-color: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    color: var(--color-text-primary);
    text-align: center;
  }

  .param-input:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  .param-validation {
    font-size: var(--text-xs);
    font-family: var(--font-mono);
  }

  .param-validation.valid {
    color: var(--color-success);
  }

  .param-validation.invalid {
    color: var(--color-error);
  }

  /* Preview */
  .preview-section {
    display: flex;
    flex-direction: column;
  }

  .preview-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-3);
  }

  .preview-stat {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: var(--space-3);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
  }

  .stat-value {
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
    font-family: var(--font-mono);
    color: var(--color-text-primary);
  }

  .stat-label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  /* Validation Messages */
  .validation-errors,
  .validation-warnings,
  .build-error {
    margin-top: var(--space-4);
    padding: var(--space-3);
    border-radius: var(--radius-md);
  }

  .validation-errors {
    background-color: var(--color-error-subtle);
  }

  .error-text {
    font-size: var(--text-sm);
    color: var(--color-error);
  }

  .validation-warnings {
    background-color: var(--color-warning-subtle);
  }

  .warning-text {
    font-size: var(--text-sm);
    color: var(--color-text-primary);
  }

  .build-error {
    background-color: var(--color-error-subtle);
  }

  .build-error p {
    font-size: var(--text-sm);
    color: var(--color-error);
  }

  /* Build Section */
  .build-section {
    /* Matches the padding of algorithm panel for alignment */
  }

  /* Responsive */
  @media (max-width: 768px) {
    .algorithm-tabs {
      grid-template-columns: repeat(2, 1fr);
    }

    .page-header {
      flex-direction: column;
      gap: var(--space-3);
    }

    .panel-header {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--space-2);
    }

    .panel-content {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 480px) {
    .algorithm-tabs {
      grid-template-columns: 1fr;
    }
  }
</style>
