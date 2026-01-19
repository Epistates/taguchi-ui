<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount, onDestroy } from 'svelte';
  import { Button } from '$lib/components/form';
  import { catalogueStore } from '$lib/stores/catalogue.svelte';
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import { context } from '$lib/stores/app.svelte';
  import type { StandardArrayInfo } from '$lib/types';

  // Selected array for preview
  let selectedArray = $state<StandardArrayInfo | null>(null);

  // Clean up context when leaving the page
  onDestroy(() => {
    context.clear();
  });

  // Filter state
  let filterLevels = $state<string>('');
  let filterMinRuns = $state<string>('');
  let filterMaxRuns = $state<string>('');

  // Check if any filters are active
  let hasActiveFilters = $derived(filterLevels !== '' || filterMinRuns !== '' || filterMaxRuns !== '');

  // Load catalogue on mount
  onMount(async () => {
    await catalogueStore.load();

    // Check for ?select= parameter
    const selectParam = $page.url.searchParams.get('select');
    if (selectParam) {
      const found = catalogueStore.standardArrays.find(a => a.name === selectParam);
      if (found) {
        selectedArray = found;
        context.set('catalogue-preview', found);
      }
    }
  });

  // Apply filters
  function applyFilters() {
    catalogueStore.setFilter({
      levels: filterLevels ? parseInt(filterLevels) : undefined,
      minRuns: filterMinRuns ? parseInt(filterMinRuns) : undefined,
      maxRuns: filterMaxRuns ? parseInt(filterMaxRuns) : undefined,
    });
  }

  // Clear filters
  function clearFilters() {
    filterLevels = '';
    filterMinRuns = '';
    filterMaxRuns = '';
    catalogueStore.clearFilters();
  }

  // Re-apply filters when inputs change
  $effect(() => {
    filterLevels; filterMinRuns; filterMaxRuns;
    applyFilters();
  });

  // Load and use array (for double-click)
  async function useArray(info: StandardArrayInfo) {
    try {
      const oa = await catalogueStore.getArray(info.name);
      await arrayStore.add(oa);
      arrayStore.setCurrent(oa.id);
      goto(`/viewer/${oa.id}`);
    } catch (e) {
      console.error('Failed to load array:', e);
    }
  }

  // Select for preview - updates the context panel
  function selectForPreview(info: StandardArrayInfo) {
    selectedArray = info;
    context.set('catalogue-preview', info);
  }
</script>

