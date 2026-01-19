<script lang="ts">
  import type { DOEConfig, OAData } from '$lib/types';
  import { doeStore } from '$lib/stores/doe.svelte';

  interface Props {
    config: DOEConfig;
    array: OAData;
    onUpdate?: () => void;
  }

  let { config, array, onUpdate }: Props = $props();

  // Focus management
  let focusedCell: { run: number; rep: number } | null = $state(null);

  function getLevelValue(factorIdx: number, levelIdx: number): string {
    const factor = config.factors[factorIdx];
    if (!factor) return String(levelIdx);
    const value = factor.levelValues[levelIdx];
    return value !== undefined ? String(value) : String(levelIdx);
  }

  function handleValueChange(runIdx: number, repIdx: number, value: string) {
    const responseId = config.responses[0]?.id;
    if (!responseId) return;

    const numValue = value.trim() === '' ? null : parseFloat(value);
    doeStore.updateMeasurement(
      config.id,
      runIdx,
      repIdx,
      responseId,
      isNaN(numValue as number) ? null : numValue
    );

    // Notify parent of update
    onUpdate?.();
  }

  function getValue(runIdx: number, repIdx: number): string {
    const responseId = config.responses[0]?.id;
    if (!responseId) return '';
    const value = doeStore.getMeasurement(config.id, runIdx, repIdx, responseId);
    return value === null ? '' : String(value);
  }

  function getRunAverage(runIdx: number): string {
    const responseId = config.responses[0]?.id;
    if (!responseId) return '-';
    const avg = doeStore.getRunAverage(config.id, runIdx, responseId);
    return avg === null ? '-' : avg.toFixed(2);
  }

  function handleKeydown(e: KeyboardEvent, runIdx: number, repIdx: number) {
    const totalRuns = array.runs;
    const totalReps = config.replicates;

    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        if (runIdx < totalRuns - 1) {
          focusCell(runIdx + 1, repIdx);
        }
        break;
      case 'ArrowUp':
        e.preventDefault();
        if (runIdx > 0) {
          focusCell(runIdx - 1, repIdx);
        }
        break;
      case 'ArrowRight':
      case 'Tab':
        if (e.key === 'Tab' && e.shiftKey) break;
        if (e.key === 'ArrowRight') e.preventDefault();
        if (repIdx < totalReps - 1) {
          if (e.key === 'ArrowRight') focusCell(runIdx, repIdx + 1);
        } else if (runIdx < totalRuns - 1) {
          if (e.key === 'ArrowRight') focusCell(runIdx + 1, 0);
        }
        break;
      case 'ArrowLeft':
        e.preventDefault();
        if (repIdx > 0) {
          focusCell(runIdx, repIdx - 1);
        } else if (runIdx > 0) {
          focusCell(runIdx - 1, totalReps - 1);
        }
        break;
      case 'Enter':
        e.preventDefault();
        if (runIdx < totalRuns - 1) {
          focusCell(runIdx + 1, repIdx);
        }
        break;
    }
  }

  function focusCell(runIdx: number, repIdx: number) {
    focusedCell = { run: runIdx, rep: repIdx };
    const input = document.querySelector(
      `input[data-cell="${runIdx}-${repIdx}"]`
    ) as HTMLInputElement;
    if (input) {
      input.focus();
      input.select();
    }
  }
</script>

<div class="results-grid-container">
  <table class="results-table">
    <thead>
      <tr>
        <th class="col-run">Run</th>
        {#each config.factors as factor, idx}
          <th class="col-factor" title={factor.unit ? `${factor.name} (${factor.unit})` : factor.name}>
            {factor.name}
          </th>
        {/each}
        {#each Array(config.replicates) as _, repIdx}
          <th class="col-value">
            {#if config.replicates > 1}
              Rep {repIdx + 1}
            {:else}
              Value
            {/if}
          </th>
        {/each}
        <th class="col-avg">Avg</th>
      </tr>
    </thead>
    <tbody>
      {#each array.data as row, runIdx}
        <tr>
          <td class="run-number">{runIdx + 1}</td>
          {#each row as level, factorIdx}
            <td class="factor-value" title={`Level ${level}`}>
              {getLevelValue(factorIdx, level)}
            </td>
          {/each}
          {#each Array(config.replicates) as _, repIdx}
            <td class="value-cell">
              <input
                type="number"
                step="any"
                class="value-input"
                data-cell="{runIdx}-{repIdx}"
                value={getValue(runIdx, repIdx)}
                placeholder="—"
                oninput={(e) => handleValueChange(runIdx, repIdx, e.currentTarget.value)}
                onkeydown={(e) => handleKeydown(e, runIdx, repIdx)}
                onfocus={() => focusedCell = { run: runIdx, rep: repIdx }}
              />
            </td>
          {/each}
          <td class="avg-value">
            {getRunAverage(runIdx)}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .results-grid-container {
    overflow: auto;
    max-height: 70vh;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .results-table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--text-sm);
  }

  .results-table th,
  .results-table td {
    padding: var(--space-2) var(--space-3);
    text-align: center;
    border-bottom: 1px solid var(--color-border-light);
  }

  .results-table th {
    font-weight: var(--font-semibold);
    color: var(--color-text-secondary);
    background-color: var(--color-bg-tertiary);
    position: sticky;
    top: 0;
    z-index: 1;
    white-space: nowrap;
  }

  .col-run {
    width: 50px;
    background-color: var(--color-bg-tertiary) !important;
  }

  .col-factor {
    min-width: 80px;
    max-width: 120px;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .col-value {
    min-width: 80px;
    background-color: var(--color-accent-subtle) !important;
  }

  .col-avg {
    width: 80px;
    background-color: var(--color-bg-secondary) !important;
  }

  .run-number {
    font-weight: var(--font-semibold);
    color: var(--color-text-muted);
    background-color: var(--color-bg-secondary);
    font-family: var(--font-mono);
  }

  .factor-value {
    font-family: var(--font-mono);
    color: var(--color-text-primary);
    background-color: var(--color-bg-elevated);
  }

  .value-cell {
    padding: 0;
    background-color: var(--color-bg-primary);
  }

  .value-input {
    width: 100%;
    padding: var(--space-2) var(--space-2);
    font-size: var(--text-sm);
    font-family: var(--font-mono);
    text-align: center;
    border: none;
    background-color: transparent;
    color: var(--color-text-primary);
    transition: background-color var(--duration-fast);
  }

  .value-input:focus {
    outline: none;
    background-color: var(--color-accent-subtle);
    box-shadow: inset 0 0 0 2px var(--color-accent);
  }

  .value-input::placeholder {
    color: var(--color-text-disabled);
  }

  /* Hide spinner buttons */
  .value-input::-webkit-outer-spin-button,
  .value-input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .value-input[type=number] {
    -moz-appearance: textfield;
  }

  .avg-value {
    font-weight: var(--font-semibold);
    font-family: var(--font-mono);
    color: var(--color-text-primary);
    background-color: var(--color-bg-secondary);
  }

  /* Alternating row colors */
  tbody tr:nth-child(even) .factor-value {
    background-color: var(--color-bg-primary);
  }

  tbody tr:nth-child(even) .value-cell {
    background-color: var(--color-bg-elevated);
  }

  tbody tr:hover .factor-value,
  tbody tr:hover .value-cell,
  tbody tr:hover .avg-value {
    background-color: var(--color-bg-hover);
  }
</style>
