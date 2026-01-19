<script lang="ts">
  import type { ResponseVariable, OptimizationType } from '$lib/types';

  interface Props {
    response: ResponseVariable;
    onUpdate: (updates: Partial<ResponseVariable>) => void;
  }

  let { response, onUpdate }: Props = $props();

  function handleOptTypeChange(type: OptimizationType) {
    const updates: Partial<ResponseVariable> = { optimizationType: type };
    // Clear target value if not nominal-is-best
    if (type !== 'nominal-is-best') {
      updates.targetValue = undefined;
    }
    onUpdate(updates);
  }
</script>

<div class="response-config">
  <div class="form-row">
    <div class="form-group">
      <label class="form-label" for="response-name">Response Name</label>
      <input
        id="response-name"
        type="text"
        class="form-input"
        value={response.name}
        placeholder="e.g., Yield, Strength, Quality"
        oninput={(e) => onUpdate({ name: e.currentTarget.value })}
      />
    </div>

    <div class="form-group unit-group">
      <label class="form-label" for="response-unit">Unit</label>
      <input
        id="response-unit"
        type="text"
        class="form-input"
        value={response.unit ?? ''}
        placeholder="e.g., %, MPa, mm"
        oninput={(e) => onUpdate({ unit: e.currentTarget.value || undefined })}
      />
    </div>
  </div>

  <div class="form-group">
    <label class="form-label">Optimization Goal</label>
    <div class="radio-group">
      <label class="radio-option">
        <input
          type="radio"
          name="opt-type"
          checked={response.optimizationType === 'larger-is-better'}
          onchange={() => handleOptTypeChange('larger-is-better')}
        />
        <span class="radio-label">
          <span class="radio-title">Larger is Better</span>
          <span class="radio-desc">Maximize the response (e.g., yield, strength)</span>
        </span>
      </label>

      <label class="radio-option">
        <input
          type="radio"
          name="opt-type"
          checked={response.optimizationType === 'smaller-is-better'}
          onchange={() => handleOptTypeChange('smaller-is-better')}
        />
        <span class="radio-label">
          <span class="radio-title">Smaller is Better</span>
          <span class="radio-desc">Minimize the response (e.g., defects, cost)</span>
        </span>
      </label>

      <label class="radio-option">
        <input
          type="radio"
          name="opt-type"
          checked={response.optimizationType === 'nominal-is-best'}
          onchange={() => handleOptTypeChange('nominal-is-best')}
        />
        <span class="radio-label">
          <span class="radio-title">Nominal is Best</span>
          <span class="radio-desc">Target a specific value (e.g., dimension, concentration)</span>
        </span>
      </label>
    </div>
  </div>

  {#if response.optimizationType === 'nominal-is-best'}
    <div class="form-group target-group">
      <label class="form-label" for="target-value">Target Value</label>
      <input
        id="target-value"
        type="number"
        step="any"
        class="form-input target-input"
        value={response.targetValue ?? ''}
        placeholder="Enter target value"
        oninput={(e) => {
          const value = parseFloat(e.currentTarget.value);
          onUpdate({ targetValue: isNaN(value) ? undefined : value });
        }}
      />
      {#if response.unit}
        <span class="target-unit">{response.unit}</span>
      {/if}
    </div>
  {/if}
</div>

<style>
  .response-config {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .form-row {
    display: flex;
    gap: var(--space-4);
  }

  .form-group {
    flex: 1;
  }

  .unit-group {
    flex: 0 0 120px;
  }

  .form-label {
    display: block;
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-secondary);
    margin-bottom: var(--space-1);
  }

  .form-input {
    width: 100%;
    padding: var(--space-2) var(--space-3);
    font-size: var(--text-base);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-bg-primary);
    color: var(--color-text-primary);
    transition: border-color var(--duration-fast);
  }

  .form-input:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: var(--shadow-focus);
  }

  .radio-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .radio-option {
    display: flex;
    align-items: flex-start;
    gap: var(--space-3);
    padding: var(--space-3);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all var(--duration-fast);
  }

  .radio-option:hover {
    background-color: var(--color-bg-hover);
  }

  .radio-option:has(input:checked) {
    border-color: var(--color-accent);
    background-color: var(--color-accent-subtle);
  }

  .radio-option input {
    margin-top: 2px;
  }

  .radio-label {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .radio-title {
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
  }

  .radio-desc {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .target-group {
    display: flex;
    align-items: flex-end;
    gap: var(--space-2);
    flex-wrap: wrap;
  }

  .target-group .form-label {
    width: 100%;
  }

  .target-input {
    width: 200px;
    flex: none;
  }

  .target-unit {
    color: var(--color-text-muted);
    font-size: var(--text-sm);
    padding-bottom: var(--space-2);
  }
</style>
