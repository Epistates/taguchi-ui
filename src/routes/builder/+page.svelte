<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount, onDestroy } from 'svelte';
  import { Button } from '$lib/components/form';
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import { context } from '$lib/stores/app.svelte';
  import type { ConstructionOption, ValidationResult } from '$lib/types';

  // Form state
  let levels = $state(3);
  let factors = $state(4);
  let strength = $state(2);
  let showCustom = $state(false);

  // Validation and suggestions
  let validation = $state<ValidationResult | null>(null);
  let constructions = $state<ConstructionOption[]>([]);
  let selectedConstruction = $derived(constructions[0] ?? null);

  // Debounce timer
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Clean up on leave
  onDestroy(() => {
    context.clear();
    if (debounceTimer) clearTimeout(debounceTimer);
  });

  // Validate and fetch constructions when params change
  async function validateParams() {
    if (debounceTimer) clearTimeout(debounceTimer);

    debounceTimer = setTimeout(async () => {
      // Fetch available constructions
      constructions = await arrayStore.getAvailableConstructions(levels, strength);

      // Filter to those that support our factor count
      constructions = constructions.filter(c => c.maxFactors >= factors);

      // Validate full request
      validation = await arrayStore.validateParams({
        levels,
        factors,
        strength,
      });

      // Update context panel
      context.set('builder', {
        construction: selectedConstruction,
        levels,
        factors,
        strength,
      });
    }, 300);
  }

  // Re-validate when inputs change
  $effect(() => {
    levels; factors; strength;
    validateParams();
  });

  // Update context when construction changes
  $effect(() => {
    if (selectedConstruction) {
      context.set('builder', {
        construction: selectedConstruction,
        levels,
        factors,
        strength,
      });
    }
  });

  // Build the array
  async function handleBuild() {
    if (!validation?.valid || !selectedConstruction) return;

    try {
      const result = await arrayStore.build({
        levels,
        factors,
        strength,
        algorithm: selectedConstruction.name,
      });

      goto(`/viewer/${result.id}`);
    } catch (e) {
      console.error('Build failed:', e);
    }
  }

  // Presets with more info
  const presets = [
    { name: 'L4', levels: 2, factors: 3, strength: 2, runs: 4, desc: 'Smallest 2-level array' },
    { name: 'L8', levels: 2, factors: 7, strength: 2, runs: 8, desc: 'Common screening design' },
    { name: 'L9', levels: 3, factors: 4, strength: 2, runs: 9, desc: 'Common 3-level array' },
    { name: 'L12', levels: 2, factors: 11, strength: 2, runs: 12, desc: 'Plackett-Burman design' },
    { name: 'L16', levels: 2, factors: 15, strength: 2, runs: 16, desc: 'Larger 2-level array' },
    { name: 'L18', levels: 3, factors: 7, strength: 2, runs: 18, desc: 'Mixed 2 and 3 level' },
    { name: 'L25', levels: 5, factors: 6, strength: 2, runs: 25, desc: '5-level design' },
    { name: 'L27', levels: 3, factors: 13, strength: 2, runs: 27, desc: 'Large 3-level array' },
  ];

  function applyPreset(preset: typeof presets[0]) {
    levels = preset.levels;
    factors = preset.factors;
    strength = preset.strength;
    showCustom = false;
  }

  // Check if current config matches a preset
  let activePreset = $derived(
    presets.find(p => p.levels === levels && p.factors === factors && p.strength === strength)
  );

  // Computed notation
  let notation = $derived(
    selectedConstruction
      ? `OA(${selectedConstruction.runs}, ${factors}, ${levels}, ${strength})`
      : `OA(?, ${factors}, ${levels}, ${strength})`
  );

  // Set context on mount
  onMount(() => {
    context.set('builder', {
      construction: null,
      levels,
      factors,
      strength,
    });
  });
</script>

