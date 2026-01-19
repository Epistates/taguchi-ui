<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { Button } from '$lib/components/form';
  import { BalanceChart, CorrelationHeatmap } from '$lib/components/charts';
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import { analysisStore } from '$lib/stores/analysis.svelte';
  import type { OAData } from '$lib/types';

  // Current array
  let array = $state<OAData | null>(null);
  let loading = $state(true);
  let analysisRun = $state(false);

  // Selected factor for balance chart
  let selectedFactor = $state(0);

  // Load array and run analysis on mount
  onMount(async () => {
    const id = $page.params.id;
    if (!id) {
      loading = false;
      return;
    }

    const found = arrayStore.get(id);
    if (found) {
      array = found;
      arrayStore.setCurrent(id);

      // Run full analysis
      try {
        await analysisStore.runFullAnalysis(found);
        analysisRun = true;
      } catch (e) {
        console.error('Analysis failed:', e);
      }
    }
    loading = false;
  });

  // Navigate back to viewer
  function goToViewer() {
    if (array) {
      goto(`/viewer/${array.id}`);
    }
  }

  // Re-run analysis
  async function rerunAnalysis() {
    if (!array) return;
    analysisRun = false;
    try {
      await analysisStore.runFullAnalysis(array);
      analysisRun = true;
    } catch (e) {
      console.error('Analysis failed:', e);
    }
  }

  // Computed notation
  let notation = $derived(
    array
      ? `OA(${array.runs}, ${array.factors}, ${array.levels[0]}, ${array.strength})`
      : ''
  );

  // Summary stats
  let summaryStats = $derived((() => {
    if (!analysisStore.verification || !analysisStore.balance) return null;

    const v = analysisStore.verification;
    const b = analysisStore.balance;

    const balancedCount = b.factorBalance.filter(Boolean).length;
    const totalFactors = b.factorBalance.length;

    return {
      strengthValid: v.isValid,
      actualStrength: v.actualStrength,
      claimedStrength: v.claimedStrength,
      issueCount: v.issues.length,
      balancedFactors: balancedCount,
      totalFactors,
      balancePercent: Math.round((balancedCount / totalFactors) * 100),
    };
  })());
</script>

