<script lang="ts">
  import type { MainEffect } from '$lib/types';

  interface Props {
    effects: MainEffect[];
  }

  let { effects }: Props = $props();

  // Sort by range (highest = most important)
  const sortedEffects = $derived(() => {
    return [...effects].sort((a, b) => (b.range ?? 0) - (a.range ?? 0));
  });

  // Calculate max range for percentage
  const maxRange = $derived(() => {
    const ranges = effects.map(e => e.range ?? 0).filter(r => isFinite(r));
    return ranges.length > 0 ? Math.max(...ranges) : 1;
  });

  // Helper to safely format numbers
  function safeFormat(n: number | null | undefined, decimals: number = 2): string {
    if (n == null || !isFinite(n)) return '-';
    return n.toFixed(decimals);
  }
</script>

<div class="ranking-chart">
  <h4 class="chart-title">Factor Importance</h4>
  <div class="rankings">
    {#each sortedEffects() as effect, idx}
      {@const range = effect.range ?? 0}
      {@const max = maxRange()}
      {@const percentage = max > 0 ? (range / max) * 100 : 0}
      <div class="ranking-item">
        <div class="rank-badge" class:top={idx === 0}>
          #{effect.rank}
        </div>
        <div class="factor-info">
          <span class="factor-name">{effect.factorName}</span>
          <span class="factor-range">Range: {safeFormat(effect.range)}</span>
        </div>
        <div class="bar-container">
          <div
            class="bar-fill"
            class:top={idx === 0}
            style="width: {isFinite(percentage) ? percentage : 0}%"
          ></div>
        </div>
        <span class="percentage">{isFinite(percentage) ? percentage.toFixed(0) : '-'}%</span>
      </div>
    {/each}
  </div>
  <p class="chart-hint">
    Factors with larger range have more influence on the response variable.
  </p>
</div>

<style>
  .ranking-chart {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .chart-title {
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    color: var(--color-text-secondary);
    margin-bottom: var(--space-2);
  }

  .rankings {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .ranking-item {
    display: grid;
    grid-template-columns: 40px 1fr 2fr 50px;
    align-items: center;
    gap: var(--space-3);
  }

  .rank-badge {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-full);
    background-color: var(--color-bg-tertiary);
    font-size: var(--text-xs);
    font-weight: var(--font-bold);
    color: var(--color-text-muted);
  }

  .rank-badge.top {
    background-color: var(--color-accent);
    color: var(--color-text-inverse);
  }

  .factor-info {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .factor-name {
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .factor-range {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    font-family: var(--font-mono);
  }

  .bar-container {
    height: 12px;
    background-color: var(--color-bg-tertiary);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .bar-fill {
    height: 100%;
    background-color: var(--color-info);
    transition: width var(--duration-normal);
    border-radius: var(--radius-full);
  }

  .bar-fill.top {
    background-color: var(--color-accent);
  }

  .percentage {
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    text-align: right;
  }

  .chart-hint {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    font-style: italic;
  }
</style>
