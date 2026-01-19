<script lang="ts">
  import type { OAData } from '$lib/types';

  interface Props {
    data: OAData;
    showHeaders?: boolean;
    highlightRow?: number | null;
    highlightCol?: number | null;
    selectable?: boolean;
    compact?: boolean;
    onCellClick?: (row: number, col: number, value: number) => void;
  }

  let {
    data,
    showHeaders = true,
    highlightRow = null,
    highlightCol = null,
    selectable = false,
    compact = false,
    onCellClick,
  }: Props = $props();

  // Level colors (CSS variables)
  const levelColors = [
    'var(--color-level-0)',
    'var(--color-level-1)',
    'var(--color-level-2)',
    'var(--color-level-3)',
    'var(--color-level-4)',
    'var(--color-level-5)',
    'var(--color-level-6)',
    'var(--color-level-7)',
    'var(--color-level-8)',
    'var(--color-level-9)',
  ];

  function getCellColor(value: number): string {
    return levelColors[value % levelColors.length];
  }

  function handleCellClick(row: number, col: number, value: number) {
    if (selectable && onCellClick) {
      onCellClick(row, col, value);
    }
  }

  // Determine max levels for this array
  let maxLevel = $derived(Math.max(...data.levels));
</script>

<div class="matrix-container" class:compact>
  <div class="matrix-wrapper">
    <table class="matrix-table">
      {#if showHeaders}
        <thead>
          <tr>
            <th class="row-header corner">#</th>
            {#each Array(data.factors) as _, col}
              <th
                class="col-header"
                class:highlighted={highlightCol === col}
              >
                F{col + 1}
              </th>
            {/each}
          </tr>
        </thead>
      {/if}
      <tbody>
        {#each data.data as row, rowIdx}
          <tr class:highlighted={highlightRow === rowIdx}>
            {#if showHeaders}
              <th class="row-header">{rowIdx + 1}</th>
            {/if}
            {#each row as cell, colIdx}
              <td
                class="cell"
                class:selectable
                class:col-highlighted={highlightCol === colIdx}
                style="--cell-bg: {getCellColor(cell)}"
                onclick={() => handleCellClick(rowIdx, colIdx, cell)}
                role={selectable ? 'button' : undefined}
                tabindex={selectable ? 0 : undefined}
              >
                {cell}
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <!-- Legend -->
  <div class="matrix-legend">
    <span class="legend-label">Levels:</span>
    {#each Array(maxLevel) as _, level}
      <span class="legend-item" style="--level-bg: {getCellColor(level)}">
        {level}
      </span>
    {/each}
  </div>
</div>

<style>
  .matrix-container {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .matrix-wrapper {
    overflow: auto;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-bg-elevated);
  }

  .matrix-table {
    border-collapse: collapse;
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    width: 100%;
  }

  .compact .matrix-table {
    font-size: var(--text-xs);
  }

  /* Headers */
  .col-header,
  .row-header {
    padding: var(--space-2) var(--space-3);
    background-color: var(--color-bg-secondary);
    font-weight: var(--font-semibold);
    color: var(--color-text-secondary);
    text-align: center;
    position: sticky;
    z-index: 1;
  }

  .compact .col-header,
  .compact .row-header {
    padding: var(--space-1) var(--space-2);
  }

  .col-header {
    top: 0;
    border-bottom: 2px solid var(--color-border);
  }

  .row-header {
    left: 0;
    border-right: 2px solid var(--color-border);
    min-width: 40px;
  }

  .compact .row-header {
    min-width: 32px;
  }

  .corner {
    position: sticky;
    top: 0;
    left: 0;
    z-index: 2;
    background-color: var(--color-bg-secondary);
  }

  .col-header.highlighted {
    background-color: var(--color-accent-subtle);
    color: var(--color-accent);
  }

  tr.highlighted .row-header {
    background-color: var(--color-accent-subtle);
    color: var(--color-accent);
  }

  /* Cells */
  .cell {
    padding: var(--space-2) var(--space-3);
    text-align: center;
    background-color: var(--cell-bg);
    color: var(--color-text-on-colored);
    border: 1px solid var(--color-border-light);
    transition: transform var(--duration-fast), box-shadow var(--duration-fast);
  }

  .compact .cell {
    padding: var(--space-1) var(--space-2);
  }

  .cell.selectable {
    cursor: pointer;
  }

  .cell.selectable:hover {
    transform: scale(1.1);
    box-shadow: var(--shadow-md);
    z-index: 1;
    position: relative;
  }

  .cell.col-highlighted {
    box-shadow: inset 0 0 0 2px var(--color-accent);
  }

  tr.highlighted .cell {
    box-shadow: inset 0 0 0 2px var(--color-accent);
  }

  /* Legend */
  .matrix-legend {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex-wrap: wrap;
    padding: var(--space-2);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
  }

  .legend-label {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    font-weight: var(--font-medium);
  }

  .legend-item {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    border-radius: var(--radius-sm);
    background-color: var(--level-bg);
    color: var(--color-text-on-colored);
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
  }

  .compact .legend-item {
    width: 20px;
    height: 20px;
  }
</style>
