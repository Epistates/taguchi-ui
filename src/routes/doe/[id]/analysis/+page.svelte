<script lang="ts">
  import { goto } from '$app/navigation';
  import { onDestroy } from 'svelte';
  import { doeStore } from '$lib/stores/doe.svelte';
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import { context } from '$lib/stores/app.svelte';
  import {
    MainEffectsPlot,
    SNRatioPlot,
    ANOVATable,
    OptimalSettingsCard,
    FactorRankingChart,
  } from '$lib/components/doe';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  // Get data (access Map properties for reactivity)
  const config = $derived(doeStore.configs.get(data.configId));
  const array = $derived(config ? arrayStore.items.get(config.arrayId) : undefined);
  const analysis = $derived(doeStore.analyses.get(data.configId));

  // Clean up context when leaving the page
  onDestroy(() => {
    context.clear();
  });

  // Update context panel with experiment config
  $effect(() => {
    if (config) {
      context.set('experiment-config', {
        config,
        array: array ?? null,
      });
    }
  });

  // Tab state
  type TabId = 'overview' | 'main-effects' | 'sn-ratios' | 'anova';
  let activeTab = $state<TabId>('overview');

  const tabs: { id: TabId; label: string }[] = [
    { id: 'overview', label: 'Overview' },
    { id: 'main-effects', label: 'Main Effects' },
    { id: 'sn-ratios', label: 'S/N Ratios' },
    { id: 'anova', label: 'ANOVA' },
  ];

  function formatDate(isoString: string): string {
    try {
      return new Date(isoString).toLocaleString();
    } catch {
      return isoString;
    }
  }

  async function exportAnalysis() {
    if (!analysis || !config) return;

    const exportData = {
      experiment: config.name,
      analyzedAt: analysis.analyzedAt,
      grandMean: analysis.grandMean,
      snGrandMean: analysis.snGrandMean,
      mainEffects: analysis.mainEffects,
      snRatioEffects: analysis.snRatioEffects,
      anova: analysis.anova,
      optimalSettings: analysis.optimalSettings,
    };

    const blob = new Blob([JSON.stringify(exportData, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${config.name.replace(/\s+/g, '_')}_analysis.json`;
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<svelte:head>
  <title>Analysis | {config?.name ?? 'Experiment'} | Taguchi DOE</title>
</svelte:head>

<main class="analysis-page">
  {#if !config || !array}
    <div class="empty-state">
      <h2>Experiment Not Found</h2>
      <p>The selected experiment could not be found.</p>
      <button class="btn btn-primary" onclick={() => goto('/doe')}>
        View Experiments
      </button>
    </div>
  {:else if !analysis}
    <div class="empty-state">
      <h2>No Analysis Available</h2>
      <p>This experiment has not been analyzed yet. Enter all results first.</p>
      <button class="btn btn-primary" onclick={() => goto(`/doe/${data.configId}/results`)}>
        Enter Results
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
            DOE Analysis Results
            <span class="analyzed-date">Analyzed {formatDate(analysis.analyzedAt)}</span>
          </p>
        </div>
      </div>

      <div class="header-actions">
        <button class="btn btn-secondary" onclick={() => goto(`/doe/${data.configId}/results`)}>
          View Data
        </button>
        <button class="btn btn-primary" onclick={exportAnalysis}>
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M8 3v8M4 7l4 4 4-4"/>
            <path d="M2 12v2h12v-2"/>
          </svg>
          Export
        </button>
      </div>
    </header>

    <!-- Tabs -->
    <nav class="tabs">
      {#each tabs as tab}
        <button
          class="tab"
          class:active={activeTab === tab.id}
          onclick={() => activeTab = tab.id}
        >
          {tab.label}
        </button>
      {/each}
    </nav>

    <!-- Tab Content -->
    <div class="tab-content">
      {#if activeTab === 'overview'}
        <div class="overview-grid">
          <!-- Optimal Settings -->
          <div class="grid-item optimal">
            <OptimalSettingsCard
              settings={analysis.optimalSettings}
              factors={config.factors}
              snEffects={analysis.snRatioEffects}
              responseName={config.responses[0]?.name ?? 'Response'}
              responseUnit={config.responses[0]?.unit}
            />
          </div>

          <!-- Factor Ranking -->
          <div class="grid-item ranking card">
            <FactorRankingChart effects={analysis.mainEffects} />
          </div>

          <!-- Summary Stats -->
          <div class="grid-item summary card">
            <h3 class="card-title">Summary Statistics</h3>
            <dl class="stats-list">
              <div class="stat">
                <dt>Grand Mean</dt>
                <dd>{analysis.grandMean != null && isFinite(analysis.grandMean) ? analysis.grandMean.toFixed(3) : '-'}</dd>
              </div>
              <div class="stat">
                <dt>S/N Grand Mean</dt>
                <dd>{analysis.snGrandMean != null && isFinite(analysis.snGrandMean) ? `${analysis.snGrandMean.toFixed(2)} dB` : '-'}</dd>
              </div>
              <div class="stat">
                <dt>Total SS</dt>
                <dd>{analysis.anova.totalSs != null && isFinite(analysis.anova.totalSs) ? analysis.anova.totalSs.toFixed(3) : '-'}</dd>
              </div>
              <div class="stat">
                <dt>Error MS</dt>
                <dd>{analysis.anova.errorMs != null && isFinite(analysis.anova.errorMs) ? analysis.anova.errorMs.toFixed(3) : '-'}</dd>
              </div>
              <div class="stat">
                <dt>Factors</dt>
                <dd>{config.factors.length}</dd>
              </div>
              <div class="stat">
                <dt>Runs</dt>
                <dd>{array.runs}</dd>
              </div>
              <div class="stat">
                <dt>Replicates</dt>
                <dd>{config.replicates}</dd>
              </div>
            </dl>
          </div>

          <!-- Mini Main Effects Plot -->
          <div class="grid-item effects card">
            <h3 class="card-title">Main Effects</h3>
            <MainEffectsPlot
              effects={analysis.mainEffects}
              factors={config.factors}
              grandMean={analysis.grandMean}
            />
          </div>
        </div>

      {:else if activeTab === 'main-effects'}
        <div class="full-chart card">
          <h3 class="card-title">Main Effects Plot</h3>
          <p class="card-desc">
            Shows how the mean response changes with each factor level.
            Steeper lines indicate stronger factor effects.
          </p>
          <MainEffectsPlot
            effects={analysis.mainEffects}
            factors={config.factors}
            grandMean={analysis.grandMean}
          />
        </div>

        <div class="effects-table card">
          <h3 class="card-title">Level Means</h3>
          <table class="data-table">
            <thead>
              <tr>
                <th>Factor</th>
                {#each Array(Math.max(...analysis.mainEffects.map(e => e.levelMeans.length))) as _, i}
                  <th class="num">Level {i + 1}</th>
                {/each}
                <th class="num">Range</th>
                <th class="num">Rank</th>
              </tr>
            </thead>
            <tbody>
              {#each analysis.mainEffects as effect}
                <tr>
                  <td class="factor-name">{effect.factorName}</td>
                  {#each effect.levelMeans as mean}
                    <td class="num">{mean != null && isFinite(mean) ? mean.toFixed(3) : '-'}</td>
                  {/each}
                  <td class="num range">{effect.range != null && isFinite(effect.range) ? effect.range.toFixed(3) : '-'}</td>
                  <td class="num rank">#{effect.rank}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>

      {:else if activeTab === 'sn-ratios'}
        <div class="full-chart card">
          <h3 class="card-title">Signal-to-Noise Ratio Plot</h3>
          <p class="card-desc">
            Higher S/N ratio indicates better performance.
            The optimal level for each factor is marked.
          </p>
          <SNRatioPlot
            effects={analysis.snRatioEffects}
            factors={config.factors}
          />
        </div>

        <div class="effects-table card">
          <h3 class="card-title">S/N Ratios by Level (dB)</h3>
          <table class="data-table">
            <thead>
              <tr>
                <th>Factor</th>
                {#each Array(Math.max(...analysis.snRatioEffects.map(e => e.levelSnRatios.length))) as _, i}
                  <th class="num">Level {i + 1}</th>
                {/each}
                <th class="num">Optimal</th>
              </tr>
            </thead>
            <tbody>
              {#each analysis.snRatioEffects as effect}
                <tr>
                  <td class="factor-name">{effect.factorName}</td>
                  {#each effect.levelSnRatios as sn, i}
                    <td class="num" class:optimal={i === effect.optimalLevel}>
                      {sn != null && isFinite(sn) ? sn.toFixed(2) : '-'}
                    </td>
                  {/each}
                  <td class="num optimal">Level {effect.optimalLevel + 1}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>

      {:else if activeTab === 'anova'}
        <div class="anova-section card">
          <h3 class="card-title">Analysis of Variance (ANOVA)</h3>
          <p class="card-desc">
            ANOVA determines the statistical significance of each factor's contribution.
            Factors with p &lt; 0.05 are considered significant.
          </p>
          <ANOVATable anova={analysis.anova} />
        </div>
      {/if}
    </div>
  {/if}
</main>

<style>
  .analysis-page {
    max-width: 1400px;
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
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .analyzed-date {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
  }

  .analyzed-date::before {
    content: '|';
    margin-right: var(--space-2);
    color: var(--color-border);
  }

  .header-actions {
    display: flex;
    gap: var(--space-3);
  }

  /* Tabs */
  .tabs {
    display: flex;
    gap: var(--space-1);
    margin-bottom: var(--space-6);
    border-bottom: 1px solid var(--color-border);
  }

  .tab {
    padding: var(--space-3) var(--space-4);
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-secondary);
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
    transition: all var(--duration-fast);
  }

  .tab:hover {
    color: var(--color-text-primary);
  }

  .tab.active {
    color: var(--color-accent);
    border-bottom-color: var(--color-accent);
  }

  /* Tab Content */
  .tab-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

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
    margin-bottom: var(--space-2);
  }

  .card-desc {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin-bottom: var(--space-4);
  }

  /* Overview Grid */
  .overview-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: auto auto;
    gap: var(--space-6);
  }

  .grid-item.optimal {
    grid-column: span 2;
  }

  @media (max-width: 1024px) {
    .overview-grid {
      grid-template-columns: 1fr;
    }

    .grid-item.optimal {
      grid-column: span 1;
    }
  }

  /* Stats List */
  .stats-list {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-3);
  }

  .stat {
    display: flex;
    justify-content: space-between;
    padding: var(--space-2);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
  }

  .stat dt {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
  }

  .stat dd {
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    font-family: var(--font-mono);
    color: var(--color-text-primary);
  }

  /* Data Table */
  .data-table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--text-sm);
  }

  .data-table th,
  .data-table td {
    padding: var(--space-2) var(--space-3);
    text-align: left;
    border-bottom: 1px solid var(--color-border-light);
  }

  .data-table th {
    font-weight: var(--font-semibold);
    color: var(--color-text-secondary);
    background-color: var(--color-bg-tertiary);
  }

  .data-table .num {
    text-align: right;
    font-family: var(--font-mono);
  }

  .data-table .factor-name {
    font-weight: var(--font-medium);
  }

  .data-table .range {
    font-weight: var(--font-semibold);
  }

  .data-table .rank {
    color: var(--color-accent);
    font-weight: var(--font-bold);
  }

  .data-table .optimal {
    background-color: var(--color-success-subtle);
    color: var(--color-success);
    font-weight: var(--font-semibold);
  }

  /* Empty State */
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
</style>
