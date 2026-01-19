<script lang="ts">
  import { goto } from '$app/navigation';
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import { commandPalette } from '$lib/stores/app.svelte';

  interface Command {
    id: string;
    label: string;
    shortcut?: string;
    category: string;
    action: () => void;
  }

  // State - use shared store for open state
  let isOpen = $derived(commandPalette.open);
  let query = $state('');
  let selectedIndex = $state(0);
  let inputEl: HTMLInputElement;

  // Commands list
  const staticCommands: Command[] = [
    // Navigation
    { id: 'nav-home', label: 'Go to Dashboard', shortcut: 'G H', category: 'Navigation', action: () => goto('/') },
    { id: 'nav-builder', label: 'Go to Builder', shortcut: 'G B', category: 'Navigation', action: () => goto('/builder') },
    { id: 'nav-catalogue', label: 'Go to Catalogue', shortcut: 'G C', category: 'Navigation', action: () => goto('/catalogue') },
    { id: 'nav-experiments', label: 'Go to Experiments', shortcut: 'G E', category: 'Navigation', action: () => goto('/experiments') },
    { id: 'nav-expert', label: 'Go to Expert Mode', shortcut: 'G X', category: 'Navigation', action: () => goto('/expert') },

    // Quick Actions
    { id: 'action-new-array', label: 'Create New Array', shortcut: 'N', category: 'Actions', action: () => goto('/builder') },
    { id: 'action-import', label: 'Import Array', shortcut: 'I', category: 'Actions', action: () => goto('/experiments') },

    // Standard Arrays
    { id: 'array-l4', label: 'Quick: L4 (2³)', category: 'Standard Arrays', action: () => goto('/catalogue?select=L4') },
    { id: 'array-l8', label: 'Quick: L8 (2⁷)', category: 'Standard Arrays', action: () => goto('/catalogue?select=L8') },
    { id: 'array-l9', label: 'Quick: L9 (3⁴)', category: 'Standard Arrays', action: () => goto('/catalogue?select=L9') },
    { id: 'array-l16', label: 'Quick: L16 (2¹⁵)', category: 'Standard Arrays', action: () => goto('/catalogue?select=L16') },
    { id: 'array-l27', label: 'Quick: L27 (3¹³)', category: 'Standard Arrays', action: () => goto('/catalogue?select=L27') },
  ];

  // Dynamic commands based on current arrays
  let dynamicCommands = $derived((() => {
    const arrays = arrayStore.list;
    return arrays.map((oa) => ({
      id: `view-${oa.id}`,
      label: `View: ${oa.metadata.name || 'Unnamed Array'}`,
      category: 'Recent Arrays',
      action: () => goto(`/viewer/${oa.id}`),
    }));
  })());

  // All commands
  let allCommands = $derived([...staticCommands, ...dynamicCommands]);

  // Filtered commands based on query
  let filteredCommands = $derived((() => {
    if (!query.trim()) return allCommands;

    const lowerQuery = query.toLowerCase();
    return allCommands.filter(
      (cmd) =>
        cmd.label.toLowerCase().includes(lowerQuery) ||
        cmd.category.toLowerCase().includes(lowerQuery)
    );
  })());

  // Group commands by category
  let groupedCommands = $derived((() => {
    const groups: Record<string, Command[]> = {};
    for (const cmd of filteredCommands) {
      if (!groups[cmd.category]) {
        groups[cmd.category] = [];
      }
      groups[cmd.category].push(cmd);
    }
    return groups;
  })());

  // Open/close handlers
  function open() {
    commandPalette.open = true;
    query = '';
    selectedIndex = 0;
    // Focus input after DOM update
    setTimeout(() => inputEl?.focus(), 0);
  }

  function close() {
    commandPalette.open = false;
    query = '';
  }

  // Focus input when palette opens
  $effect(() => {
    if (isOpen) {
      query = '';
      selectedIndex = 0;
      setTimeout(() => inputEl?.focus(), 0);
    }
  });

  // Execute selected command
  function executeCommand(cmd: Command) {
    close();
    cmd.action();
  }

  // Execute currently selected command
  function executeSelected() {
    const cmd = filteredCommands[selectedIndex];
    if (cmd) {
      executeCommand(cmd);
    }
  }

  // Keyboard navigation
  function handleKeydown(e: KeyboardEvent) {
    if (!isOpen) return;

    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, filteredCommands.length - 1);
        break;
      case 'ArrowUp':
        e.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        break;
      case 'Enter':
        e.preventDefault();
        executeSelected();
        break;
      case 'Escape':
        e.preventDefault();
        close();
        break;
    }
  }

  // Reset selection when query changes
  $effect(() => {
    query; // Track query
    selectedIndex = 0;
  });