<div class="catalogue-page">
  <header class="page-header">
    <div class="header-top">
      <div class="header-text">
        <h1 class="page-title">Standard Arrays</h1>
        <p class="page-subtitle">
          <span class="count">{catalogueStore.filtered.length}</span> Taguchi orthogonal arrays
        </p>
      </div>
    </div>

    <!-- Inline Filters -->
    <div class="filter-bar">
      <div class="filter-group">
        <label class="filter-label" for="filter-levels">Levels</label>
        <select id="filter-levels" class="filter-select" bind:value={filterLevels}>
          <option value="">All</option>
          <option value="2">2</option>
          <option value="3">3</option>
          <option value="5">5</option>
          <option value="6">6</option>
          <option value="7">7</option>
        </select>
      </div>

      <div class="filter-group">
        <label class="filter-label" for="filter-min">Min runs</label>
        <input
          id="filter-min"
          type="number"
          class="filter-input"
          placeholder="—"
          bind:value={filterMinRuns}
          min="1"
        />
      </div>

      <div class="filter-group">
        <label class="filter-label" for="filter-max">Max runs</label>
        <input
          id="filter-max"
          type="number"
          class="filter-input"
          placeholder="—"
          bind:value={filterMaxRuns}
          min="1"
        />
      </div>

      {#if hasActiveFilters}
        <button class="clear-btn" onclick={clearFilters}>
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
          Clear
        </button>
      {/if}
    </div>
  </header>

  <!-- Array Grid -->
  <main class="array-grid-container">
    {#if catalogueStore.loading && !catalogueStore.loaded}
      <div class="loading-state">
        <div class="spinner"></div>
        <p>Loading catalogue...</p>
      </div>
    {:else if catalogueStore.filtered.length === 0}
      <div class="empty-state">
        <p>No arrays match your filters.</p>
        <Button variant="secondary" onclick={clearFilters}>
          {#snippet children()}
            Clear Filters
          {/snippet}
        </Button>
      </div>
    {:else}
      <div class="array-grid">
        {#each catalogueStore.filtered as info}
          <button
            class="array-card"
            class:selected={selectedArray?.name === info.name}
            onclick={() => selectForPreview(info)}
            ondblclick={() => useArray(info)}
          >
            <div class="array-name">{info.name}</div>
            <div class="array-params">
              <span class="param">
                <span class="param-value">{info.runs}</span>
                <span class="param-label">runs</span>
              </span>
              <span class="param">
                <span class="param-value">{info.factors}</span>
                <span class="param-label">factors</span>
              </span>
              <span class="param">
                <span class="param-value">{info.levels}</span>
                <span class="param-label">levels</span>
              </span>
            </div>
            <p class="array-desc">{info.description}</p>
          </button>
        {/each}
      </div>
    {/if}
  </main>
</div>

<style>
  .catalogue-page {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .page-header {
    margin-bottom: var(--space-4);
    flex-shrink: 0;
  }

  .header-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: var(--space-4);
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

  .page-subtitle .count {
    font-weight: var(--font-semibold);
    color: var(--color-accent);
  }

  /* Filter Bar */
  .filter-bar {
    display: flex;
    align-items: flex-end;
    gap: var(--space-4);
    padding: var(--space-3);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-lg);
  }

  .filter-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .filter-label {
    font-size: var(--text-xs);
    font-weight: var(--font-medium);
    color: var(--color-text-muted);
  }

  .filter-select,
  .filter-input {
    height: 32px;
    padding: 0 var(--space-2);
    font-size: var(--text-sm);
    background-color: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    color: var(--color-text-primary);
  }

  .filter-select {
    min-width: 70px;
    cursor: pointer;
  }

  .filter-input {
    width: 80px;
  }

  .filter-input::placeholder {
    color: var(--color-text-muted);
  }

  .filter-select:focus,
  .filter-input:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  .clear-btn {
    display: flex;
    align-items: center;
    gap: var(--space-1);
    height: 32px;
    padding: 0 var(--space-3);
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    background: none;
    border: none;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all var(--duration-fast);
  }

  .clear-btn:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  /* Array Grid */
  .array-grid-container {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-1);
  }

  .array-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: var(--space-3);
  }

  .array-card {
    text-align: left;
    padding: var(--space-4);
    background-color: var(--color-bg-elevated);
    border: 2px solid var(--color-border);
    border-radius: var(--radius-lg);
    cursor: pointer;
    transition: all var(--duration-fast);
  }

  .array-card:hover {
    border-color: var(--color-accent);
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
  }

  .array-card.selected {
    border-color: var(--color-accent);
    background-color: var(--color-accent-subtle);
  }

  .array-name {
    font-size: var(--text-xl);
    font-weight: var(--font-bold);
    color: var(--color-accent);
    margin-bottom: var(--space-2);
  }

  .array-params {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    margin-bottom: var(--space-2);
  }

  .param {
    display: flex;
    align-items: baseline;
    gap: var(--space-1);
  }

  .param-value {
    font-family: var(--font-mono);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .param-label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .array-desc {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    line-height: var(--leading-snug);
  }

  /* Loading & Empty */
  .loading-state,
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--space-4);
    padding: var(--space-12);
    color: var(--color-text-muted);
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

  /* Responsive */
  @media (max-width: 640px) {
    .filter-bar {
      flex-wrap: wrap;
    }

    .filter-input {
      width: 70px;
    }
  }
</style>
