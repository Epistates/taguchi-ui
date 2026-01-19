<script lang="ts">
  import type { NavSection } from '$lib/types';
  import NavItem from './NavItem.svelte';

  interface Props {
    section: NavSection;
    collapsed: boolean;
    currentPath: string;
  }

  let { section, collapsed, currentPath }: Props = $props();

  // Use $derived to react to section changes
  let defaultOpen = $derived(section.defaultOpen ?? true);
  let isOpen = $state(true);

  // Initialize open state when section changes
  $effect(() => {
    isOpen = defaultOpen;
  });
</script>

<div class="nav-section">
  {#if section.collapsible && !collapsed}
    <button
      class="section-header collapsible"
      onclick={() => isOpen = !isOpen}
      aria-expanded={isOpen}
    >
      <span class="section-title">{section.title}</span>
      <svg
        class="chevron"
        class:rotated={isOpen}
        width="12"
        height="12"
        viewBox="0 0 12 12"
      >
        <path d="M3 4.5L6 7.5L9 4.5" stroke="currentColor" stroke-width="1.5" fill="none"/>
      </svg>
    </button>
  {:else if !collapsed}
    <div class="section-header">
      <span class="section-title">{section.title}</span>
    </div>
  {/if}

  {#if !section.collapsible || isOpen || collapsed}
    <ul class="nav-items" role="list">
      {#each section.items as item}
        <li>
          <NavItem {item} {collapsed} active={currentPath === item.href} />
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .nav-section {
    margin-bottom: var(--space-4);
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-1) var(--space-2);
    margin-bottom: var(--space-1);
  }

  .section-header.collapsible {
    cursor: pointer;
    border-radius: var(--radius-sm);
    transition: background-color var(--duration-fast);
    width: 100%;
  }

  .section-header.collapsible:hover {
    background-color: var(--color-bg-hover);
  }

  .section-title {
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .chevron {
    color: var(--color-text-muted);
    transition: transform var(--duration-fast);
  }

  .chevron.rotated {
    transform: rotate(180deg);
  }

  .nav-items {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }
</style>