</script>

{#if isOpen}
  <!-- Backdrop -->
  <div
    class="backdrop"
    onclick={close}
    onkeydown={(e) => e.key === 'Escape' && close()}
    role="button"
    tabindex="-1"
  ></div>

  <!-- Palette -->
  <div class="palette" role="dialog" aria-label="Command palette">
    <div class="palette-header">
      <svg class="search-icon" width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="8" cy="8" r="6"/>
        <path d="M14 14l4 4"/>
      </svg>
      <input
        bind:this={inputEl}
        bind:value={query}
        type="text"
        class="palette-input"
        placeholder="Type a command or search..."
        onkeydown={handleKeydown}
      />
      <kbd class="shortcut-hint">Esc</kbd>
    </div>

    <div class="palette-content">
      {#if filteredCommands.length === 0}
        <div class="empty-results">
          <p>No commands found for "{query}"</p>
        </div>
      {:else}
        {#each Object.entries(groupedCommands) as [category, commands]}
          <div class="command-group">
            <div class="group-label">{category}</div>
            {#each commands as cmd, i}
              {@const globalIndex = filteredCommands.indexOf(cmd)}
              <button
                class="command-item"
                class:selected={globalIndex === selectedIndex}
                onclick={() => executeCommand(cmd)}
                onmouseenter={() => selectedIndex = globalIndex}
              >
                <span class="command-label">{cmd.label}</span>
                {#if cmd.shortcut}
                  <kbd class="command-shortcut">{cmd.shortcut}</kbd>
                {/if}
              </button>
            {/each}
          </div>
        {/each}
      {/if}
    </div>

    <div class="palette-footer">
      <span class="hint"><kbd>↑↓</kbd> Navigate</span>
      <span class="hint"><kbd>↵</kbd> Select</span>
      <span class="hint"><kbd>Esc</kbd> Close</span>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(2px);
    z-index: 100;
  }

  .palette {
    position: fixed;
    top: 20%;
    left: 50%;
    transform: translateX(-50%);
    width: 90%;
    max-width: 560px;
    background-color: var(--color-bg-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-xl);
    z-index: 101;
    overflow: hidden;
    animation: slideDown 0.15s ease-out;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0);
    }
  }

  .palette-header {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-4);
    border-bottom: 1px solid var(--color-border);
  }

  .search-icon {
    flex-shrink: 0;
    color: var(--color-text-muted);
  }

  .palette-input {
    flex: 1;
    border: none;
    background: none;
    font-size: var(--text-base);
    color: var(--color-text-primary);
    outline: none;
  }

  .palette-input::placeholder {
    color: var(--color-text-muted);
  }

  .shortcut-hint {
    padding: 2px 6px;
    font-size: var(--text-xs);
    font-family: var(--font-mono);
    color: var(--color-text-muted);
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
  }

  .palette-content {
    max-height: 400px;
    overflow-y: auto;
    padding: var(--space-2);
  }

  .empty-results {
    padding: var(--space-8);
    text-align: center;
    color: var(--color-text-muted);
  }

  .command-group {
    margin-bottom: var(--space-2);
  }

  .group-label {
    padding: var(--space-2) var(--space-3);
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .command-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: var(--space-2) var(--space-3);
    border: none;
    background: none;
    border-radius: var(--radius-md);
    cursor: pointer;
    text-align: left;
    transition: background-color var(--duration-fast);
  }

  .command-item:hover,
  .command-item.selected {
    background-color: var(--color-bg-secondary);
  }

  .command-item.selected {
    background-color: var(--color-accent-subtle);
  }

  .command-label {
    font-size: var(--text-sm);
    color: var(--color-text-primary);
  }

  .command-shortcut {
    padding: 2px 6px;
    font-size: var(--text-xs);
    font-family: var(--font-mono);
    color: var(--color-text-muted);
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border-light);
    border-radius: var(--radius-sm);
  }

  .palette-footer {
    display: flex;
    gap: var(--space-4);
    padding: var(--space-2) var(--space-4);
    border-top: 1px solid var(--color-border);
    background-color: var(--color-bg-secondary);
  }

  .hint {
    display: flex;
    align-items: center;
    gap: var(--space-1);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .hint kbd {
    padding: 1px 4px;
    font-family: var(--font-mono);
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border-light);
    border-radius: 2px;
  }
</style>
