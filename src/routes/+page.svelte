<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount, onDestroy } from 'svelte';
  import { Button } from '$lib/components/form';
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import { doeStore } from '$lib/stores/doe.svelte';
  import { context } from '$lib/stores/app.svelte';

  // Set dashboard context on mount
  onMount(() => {
    context.set('dashboard');
  });

  // Clean up context when leaving
  onDestroy(() => {
    context.clear();
  });

  // Recent arrays (sorted by creation date, newest first)
  let recentArrays = $derived(
    arrayStore.list
      .sort((a, b) => {
        const dateA = a.metadata.createdAt ? new Date(a.metadata.createdAt).getTime() : 0;
        const dateB = b.metadata.createdAt ? new Date(b.metadata.createdAt).getTime() : 0;
        return dateB - dateA;
      })
      .slice(0, 4)
  );

  // Recent experiments (sorted by update date, newest first)
  let recentExperiments = $derived(
    doeStore.configList
      .sort((a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime())
      .slice(0, 4)
  );

  // Check if user has any data
  let hasData = $derived(arrayStore.list.length > 0 || doeStore.configList.length > 0);

  // Get experiment status
  function getExperimentStatus(configId: string): 'new' | 'in-progress' | 'analyzed' {
    const analysis = doeStore.analyses.get(configId);
    if (analysis) return 'analyzed';

    const results = doeStore.results.get(configId);
    if (results && results.measurements.some(m => m.value !== null)) {
      return 'in-progress';
    }
    return 'new';
  }

  // Format relative time
  function formatRelativeTime(isoString: string): string {
    const date = new Date(isoString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    if (diffDays < 7) return `${diffDays}d ago`;
    return date.toLocaleDateString();
  }

  // Standard arrays for quick access (when no recent data)
  const standardArrays = [
    { name: 'L4', runs: 4, factors: 3, levels: 2, desc: 'Smallest 2-level' },
    { name: 'L8', runs: 8, factors: 7, levels: 2, desc: 'Common 2-level' },
    { name: 'L9', runs: 9, factors: 4, levels: 3, desc: 'Common 3-level' },
    { name: 'L12', runs: 12, factors: 11, levels: 2, desc: 'Plackett-Burman' },
    { name: 'L18', runs: 18, factors: 7, levels: 3, desc: 'Mixed levels' },
    { name: 'L27', runs: 27, factors: 13, levels: 3, desc: 'Large 3-level' },
  ];
</script>

<div class="dashboard">
  <header class="dashboard-header">
    <h1 class="page-title">Taguchi DOE</h1>
    <p class="page-subtitle">Design of Experiments with Orthogonal Arrays</p>
  </header>

  <!-- Quick Actions -->
  <section class="quick-actions">
    <a href="/builder" class="action-card primary">
      <div class="action-icon">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="12" y1="5" x2="12" y2="19"/>
          <line x1="5" y1="12" x2="19" y2="12"/>
        </svg>
      </div>
      <div class="action-text">
        <span class="action-label">New Array</span>
        <span class="action-desc">Build a custom OA</span>
      </div>
    </a>

    <a href="/catalogue" class="action-card">
      <div class="action-icon">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="7" height="7"/>
          <rect x="14" y="3" width="7" height="7"/>
          <rect x="14" y="14" width="7" height="7"/>
          <rect x="3" y="14" width="7" height="7"/>
        </svg>
      </div>
      <div class="action-text">
        <span class="action-label">Catalogue</span>
        <span class="action-desc">Standard arrays</span>
      </div>
    </a>

    <a href="/doe" class="action-card">
      <div class="action-icon">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M3 3v18h18"/>
          <path d="M18 9l-5 5-4-4-3 3"/>
        </svg>
      </div>
      <div class="action-text">
        <span class="action-label">Experiments</span>
        <span class="action-desc">Run DOE analysis</span>
      </div>
    </a>
  </section>

  {#if hasData}
    <!-- Recent Arrays -->
    {#if recentArrays.length > 0}
      <section class="recent-section">
        <div class="section-header">
          <h2 class="section-title">Recent Arrays</h2>
          {#if arrayStore.list.length > 4}
            <a href="/catalogue" class="view-all">View all</a>
          {/if}
        </div>
        <div class="recent-grid">
          {#each recentArrays as array}
            <button class="recent-card" onclick={() => goto(`/viewer/${array.id}`)}>
              <div class="recent-main">
                <span class="recent-name">{array.metadata.name || 'Unnamed Array'}</span>
                <span class="recent-notation">
                  OA({array.runs}, {array.factors}, {array.levels[0]}, {array.strength})
                </span>
              </div>
              <div class="recent-meta">
                <span class="recent-time">{formatRelativeTime(array.metadata.createdAt || new Date().toISOString())}</span>
              </div>
            </button>
          {/each}
        </div>
      </section>
    {/if}

    <!-- Recent Experiments -->
    {#if recentExperiments.length > 0}
      <section class="recent-section">
        <div class="section-header">
          <h2 class="section-title">Recent Experiments</h2>
          {#if doeStore.configList.length > 4}
            <a href="/doe" class="view-all">View all</a>
          {/if}
        </div>
        <div class="recent-grid">
          {#each recentExperiments as exp}
            {@const status = getExperimentStatus(exp.id)}
            <button
              class="recent-card experiment"
              onclick={() => {
                if (status === 'analyzed') {
                  goto(`/doe/${exp.id}/analysis`);
                } else if (status === 'in-progress') {
                  goto(`/doe/${exp.id}/results`);
                } else {
                  goto(`/doe/${exp.id}/results`);
                }
              }}
            >
              <div class="recent-main">
                <span class="recent-name">{exp.name}</span>
                <span class="recent-notation">
                  {exp.factors.length} factors, {exp.replicates} rep{exp.replicates > 1 ? 's' : ''}
                </span>
              </div>
              <div class="recent-meta">
                <span class="status-badge" class:analyzed={status === 'analyzed'} class:in-progress={status === 'in-progress'}>
                  {status === 'analyzed' ? 'Analyzed' : status === 'in-progress' ? 'In Progress' : 'New'}
                </span>
              </div>
            </button>
          {/each}
        </div>
      </section>
    {/if}
  {:else}
    <!-- Getting Started - shown when no data -->
    <section class="getting-started">
      <h2 class="section-title">Get Started</h2>
      <p class="getting-started-text">
        Choose a standard Taguchi array to begin your first experiment, or build a custom array.
      </p>
      <div class="standard-grid">
        {#each standardArrays as array}
          <a href="/catalogue?select={array.name}" class="standard-card">
            <div class="standard-name">{array.name}</div>
            <div class="standard-stats">
              <span>{array.runs} runs</span>
              <span>{array.factors} factors</span>
              <span>{array.levels} levels</span>
            </div>
            <div class="standard-desc">{array.desc}</div>
          </a>
        {/each}
      </div>
    </section>
  {/if}
</div>

<style>
  .dashboard {
    max-width: 900px;
    margin: 0 auto;
  }

  .dashboard-header {
    margin-bottom: var(--space-8);
  }

  .page-title {
    font-size: var(--text-3xl);
    font-weight: var(--font-bold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-1);
  }

  .page-subtitle {
    font-size: var(--text-base);
    color: var(--color-text-muted);
  }

  /* Quick Actions */
  .quick-actions {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-4);
    margin-bottom: var(--space-8);
  }

  @media (max-width: 640px) {
    .quick-actions {
      grid-template-columns: 1fr;
    }
  }

  .action-card {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-4);
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    text-decoration: none;
    transition: all var(--duration-fast);
  }

  .action-card:hover {
    border-color: var(--color-accent);
    box-shadow: var(--shadow-md);
    transform: translateY(-2px);
    text-decoration: none;
  }

  .action-card.primary {
    background-color: var(--color-accent);
    border-color: var(--color-accent);
    color: var(--color-text-inverse);
  }

  .action-card.primary:hover {
    background-color: var(--color-accent-hover);
    border-color: var(--color-accent-hover);
  }

  .action-icon {
    width: 44px;
    height: 44px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--color-bg-hover);
    border-radius: var(--radius-md);
    flex-shrink: 0;
  }

  .action-card.primary .action-icon {
    background-color: rgba(255, 255, 255, 0.2);
  }

  .action-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .action-label {
    font-weight: var(--font-semibold);
    color: inherit;
  }

  .action-card:not(.primary) .action-label {
    color: var(--color-text-primary);
  }

  .action-desc {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .action-card.primary .action-desc {
    color: rgba(255, 255, 255, 0.8);
  }

  /* Section Header */
  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-4);
  }

  .section-title {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .view-all {
    font-size: var(--text-sm);
    color: var(--color-accent);
  }

  /* Recent Section */
  .recent-section {
    margin-bottom: var(--space-8);
  }

  .recent-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-3);
  }

  @media (max-width: 640px) {
    .recent-grid {
      grid-template-columns: 1fr;
    }
  }

  .recent-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-4);
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    text-align: left;
    cursor: pointer;
    transition: all var(--duration-fast);
  }

  .recent-card:hover {
    border-color: var(--color-accent);
    box-shadow: var(--shadow-sm);
  }

  .recent-main {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .recent-name {
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
  }

  .recent-notation {
    font-size: var(--text-xs);
    font-family: var(--font-mono);
    color: var(--color-text-muted);
  }

  .recent-meta {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .recent-time {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .status-badge {
    padding: 2px 8px;
    font-size: var(--text-xs);
    font-weight: var(--font-medium);
    border-radius: var(--radius-full);
    background-color: var(--color-bg-tertiary);
    color: var(--color-text-secondary);
  }

  .status-badge.analyzed {
    background-color: var(--color-success-subtle);
    color: var(--color-success);
  }

  .status-badge.in-progress {
    background-color: var(--color-warning-subtle);
    color: var(--color-text-primary);
  }

  /* Getting Started */
  .getting-started {
    margin-bottom: var(--space-8);
  }

  .getting-started-text {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    margin-bottom: var(--space-4);
  }

  .standard-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: var(--space-3);
  }

  .standard-card {
    padding: var(--space-4);
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    text-decoration: none;
    transition: all var(--duration-fast);
  }

  .standard-card:hover {
    border-color: var(--color-accent);
    box-shadow: var(--shadow-sm);
    text-decoration: none;
  }

  .standard-name {
    font-size: var(--text-xl);
    font-weight: var(--font-bold);
    color: var(--color-accent);
    margin-bottom: var(--space-2);
  }

  .standard-stats {
    display: flex;
    flex-direction: column;
    gap: 2px;
    font-size: var(--text-xs);
    color: var(--color-text-secondary);
    margin-bottom: var(--space-2);
  }

  .standard-desc {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }
</style>
