<script lang="ts">
  import type { NavItem } from '$lib/types';

  interface Props {
    item: NavItem;
    collapsed: boolean;
    active: boolean;
  }

  let { item, collapsed, active }: Props = $props();

  // Icon mapping - using simple SVG icons
  const icons: Record<string, string> = {
    'home': 'M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z M9 22V12h6v10',
    'plus-square': 'M3 3h18v18H3z M12 8v8 M8 12h8',
    'grid': 'M3 3h7v7H3z M14 3h7v7h-7z M14 14h7v7h-7z M3 14h7v7H3z',
    'folder': 'M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z',
    'clock': 'M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20z M12 6v6l4 2',
    'terminal': 'M4 17l6-6-6-6 M12 19h8',
    'settings': 'M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z',
  };
</script>

<a
  href={item.href}
  class="nav-item"
  class:active
  class:collapsed
  aria-current={active ? 'page' : undefined}
  title={collapsed ? item.label : undefined}
>
  {#if item.icon && icons[item.icon]}
    <svg
      class="nav-icon"
      width="18"
      height="18"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d={icons[item.icon]} />
    </svg>
  {/if}

  {#if !collapsed}
    <span class="nav-label">{item.label}</span>

    {#if item.badge}
      <span class="nav-badge">{item.badge}</span>
    {/if}
  {/if}
</a>

<style>
  .nav-item {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-2) var(--space-3);
    border-radius: var(--radius-md);
    color: var(--color-text-secondary);
    text-decoration: none;
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    transition: all var(--duration-fast);
  }

  .nav-item:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
    text-decoration: none;
  }

  .nav-item.active {
    background-color: var(--color-accent-subtle);
    color: var(--color-accent);
  }

  .nav-item.active:hover {
    background-color: var(--color-accent-subtle);
  }

  .nav-item.collapsed {
    justify-content: center;
    padding: var(--space-2);
  }

  .nav-icon {
    flex-shrink: 0;
  }

  .nav-label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .nav-badge {
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
    padding: 2px 6px;
    border-radius: var(--radius-full);
    background-color: var(--color-accent);
    color: var(--color-text-inverse);
  }
</style>
