<script lang="ts">
  import '../app.css';
  import { goto } from '$app/navigation';
  import { Sidebar, ContextPanel } from '$lib/components/layout';
  import { CommandPalette } from '$lib/components/overlay';
  import { theme, sidebar, contextPanel, commandPalette } from '$lib/stores/app.svelte';
  import { onMount } from 'svelte';
  import type { Snippet } from 'svelte';

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();

  // Track 'g' key for vim-style navigation
  let awaitingNavKey = false;
  let navKeyTimeout: ReturnType<typeof setTimeout> | null = null;

  onMount(() => {
    // Initialize theme from localStorage/system preference
    theme.init();

    // Set up global keyboard shortcuts
    const handleKeydown = (e: KeyboardEvent) => {
      // Skip if in input/textarea
      const target = e.target as HTMLElement;
      const isInputFocused = target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable;

      // Cmd/Ctrl + K - Command palette (works everywhere)
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        commandPalette.toggle();
        return;
      }

      // Cmd/Ctrl + B - Toggle sidebar (works everywhere)
      if ((e.metaKey || e.ctrlKey) && e.key === 'b') {
        e.preventDefault();
        sidebar.toggleCollapse();
        return;
      }

      // Escape - Close modals/palettes
      if (e.key === 'Escape') {
        if (commandPalette.open) {
          commandPalette.open = false;
        }
        return;
      }

      // Skip remaining shortcuts if focused on input or command palette is open
      if (isInputFocused || commandPalette.open) return;

      // Vim-style navigation: G + key
      if (awaitingNavKey) {
        awaitingNavKey = false;
        if (navKeyTimeout) clearTimeout(navKeyTimeout);

        switch (e.key.toLowerCase()) {
          case 'h': goto('/'); break;
          case 'b': goto('/builder'); break;
          case 'c': goto('/catalogue'); break;
          case 'e': goto('/experiments'); break;
          case 'x': goto('/expert'); break;
          case 'a': goto('/analysis'); break;
        }
        return;
      }

      // Start vim navigation sequence
      if (e.key === 'g' && !e.metaKey && !e.ctrlKey && !e.altKey) {
        awaitingNavKey = true;
        navKeyTimeout = setTimeout(() => {
          awaitingNavKey = false;
        }, 1000); // 1 second timeout
        return;
      }

      // Single-key shortcuts
      if (e.key === '?' && !e.shiftKey) {
        // Show keyboard shortcuts help (open command palette)
        commandPalette.open = true;
      }
    };

    document.addEventListener('keydown', handleKeydown);

    return () => {
      document.removeEventListener('keydown', handleKeydown);
      if (navKeyTimeout) clearTimeout(navKeyTimeout);
    };
  });
</script>

<div
  class="app-layout"
  class:sidebar-collapsed={sidebar.collapsed}
  class:context-hidden={!contextPanel.open}
>
  <Sidebar />

  <main class="main-content">
    {@render children()}
  </main>

  <ContextPanel />
</div>

<!-- Mobile sidebar overlay -->
{#if sidebar.open}
  <button
    class="sidebar-overlay"
    onclick={() => sidebar.open = false}
    aria-label="Close sidebar"
  ></button>
{/if}

<!-- Global command palette -->
<CommandPalette />

<style>
  .sidebar-overlay {
    display: none;
  }

  @media (max-width: 800px) {
    .sidebar-overlay {
      display: block;
      position: fixed;
      inset: 0;
      background-color: rgba(0, 0, 0, 0.5);
      z-index: calc(var(--z-fixed) - 1);
    }
  }
</style>
