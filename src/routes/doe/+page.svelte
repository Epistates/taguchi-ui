<script lang="ts">
  import { goto } from '$app/navigation';
  import { doeStore } from '$lib/stores/doe.svelte';
  import { arrayStore } from '$lib/stores/arrays.svelte';

  const configs = $derived(doeStore.configList);

  function formatDate(isoString: string): string {
    try {
      return new Date(isoString).toLocaleDateString();
    } catch {
      return isoString;
    }
  }

  function getArrayInfo(arrayId: string) {
    const array = arrayStore.get(arrayId);
    if (!array) return null;
    return {
      name: array.metadata.name || `${array.runs}-run array`,
      runs: array.runs,
      factors: array.factors,
    };
  }

  function getProgress(configId: string): { complete: number; total: number } {
    const results = doeStore.getResults(configId);
    if (!results) return { complete: 0, total: 0 };
    const complete = results.measurements.filter(m => m.value !== null).length;
    return { complete, total: results.measurements.length };
  }

  function hasAnalysis(configId: string): boolean {
    return !!doeStore.getAnalysis(configId);
  }
</script>

<svelte:head>
  <title>DOE Experiments | Taguchi DOE</title>
</svelte:head>

<main class="doe-page">
  <header class="page-header">
    <div class="header-content">
      <h1>DOE Experiments</h1>
      <p class="subtitle">Manage your Design of Experiments</p>
    </div>
    <button class="btn btn-primary" onclick={() => goto('/catalogue')}>
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M8 3v10M3 8h10"/>
      </svg>
      New Experiment
    </button>
  </header>

  {#if configs.length === 0}
    <div class="empty-state">
      <div class="empty-icon">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M9 3H5a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-4"/>
          <path d="M9 15h3l8.5-8.5a2.121 2.121 0 00-3-3L9 12v3"/>
          <path d="M16 5l3 3"/>
        </svg>
      </div>
      <h2>No Experiments Yet</h2>
      <p>Create an orthogonal array and start an experiment to analyze your DOE results.</p>
      <div class="empty-actions">
        <button class="btn btn-secondary" onclick={() => goto('/catalogue')}>
          Browse Catalogue
        </button>
        <button class="btn btn-primary" onclick={() => goto('/builder')}>
          Build Custom Array
        </button>
      </div>
    </div>
  {:else}
    <div class="experiments-grid">
      {#each configs as config}
        {@const arrayInfo = getArrayInfo(config.arrayId)}
        {@const progress = getProgress(config.id)}
        {@const analyzed = hasAnalysis(config.id)}
        <article class="experiment-card">
          <div class="card-header">
            <h3 class="experiment-name">{config.name}</h3>
            {#if analyzed}
              <span class="badge badge-success">Analyzed</span>
            {:else if progress.complete === progress.total && progress.total > 0}
              <span class="badge badge-info">Ready</span>
            {:else}
              <span class="badge badge-neutral">In Progress</span>
            {/if}
          </div>

          <div class="card-meta">
            {#if arrayInfo}
              <span class="meta-item">
                <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="2" y="2" width="12" height="12" rx="1"/>
                  <path d="M2 6h12M6 2v12"/>
                </svg>
                {arrayInfo.name}
              </span>
            {/if}
            <span class="meta-item">
              <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="8" cy="8" r="6"/>
                <path d="M8 4v4l2 2"/>
              </svg>
              {formatDate(config.createdAt)}
            </span>
          </div>

          <div class="card-stats">
            <div class="stat">
              <span class="stat-value">{config.factors.length}</span>
              <span class="stat-label">Factors</span>
            </div>
            <div class="stat">
              <span class="stat-value">{config.replicates}</span>
              <span class="stat-label">Replicates</span>
            </div>
            <div class="stat">
              <span class="stat-value">{config.responses[0]?.name || '-'}</span>
              <span class="stat-label">Response</span>
            </div>
          </div>

          {#if progress.total > 0}
            <div class="progress-section">
              <div class="progress-bar">
                <div
                  class="progress-fill"
                  style="width: {(progress.complete / progress.total) * 100}%"
                ></div>
              </div>
              <span class="progress-text">
                {progress.complete} / {progress.total} measurements
              </span>
            </div>
          {/if}

          <div class="card-actions">
            {#if analyzed}
              <button
                class="btn btn-primary btn-sm"
                onclick={() => goto(`/doe/${config.id}/analysis`)}
              >
                View Analysis
              </button>
            {:else if progress.complete === progress.total && progress.total > 0}
              <button
                class="btn btn-primary btn-sm"
                onclick={() => goto(`/doe/${config.id}/results`)}
              >
                Run Analysis
              </button>
            {:else}
              <button
                class="btn btn-primary btn-sm"
                onclick={() => goto(`/doe/${config.id}/results`)}
              >
                Enter Results
              </button>
            {/if}
            <button
              class="btn btn-ghost btn-sm"
              onclick={() => doeStore.deleteConfig(config.id)}
            >
              Delete
            </button>
          </div>
        </article>
      {/each}
    </div>
  {/if}
</main>

<style>
  .doe-page {
    max-width: 1200px;
    margin: 0 auto;
    padding: var(--space-6);
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: var(--space-8);
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

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: var(--space-12);
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .empty-icon {
    color: var(--color-text-muted);
    margin-bottom: var(--space-4);
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
    max-width: 400px;
  }

  .empty-actions {
    display: flex;
    gap: var(--space-3);
  }

  .experiments-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: var(--space-4);
  }

  .experiment-card {
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    transition: border-color var(--duration-fast);
  }

  .experiment-card:hover {
    border-color: var(--color-border-strong);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: var(--space-2);
  }

  .experiment-name {
    font-size: var(--text-base);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .card-meta {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-3);
  }

  .meta-item {
    display: flex;
    align-items: center;
    gap: var(--space-1);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .card-stats {
    display: flex;
    gap: var(--space-4);
    padding: var(--space-3) 0;
    border-top: 1px solid var(--color-border-light);
    border-bottom: 1px solid var(--color-border-light);
  }

  .stat {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .stat-value {
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .stat-label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .progress-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .progress-bar {
    height: 4px;
    background-color: var(--color-bg-tertiary);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background-color: var(--color-accent);
    transition: width var(--duration-normal);
  }

  .progress-text {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .card-actions {
    display: flex;
    gap: var(--space-2);
    margin-top: auto;
  }

  .btn-sm {
    padding: var(--space-1) var(--space-3);
    font-size: var(--text-xs);
  }
</style>
