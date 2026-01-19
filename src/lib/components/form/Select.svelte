<script lang="ts">
  import type { HTMLSelectAttributes } from 'svelte/elements';

  interface Option {
    value: string | number;
    label: string;
    disabled?: boolean;
  }

  interface Props extends Omit<HTMLSelectAttributes, 'value'> {
    label?: string;
    hint?: string;
    error?: string;
    value?: string | number;
    options: Option[];
    placeholder?: string;
  }

  let {
    label,
    hint,
    error,
    value = $bindable(''),
    options,
    placeholder,
    id,
    class: className = '',
    ...rest
  }: Props = $props();

  // Generate stable ID once at component creation
  const generatedId = `select-${Math.random().toString(36).slice(2, 9)}`;
  let selectId = $derived(id || generatedId);
</script>

<div class="form-field {className}" class:has-error={!!error}>
  {#if label}
    <label class="form-label" for={selectId}>
      {label}
      {#if rest.required}
        <span class="required">*</span>
      {/if}
    </label>
  {/if}

  <div class="select-wrapper">
    <select
      id={selectId}
      class="form-select"
      class:error={!!error}
      class:has-placeholder={!value && placeholder}
      bind:value
      aria-invalid={!!error}
      aria-describedby={error ? `${selectId}-error` : hint ? `${selectId}-hint` : undefined}
      {...rest}
    >
      {#if placeholder}
        <option value="" disabled>{placeholder}</option>
      {/if}
      {#each options as option}
        <option value={option.value} disabled={option.disabled}>
          {option.label}
        </option>
      {/each}
    </select>
    <svg class="select-icon" width="16" height="16" viewBox="0 0 16 16">
      <path d="M4 6L8 10L12 6" stroke="currentColor" stroke-width="1.5" fill="none" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  </div>

  {#if error}
    <p class="form-error" id="{selectId}-error" role="alert">{error}</p>
  {:else if hint}
    <p class="form-hint" id="{selectId}-hint">{hint}</p>
  {/if}
</div>

<style>
  .form-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .form-label {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-secondary);
  }

  .required {
    color: var(--color-error);
    margin-left: 2px;
  }

  .select-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .form-select {
    width: 100%;
    padding: var(--space-2) var(--space-8) var(--space-2) var(--space-3);
    font-size: var(--text-base);
    color: var(--color-text-primary);
    background-color: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    appearance: none;
    cursor: pointer;
    transition: border-color var(--duration-fast), box-shadow var(--duration-fast);
  }

  .form-select:hover:not(:disabled) {
    border-color: var(--color-border-strong);
  }

  .form-select:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: var(--shadow-focus);
  }

  .form-select:disabled {
    background-color: var(--color-bg-tertiary);
    color: var(--color-text-disabled);
    cursor: not-allowed;
  }

  .form-select.error {
    border-color: var(--color-error);
  }

  .form-select.error:focus {
    box-shadow: 0 0 0 3px var(--color-error-subtle);
  }

  .form-select.has-placeholder {
    color: var(--color-text-muted);
  }

  .select-icon {
    position: absolute;
    right: var(--space-3);
    pointer-events: none;
    color: var(--color-text-muted);
  }

  .form-hint {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .form-error {
    font-size: var(--text-xs);
    color: var(--color-error);
  }
</style>