<div class="analysis-page">
  {#if loading || analysisStore.loading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>{loading ? 'Loading array...' : 'Running analysis...'}</p>
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
    <!-- Header -->
    <header class="page-header">
      <div class="header-main">
        <Button variant="ghost" onclick={goToViewer}>
          {#snippet children()}
            &larr; Back
          {/snippet}
        </Button>
        <div class="header-title">
          <h1 class="page-title">Array Verification</h1>
          <span class="array-notation">{array.metadata.name || notation}</span>
        </div>
      </div>
      <div class="header-actions">
        <Button variant="secondary" onclick={rerunAnalysis} loading={analysisStore.loading}>
          {#snippet children()}
            Re-verify
          {/snippet}
        </Button>
      </div>
    </header>

    {#if analysisStore.error}
      <div class="error-banner">
        <p>{analysisStore.error}</p>
        <Button variant="ghost" size="sm" onclick={() => analysisStore.clearError()}>
          {#snippet children()}
            Dismiss
          {/snippet}
        </Button>
      </div>
    {/if}

    {#if analysisRun && summaryStats}
      <!-- Summary Cards -->
      <section class="summary-section">
        <div class="summary-card" class:success={summaryStats.strengthValid} class:error={!summaryStats.strengthValid}>
          <div class="summary-icon">
            {#if summaryStats.strengthValid}
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M20 6L9 17l-5-5"/>
              </svg>
            {:else}
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <line x1="12" y1="8" x2="12" y2="12"/>
                <line x1="12" y1="16" x2="12.01" y2="16"/>
              </svg>
            {/if}
          </div>
          <div class="summary-content">
            <span class="summary-label">Strength Verification</span>
            <span class="summary-value">
              {summaryStats.strengthValid ? 'Valid' : 'Invalid'}
            </span>
            <span class="summary-detail">
              Claimed: {summaryStats.claimedStrength}, Actual: {summaryStats.actualStrength}
            </span>
          </div>
        </div>

        <div class="summary-card" class:success={summaryStats.balancePercent === 100}>
          <div class="summary-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="3" width="7" height="7"/>
              <rect x="14" y="3" width="7" height="7"/>
              <rect x="14" y="14" width="7" height="7"/>
              <rect x="3" y="14" width="7" height="7"/>
            </svg>
          </div>
          <div class="summary-content">
            <span class="summary-label">Factor Balance</span>
            <span class="summary-value">
              {summaryStats.balancedFactors}/{summaryStats.totalFactors} Balanced
            </span>
            <span class="summary-detail">
              {summaryStats.balancePercent}% of factors perfectly balanced
            </span>
          </div>
        </div>

        <div class="summary-card">
          <div class="summary-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 2v20M2 12h20"/>
              <circle cx="12" cy="12" r="4"/>
            </svg>
          </div>
          <div class="summary-content">
            <span class="summary-label">Correlation</span>
            <span class="summary-value">Computed</span>
            <span class="summary-detail">
              {array.factors}×{array.factors} matrix
            </span>
          </div>
        </div>
      </section>

      <!-- Analysis Grid -->
      <div class="analysis-grid">
        <!-- Verification Details -->
        <section class="analysis-card">
          <h2 class="card-title">Strength Verification</h2>

          {#if analysisStore.verification}
            <div class="verification-result">
              <div class="result-row">
                <span class="result-label">Status</span>
                <span class="result-badge" class:valid={analysisStore.verification.isValid}>
                  {analysisStore.verification.isValid ? 'Valid OA' : 'Invalid OA'}
                </span>
              </div>
              <div class="result-row">
                <span class="result-label">Claimed Strength</span>
                <span class="result-value">{analysisStore.verification.claimedStrength}</span>
              </div>
              <div class="result-row">
                <span class="result-label">Actual Strength</span>
                <span class="result-value">{analysisStore.verification.actualStrength}</span>
              </div>

              {#if analysisStore.verification.issues.length > 0}
                <div class="issues-section">
                  <h4 class="issues-title">Issues Found ({analysisStore.verification.issues.length})</h4>
                  <ul class="issues-list">
                    {#each analysisStore.verification.issues as issue}
                      <li class="issue-item">
                        <span class="issue-type">{issue.issueType}</span>
                        <span class="issue-desc">{issue.description}</span>
                      </li>
                    {/each}
                  </ul>
                </div>
              {/if}
            </div>
          {/if}
        </section>

        <!-- Balance Analysis -->
        <section class="analysis-card">
          <h2 class="card-title">Factor Balance</h2>

          {#if analysisStore.balance}
            <div class="factor-selector">
              <label class="selector-label" for="factor-select">Factor:</label>
              <select id="factor-select" bind:value={selectedFactor} class="factor-select">
                {#each Array(array.factors) as _, idx}
                  <option value={idx}>
                    Factor {idx + 1}
                    {analysisStore.balance.factorBalance[idx] ? '(Balanced)' : '(Unbalanced)'}
                  </option>
                {/each}
              </select>
            </div>

            <BalanceChart data={analysisStore.balance} factorIndex={selectedFactor} height={180} />

            <div class="balance-summary">
              <h4 class="summary-title">All Factors</h4>
              <div class="factor-badges">
                {#each analysisStore.balance.factorBalance as balanced, idx}
                  <button
                    class="factor-badge"
                    class:balanced
                    class:selected={selectedFactor === idx}
                    onclick={() => selectedFactor = idx}
                  >
                    F{idx + 1}
                  </button>
                {/each}
              </div>
            </div>
          {/if}
        </section>

        <!-- Correlation Matrix -->
        <section class="analysis-card correlation-card">
          <h2 class="card-title">Correlation Matrix</h2>

          {#if analysisStore.correlation}
            <CorrelationHeatmap
              data={analysisStore.correlation}
              size={Math.min(400, array.factors * 50)}
            />
          {/if}
        </section>
      </div>
    {/if}
  {/if}
</div>

<style>
  .analysis-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-shrink: 0;
  }

  .header-main {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .header-title {
    display: flex;
    flex-direction: column;
  }

  .page-title {
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
    color: var(--color-text-primary);
  }

  .array-notation {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--color-text-muted);
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

  /* Summary Section */
  .summary-section {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: var(--space-4);
  }

  .summary-card {
    display: flex;
    gap: var(--space-3);
    padding: var(--space-4);
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .summary-card.success {
    border-color: var(--color-success);
    background-color: var(--color-success-subtle);
  }

  .summary-card.success .summary-icon {
    color: var(--color-success);
  }

  .summary-card.error {
    border-color: var(--color-error);
    background-color: var(--color-error-subtle);
  }

  .summary-card.error .summary-icon {
    color: var(--color-error);
  }

  .summary-icon {
    flex-shrink: 0;
    color: var(--color-accent);
  }

  .summary-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .summary-label {
    font-size: var(--text-xs);
    font-weight: var(--font-medium);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .summary-value {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .summary-detail {
    font-size: var(--text-xs);
    color: var(--color-text-secondary);
  }

  /* Analysis Grid */
  .analysis-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-4);
  }

  @media (max-width: 1024px) {
    .analysis-grid {
      grid-template-columns: 1fr;
    }
  }

  .analysis-card {
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-5);
  }

  .correlation-card {
    grid-column: span 2;
  }

  @media (max-width: 1024px) {
    .correlation-card {
      grid-column: span 1;
    }
  }

  .card-title {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-4);
  }

  /* Verification */
  .verification-result {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .result-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-2) 0;
    border-bottom: 1px solid var(--color-border-light);
  }

  .result-row:last-child {
    border-bottom: none;
  }

  .result-label {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
  }

  .result-value {
    font-family: var(--font-mono);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .result-badge {
    padding: 4px 12px;
    border-radius: var(--radius-full);
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    background-color: var(--color-error-subtle);
    color: var(--color-error);
  }

  .result-badge.valid {
    background-color: var(--color-success-subtle);
    color: var(--color-success);
  }

  .issues-section {
    margin-top: var(--space-3);
    padding-top: var(--space-3);
    border-top: 1px solid var(--color-border);
  }

  .issues-title {
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    color: var(--color-error);
    margin-bottom: var(--space-2);
  }

  .issues-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .issue-item {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: var(--space-2);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-sm);
  }

  .issue-type {
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
    color: var(--color-text-secondary);
  }

  .issue-desc {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    word-break: break-word;
  }

  /* Balance */
  .factor-selector {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin-bottom: var(--space-3);
  }

  .selector-label {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
  }

  .factor-select {
    padding: var(--space-1) var(--space-2);
    font-size: var(--text-sm);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-bg-primary);
    color: var(--color-text-primary);
  }

  .balance-summary {
    margin-top: var(--space-4);
    padding-top: var(--space-3);
    border-top: 1px solid var(--color-border-light);
  }

  .summary-title {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-secondary);
    margin-bottom: var(--space-2);
  }

  .factor-badges {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-1);
  }

  .factor-badge {
    padding: 4px 8px;
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
    border-radius: var(--radius-sm);
    border: 1px solid var(--color-border);
    background-color: var(--color-warning-subtle);
    color: var(--color-text-primary);
    cursor: pointer;
    transition: all var(--duration-fast);
  }

  .factor-badge.balanced {
    background-color: var(--color-success-subtle);
    border-color: var(--color-success);
    color: var(--color-success);
  }

  .factor-badge.selected {
    box-shadow: 0 0 0 2px var(--color-accent);
  }

  .factor-badge:hover {
    transform: scale(1.05);
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
</style>
