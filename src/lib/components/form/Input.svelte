<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';

  interface Props extends Omit<HTMLInputAttributes, 'value'> {
    label?: string;
    hint?: string;
    error?: string;
    value?: string | number;
  }

  let {
    label,
    hint,
    error,
    value = $bindable(''),
    id,
    type = 'text',
    class: className = '',
    ...rest
  }: Props = $props();

  // Generate stable ID once at component creation
  const generatedId = `input-${Math.random().toString(36).slice(2, 9)}`;
  let inputId = $derived(id || generatedId);
</script>

<div class="form-field {className}" class:has-error={!!error}>
  {#if label}
    <label class="form-label" for={inputId}>
      {label}
      {#if rest.required}
        <span class="required">*</span>
      {/if}
    </label>
  {/if}

  <input
    {type}
    id={inputId}
    class="form-input"
    class:error={!!error}
    bind:value
    aria-invalid={!!error}
    aria-describedby={error ? `${inputId}-error` : hint ? `${inputId}-hint` : undefined}
    {...rest}
  />

  {#if error}
    <p class="form-error" id="{inputId}-error" role="alert">{error}</p>
  {:else if hint}
    <p class="form-hint" id="{inputId}-hint">{hint}</p>
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

  .form-input {
    width: 100%;
    padding: var(--space-2) var(--space-3);
    font-size: var(--text-base);
    color: var(--color-text-primary);
    background-color: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    transition: border-color var(--duration-fast), box-shadow var(--duration-fast);
  }

  .form-input:hover:not(:disabled) {
    border-color: var(--color-border-strong);
  }

  .form-input:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: var(--shadow-focus);
  }

  .form-input:disabled {
    background-color: var(--color-bg-tertiary);
    color: var(--color-text-disabled);
    cursor: not-allowed;
  }

  .form-input.error {
    border-color: var(--color-error);
  }

  .form-input.error:focus {
    box-shadow: 0 0 0 3px var(--color-error-subtle);
  }

  .form-input::placeholder {
    color: var(--color-text-muted);
  }

  /* Number input - hide spinners */
  .form-input[type="number"]::-webkit-outer-spin-button,
  .form-input[type="number"]::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .form-input[type="number"] {
    appearance: textfield;
    -moz-appearance: textfield;
    font-family: var(--font-mono);
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
