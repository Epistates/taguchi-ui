<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';

  interface Props extends HTMLButtonAttributes {
    variant?: 'primary' | 'secondary' | 'ghost' | 'danger';
    size?: 'sm' | 'md' | 'lg';
    loading?: boolean;
    fullWidth?: boolean;
    icon?: Snippet;
    children?: Snippet;
  }

  let {
    variant = 'primary',
    size = 'md',
    loading = false,
    fullWidth = false,
    disabled = false,
    icon,
    children,
    class: className = '',
    ...rest
  }: Props = $props();
</script>

<button
  class="btn btn-{variant} btn-{size} {className}"
  class:full-width={fullWidth}
  disabled={disabled || loading}
  {...rest}
>
  {#if loading}
    <svg class="spinner" width="16" height="16" viewBox="0 0 24 24" fill="none">
      <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" stroke-linecap="round" opacity="0.25"/>
      <path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="3" stroke-linecap="round"/>
    </svg>
  {:else if icon}
    <span class="btn-icon">
      {@render icon()}
    </span>
  {/if}

  {#if children}
    <span class="btn-label">
      {@render children()}
    </span>
  {/if}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-2);
    font-weight: var(--font-medium);
    line-height: 1;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all var(--duration-fast) var(--ease-default);
    white-space: nowrap;
    border: 1px solid transparent;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn.full-width {
    width: 100%;
  }

  .btn:focus-visible {
    outline: 2px solid var(--color-accent);
    outline-offset: 2px;
  }

  /* Sizes */
  .btn-sm {
    padding: var(--space-1) var(--space-2);
    font-size: var(--text-xs);
    height: 28px;
  }

  .btn-md {
    padding: var(--space-2) var(--space-4);
    font-size: var(--text-sm);
    height: 36px;
  }

  .btn-lg {
    padding: var(--space-3) var(--space-6);
    font-size: var(--text-base);
    height: 44px;
  }

  /* Variants */
  .btn-primary {
    color: var(--color-text-inverse);
    background-color: var(--color-accent);
    border-color: var(--color-accent);
  }

  .btn-primary:hover:not(:disabled) {
    background-color: var(--color-accent-hover);
    border-color: var(--color-accent-hover);
  }

  .btn-primary:active:not(:disabled) {
    background-color: var(--color-accent-active);
    border-color: var(--color-accent-active);
  }

  .btn-secondary {
    color: var(--color-text-primary);
    background-color: var(--color-bg-primary);
    border-color: var(--color-border);
  }

  .btn-secondary:hover:not(:disabled) {
    background-color: var(--color-bg-hover);
    border-color: var(--color-border-strong);
  }

  .btn-ghost {
    color: var(--color-text-secondary);
    background-color: transparent;
    border-color: transparent;
  }

  .btn-ghost:hover:not(:disabled) {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .btn-danger {
    color: var(--color-text-inverse);
    background-color: var(--color-error);
    border-color: var(--color-error);
  }

  .btn-danger:hover:not(:disabled) {
    background-color: #c82333;
    border-color: #c82333;
  }

  /* Spinner animation */
  .spinner {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .btn-icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .btn-label {
    display: flex;
    align-items: center;
  }
</style>
