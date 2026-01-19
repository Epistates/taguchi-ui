<script lang="ts">
  import type { ANOVAResult } from '$lib/types';

  interface Props {
    anova: ANOVAResult;
  }

  let { anova }: Props = $props();

  function formatNumber(n: number | undefined | null, decimals: number = 2): string {
    if (n === undefined || n === null || !isFinite(n)) return '-';
    return n.toFixed(decimals);
  }

  function formatPValue(p: number | undefined | null): string {
    if (p === undefined || p === null || !isFinite(p)) return '-';
    if (p < 0.001) return '< 0.001';
    if (p < 0.01) return '< 0.01';
    if (p < 0.05) return '< 0.05';
    return p.toFixed(3);
  }

  function isSignificant(p: number | undefined | null): boolean {
    return p !== undefined && p !== null && isFinite(p) && p < 0.05;
  }
</script>

<div class="anova-container">
  <table class="anova-table">
    <thead>
      <tr>
        <th>Source</th>
        <th class="num">SS</th>
        <th class="num">DF</th>
        <th class="num">MS</th>
        <th class="num">F</th>
        <th class="num">p-value</th>
        <th class="num">Contribution</th>
      </tr>
    </thead>
    <tbody>
      {#each anova.entries as entry}
        <tr
          class:pooled={entry.pooled}
          class:significant={isSignificant(entry.pValue)}
        >
          <td class="source">
            {entry.factorName}
            {#if entry.pooled}
              <span class="pooled-badge">Pooled</span>
            {/if}
          </td>
          <td class="num">{formatNumber(entry.sumOfSquares, 3)}</td>
          <td class="num">{entry.degreesOfFreedom}</td>
          <td class="num">{formatNumber(entry.meanSquare, 3)}</td>
          <td class="num">{formatNumber(entry.fRatio, 2)}</td>
          <td class="num" class:sig={isSignificant(entry.pValue)}>
            {formatPValue(entry.pValue)}
            {#if isSignificant(entry.pValue)}
              <span class="sig-marker">*</span>
            {/if}
          </td>
          <td class="num contribution">
            <div class="contribution-bar">
              <div
                class="contribution-fill"
                style="width: {entry.contributionPercent}%"
              ></div>
            </div>
            <span>{formatNumber(entry.contributionPercent, 1)}%</span>
          </td>
        </tr>
      {/each}
      <tr class="error-row">
        <td class="source">Error</td>
        <td class="num">{formatNumber(anova.errorSs, 3)}</td>
        <td class="num">{anova.errorDf}</td>
        <td class="num">{formatNumber(anova.errorMs, 3)}</td>
        <td class="num">-</td>
        <td class="num">-</td>
        <td class="num">-</td>
      </tr>
      <tr class="total-row">
        <td class="source"><strong>Total</strong></td>
        <td class="num"><strong>{formatNumber(anova.totalSs, 3)}</strong></td>
        <td class="num"><strong>{anova.totalDf}</strong></td>
        <td class="num">-</td>
        <td class="num">-</td>
        <td class="num">-</td>
        <td class="num">-</td>
      </tr>
    </tbody>
  </table>

  <div class="legend">
    <span class="legend-item">
      <span class="sig-marker">*</span> Significant at p &lt; 0.05
    </span>
    <span class="legend-item pooled-legend">
      Pooled factors contribute to error term
    </span>
  </div>
</div>

<style>
  .anova-container {
    overflow-x: auto;
  }

  .anova-table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--text-sm);
  }

  .anova-table th,
  .anova-table td {
    padding: var(--space-2) var(--space-3);
    text-align: left;
    border-bottom: 1px solid var(--color-border-light);
  }

  .anova-table th {
    font-weight: var(--font-semibold);
    color: var(--color-text-secondary);
    background-color: var(--color-bg-tertiary);
    white-space: nowrap;
  }

  .num {
    text-align: right;
    font-family: var(--font-mono);
  }

  .source {
    font-weight: var(--font-medium);
  }

  tr.pooled {
    opacity: 0.6;
  }

  tr.pooled .source {
    font-style: italic;
  }

  tr.significant {
    background-color: var(--color-success-subtle);
  }

  .pooled-badge {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    font-style: italic;
    font-weight: normal;
    margin-left: var(--space-2);
  }

  .sig {
    color: var(--color-success);
    font-weight: var(--font-semibold);
  }

  .sig-marker {
    color: var(--color-success);
    font-weight: var(--font-bold);
    margin-left: var(--space-1);
  }

  .contribution {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .contribution-bar {
    width: 60px;
    height: 8px;
    background-color: var(--color-bg-tertiary);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .contribution-fill {
    height: 100%;
    background-color: var(--color-accent);
    transition: width var(--duration-normal);
  }

  .error-row {
    background-color: var(--color-bg-secondary);
  }

  .total-row {
    background-color: var(--color-bg-tertiary);
    border-top: 2px solid var(--color-border);
  }

  .legend {
    display: flex;
    gap: var(--space-4);
    margin-top: var(--space-3);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .legend-item {
    display: flex;
    align-items: center;
    gap: var(--space-1);
  }

  .pooled-legend {
    font-style: italic;
  }
</style>