<div class="builder-page">
  <header class="page-header">
    <div class="header-text">
      <h1 class="page-title">Build Array</h1>
      <p class="page-subtitle">Create a custom orthogonal array</p>
    </div>
    {#if selectedConstruction}
      <div class="header-preview">
        <code class="notation">{notation}</code>
        <span class="runs-badge">{selectedConstruction.runs} runs</span>
      </div>
    {/if}
  </header>

  <!-- Presets Grid -->
  <section class="presets-section">
    <h2 class="section-title">Standard Arrays</h2>
    <p class="section-desc">Select a common Taguchi array or customize below</p>

    <div class="presets-grid">
      {#each presets as preset}
        <button
          class="preset-card"
          class:active={activePreset?.name === preset.name}
          onclick={() => applyPreset(preset)}
        >
          <div class="preset-name">{preset.name}</div>
          <div class="preset-stats">
            <span class="preset-runs">{preset.runs} runs</span>
            <span class="preset-factors">{preset.factors} factors</span>
            <span class="preset-levels">{preset.levels} levels</span>
          </div>
          <p class="preset-desc">{preset.desc}</p>
        </button>
      {/each}
    </div>
  </section>

  <!-- Custom Parameters (collapsible) -->
  <section class="custom-section">
    <button class="custom-toggle" onclick={() => showCustom = !showCustom}>
      <span class="toggle-label">Custom Parameters</span>
      <svg
        class="toggle-icon"
        class:open={showCustom}
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <polyline points="6 9 12 15 18 9"/>
      </svg>
    </button>

    {#if showCustom}
      <div class="custom-form">
        <div class="form-row">
          <div class="form-field">
            <label class="field-label" for="levels">Levels</label>
            <input
              id="levels"
              type="number"
              class="field-input"
              bind:value={levels}
              min="2"
              max="100"
            />
            <span class="field-hint">Per factor</span>
          </div>

          <div class="form-field">
            <label class="field-label" for="factors">Factors</label>
            <input
              id="factors"
              type="number"
              class="field-input"
              bind:value={factors}
              min="1"
              max="100"
            />
            <span class="field-hint">Columns</span>
          </div>

          <div class="form-field">
            <label class="field-label" for="strength">Strength</label>
            <input
              id="strength"
              type="number"
              class="field-input"
              bind:value={strength}
              min="1"
              max="6"
            />
            <span class="field-hint">Usually 2</span>
          </div>
        </div>

        {#if validation?.errors.length}
          <div class="validation-errors">
            {#each validation.errors as error}
              <p class="error-text">{error}</p>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </section>

  <!-- Build Action -->
  <section class="build-section">
    {#if validation?.warnings.length}
      <div class="warnings">
        {#each validation.warnings as warning}
          <p class="warning-text">{warning}</p>
        {/each}
      </div>
    {/if}

    {#if constructions.length === 0 && !arrayStore.loading}
      <div class="no-construction">
        <p>No construction available for these parameters.</p>
        <p class="text-muted">Try adjusting levels, factors, or strength.</p>
      </div>
    {:else}
      <Button
        variant="primary"
        size="lg"
        disabled={!validation?.valid || constructions.length === 0 || arrayStore.loading}
        loading={arrayStore.loading}
        onclick={handleBuild}
        fullWidth
      >
        {#snippet children()}
          {#if selectedConstruction}
            Build {notation}
          {:else}
            Build Array
          {/if}
        {/snippet}
      </Button>
    {/if}

    {#if arrayStore.error}
      <p class="build-error">{arrayStore.error}</p>
    {/if}
  </section>
</div>

<style>
  .builder-page {
    max-width: 800px;
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

  /* Section Titles */
  .section-title {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-1);
  }

  .section-desc {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin-bottom: var(--space-4);
  }

  /* Presets Grid */
  .presets-section {
    margin-bottom: var(--space-6);
  }

  .presets-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: var(--space-3);
  }

  .preset-card {
    text-align: left;
    padding: var(--space-4);
    background-color: var(--color-bg-elevated);
    border: 2px solid var(--color-border);
    border-radius: var(--radius-lg);
    cursor: pointer;
    transition: all var(--duration-fast);
  }

  .preset-card:hover {
    border-color: var(--color-accent);
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
  }

  .preset-card.active {
    border-color: var(--color-accent);
    background-color: var(--color-accent-subtle);
  }

  .preset-name {
    font-size: var(--text-xl);
    font-weight: var(--font-bold);
    color: var(--color-accent);
    margin-bottom: var(--space-2);
  }

  .preset-stats {
    display: flex;
    flex-direction: column;
    gap: 2px;
    margin-bottom: var(--space-2);
    font-size: var(--text-xs);
    color: var(--color-text-secondary);
  }

  .preset-desc {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  /* Custom Section */
  .custom-section {
    margin-bottom: var(--space-6);
  }

  .custom-toggle {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: var(--space-3) var(--space-4);
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    cursor: pointer;
    transition: all var(--duration-fast);
  }

  .custom-toggle:hover {
    border-color: var(--color-accent);
  }

  .toggle-label {
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
  }

  .toggle-icon {
    color: var(--color-text-muted);
    transition: transform var(--duration-fast);
  }

  .toggle-icon.open {
    transform: rotate(180deg);
  }

  .custom-form {
    margin-top: var(--space-3);
    padding: var(--space-4);
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .form-row {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-4);
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .field-label {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
  }

  .field-input {
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

  .field-input:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  .field-hint {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    text-align: center;
  }

  .validation-errors {
    margin-top: var(--space-4);
    padding: var(--space-3);
    background-color: var(--color-error-subtle);
    border-radius: var(--radius-md);
  }

  .error-text {
    font-size: var(--text-sm);
    color: var(--color-error);
  }

  /* Build Section */
  .build-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .warnings {
    padding: var(--space-3);
    background-color: var(--color-warning-subtle);
    border-radius: var(--radius-md);
  }

  .warning-text {
    font-size: var(--text-sm);
    color: var(--color-text-primary);
  }

  .no-construction {
    text-align: center;
    padding: var(--space-6);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-lg);
  }

  .no-construction p {
    color: var(--color-text-secondary);
  }

  .no-construction .text-muted {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin-top: var(--space-1);
  }

  .build-error {
    padding: var(--space-3);
    background-color: var(--color-error-subtle);
    border-radius: var(--radius-md);
    font-size: var(--text-sm);
    color: var(--color-error);
    text-align: center;
  }

  /* Responsive */
  @media (max-width: 640px) {
    .page-header {
      flex-direction: column;
      gap: var(--space-3);
    }

    .form-row {
      grid-template-columns: 1fr;
    }

    .presets-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>
