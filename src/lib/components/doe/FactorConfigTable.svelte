<script lang="ts">
  import type { ExperimentFactor } from '$lib/types';

  interface Props {
    factors: ExperimentFactor[];
    numLevels: number[];
    onUpdate: (factorId: string, updates: Partial<ExperimentFactor>) => void;
  }

  let { factors, numLevels, onUpdate }: Props = $props();

  function handleNameChange(factorId: string, value: string) {
    onUpdate(factorId, { name: value });
  }

  function handleUnitChange(factorId: string, value: string) {
    onUpdate(factorId, { unit: value || undefined });
  }

  function handleLevelValueChange(factorId: string, levelIndex: number, value: string) {
    const factor = factors.find(f => f.id === factorId);
    if (!factor) return;

    const newLevelValues = [...factor.levelValues];
    // Try to parse as number, otherwise keep as string
    const numValue = parseFloat(value);
    newLevelValues[levelIndex] = isNaN(numValue) ? value : numValue;
    onUpdate(factorId, { levelValues: newLevelValues });
  }

  // Get max levels across all factors for table width
  const maxLevels = $derived(Math.max(...numLevels, 2));
</script>

<div class="factor-config">
  <table class="factor-table">
    <thead>
      <tr>
        <th class="col-factor">Factor</th>
        <th class="col-name">Name</th>
        <th class="col-unit">Unit</th>
        {#each Array(maxLevels) as _, i}
          <th class="col-level">Level {i + 1}</th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each factors as factor, idx}
        {@const factorLevels = numLevels[idx] ?? numLevels[0]}
        <tr>
          <td class="factor-label">F{idx + 1}</td>
          <td>
            <input
              type="text"
              class="form-input"
              value={factor.name}
              placeholder="Factor name"
              oninput={(e) => handleNameChange(factor.id, e.currentTarget.value)}
            />
          </td>
          <td>
            <input
              type="text"
              class="form-input unit-input"
              value={factor.unit ?? ''}
              placeholder="Unit"
              oninput={(e) => handleUnitChange(factor.id, e.currentTarget.value)}
            />
          </td>
          {#each Array(maxLevels) as _, levelIdx}
            {#if levelIdx < factorLevels}
              <td>
                <input
                  type="text"
                  class="form-input level-input"
                  value={factor.levelValues[levelIdx] ?? levelIdx}
                  placeholder={`L${levelIdx + 1}`}
                  oninput={(e) => handleLevelValueChange(factor.id, levelIdx, e.currentTarget.value)}
                />
              </td>
            {:else}
              <td class="empty-cell"></td>
            {/if}
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>

  <p class="hint">
    Enter meaningful names and actual level values for your factors.
    Level values can be numeric (100, 150, 200) or categorical (Low, Medium, High).
  </p>
</div>

<style>
  .factor-config {
    overflow-x: auto;
  }

  .factor-table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--text-sm);
  }

  .factor-table th,
  .factor-table td {
    padding: var(--space-2);
    text-align: left;
    border-bottom: 1px solid var(--color-border);
  }

  .factor-table th {
    font-weight: var(--font-semibold);
    color: var(--color-text-secondary);
    background-color: var(--color-bg-tertiary);
    white-space: nowrap;
  }

  .col-factor {
    width: 60px;
  }

  .col-name {
    min-width: 150px;
  }

  .col-unit {
    width: 80px;
  }

  .col-level {
    width: 100px;
  }

  .factor-label {
    font-weight: var(--font-semibold);
    color: var(--color-accent);
    font-family: var(--font-mono);
  }

  .form-input {
    width: 100%;
    padding: var(--space-1) var(--space-2);
    font-size: var(--text-sm);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    background-color: var(--color-bg-primary);
    color: var(--color-text-primary);
    transition: border-color var(--duration-fast);
  }

  .form-input:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: var(--shadow-focus);
  }

  .unit-input {
    max-width: 80px;
  }

  .level-input {
    max-width: 100px;
    text-align: center;
    font-family: var(--font-mono);
  }

  .empty-cell {
    background-color: var(--color-bg-tertiary);
  }

  .hint {
    margin-top: var(--space-3);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }
</style>
