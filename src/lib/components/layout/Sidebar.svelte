<script lang="ts">
  import { page } from '$app/stores';
  import { sidebar, theme } from '$lib/stores/app.svelte';
  import NavSection from './NavSection.svelte';
  import type { NavSection as NavSectionType } from '$lib/types';

  const navigation: NavSectionType[] = [
    {
      id: 'main',
      title: 'Main',
      items: [
        { id: 'dashboard', label: 'Dashboard', href: '/', icon: 'home' },
        { id: 'builder', label: 'OA Builder', href: '/builder', icon: 'plus-square' },
        { id: 'catalogue', label: 'Catalogue', href: '/catalogue', icon: 'grid' },
      ]
    },
    {
      id: 'doe',
      title: 'DOE',
      items: [
        { id: 'experiments', label: 'Experiments', href: '/doe', icon: 'beaker' },
      ]
    },
    {
      id: 'advanced',
      title: 'Advanced',
      collapsible: true,
      defaultOpen: false,
      items: [
        { id: 'expert', label: 'Expert Mode', href: '/expert', icon: 'terminal' },
      ]
    }
  ];
</script>

<aside class="sidebar" class:collapsed={sidebar.collapsed} class:open={sidebar.open}>
  <div class="sidebar-header">
    <div class="logo">
      {#if !sidebar.collapsed}
        <span class="logo-text">Taguchi</span>
        <span class="logo-subtitle">DOE</span>
      {:else}
        <span class="logo-icon">T</span>
      {/if}
    </div>
    <button
      class="collapse-btn"
      onclick={() => sidebar.toggleCollapse()}
      aria-label={sidebar.collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
        {#if sidebar.collapsed}
          <path d="M6 3L11 8L6 13" stroke="currentColor" stroke-width="2" fill="none"/>
        {:else}
          <path d="M10 3L5 8L10 13" stroke="currentColor" stroke-width="2" fill="none"/>
        {/if}
      </svg>
    </button>
  </div>

  <nav class="sidebar-nav">
    {#each navigation as section}
      <NavSection {section} collapsed={sidebar.collapsed} currentPath={$page.url.pathname} />
    {/each}
  </nav>

  <div class="sidebar-footer">
    <button
      class="theme-toggle"
      onclick={() => theme.toggle()}
      aria-label={theme.value === 'light' ? 'Switch to dark mode' : 'Switch to light mode'}
    >
      {#if theme.value === 'light'}
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
        </svg>
      {:else}
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="5"/>
          <line x1="12" y1="1" x2="12" y2="3"/>
          <line x1="12" y1="21" x2="12" y2="23"/>
          <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/>
          <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
          <line x1="1" y1="12" x2="3" y2="12"/>
          <line x1="21" y1="12" x2="23" y2="12"/>
          <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/>
          <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
        </svg>
      {/if}
      {#if !sidebar.collapsed}
        <span>{theme.value === 'light' ? 'Dark Mode' : 'Light Mode'}</span>
      {/if}
    </button>

    <div class="version">
      {#if !sidebar.collapsed}
        <span class="text-muted text-xs">v0.1.0</span>
      {/if}
    </div>
  </div>
</aside>

<style>
  .sidebar {
    width: var(--sidebar-width);
    background-color: var(--color-bg-secondary);
    border-right: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    height: 100vh;
    position: sticky;
    top: 0;
    transition: width var(--duration-normal) var(--ease-default);
    overflow: hidden;
  }

  .sidebar.collapsed {
    width: var(--sidebar-width-collapsed);
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-4);
    border-bottom: 1px solid var(--color-border);
    min-height: 64px;
  }

  .logo {
    display: flex;
    align-items: baseline;
    gap: var(--space-2);
  }

  .logo-text {
    font-size: var(--text-xl);
    font-weight: var(--font-bold);
    color: var(--color-text-primary);
  }

  .logo-subtitle {
    font-size: var(--text-xs);
    font-weight: var(--font-medium);
    color: var(--color-accent);
    text-transform: uppercase;
    letter-spacing: 0.1em;
  }

  .logo-icon {
    font-size: var(--text-xl);
    font-weight: var(--font-bold);
    color: var(--color-accent);
  }

  .collapse-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-md);
    color: var(--color-text-muted);
    transition: all var(--duration-fast);
  }

  .collapse-btn:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .sidebar-nav {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-2);
  }

  .sidebar-footer {
    padding: var(--space-3);
    border-top: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .theme-toggle {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2);
    border-radius: var(--radius-md);
    color: var(--color-text-secondary);
    font-size: var(--text-sm);
    width: 100%;
    justify-content: flex-start;
    transition: all var(--duration-fast);
  }

  .theme-toggle:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .sidebar.collapsed .theme-toggle {
    justify-content: center;
  }

  .version {
    text-align: center;
  }

  /* Mobile responsiveness */
  @media (max-width: 800px) {
    .sidebar {
      position: fixed;
      left: 0;
      top: 0;
      z-index: var(--z-fixed);
      transform: translateX(-100%);
    }

    .sidebar.open {
      transform: translateX(0);
    }
  }
</style>
