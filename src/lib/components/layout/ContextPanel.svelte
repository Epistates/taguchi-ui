<script lang="ts">
  import { contextPanel, context } from '$lib/stores/app.svelte';
  import { CataloguePreview, ArrayDetails, ExperimentConfig, DashboardContext, BuilderContext, ExpertContext } from './context';
  import type { Snippet } from 'svelte';
  import type { StandardArrayInfo, OAData, DOEConfig, ConstructionOption } from '$lib/types';

  interface Props {
    title?: string;
    children?: Snippet;
  }

  let { title, children }: Props = $props();

  // Derive the title from context type if not provided
  const panelTitle = $derived(() => {
    if (title) return title;
    switch (context.type) {
      case 'dashboard': return 'Overview';
      case 'builder': return 'Reference';
      case 'expert': return 'Algorithm Reference';
      case 'catalogue-preview': return 'Preview';
      case 'array-details': return 'Array Details';
      case 'experiment-config': return 'Experiment';
      case 'help': return 'Help';
      default: return 'Details';
    }
  });
</script>

{#if contextPanel.open}
  <aside class="context-panel">
    <div class="context-header">
      <h2 class="context-title">{panelTitle()}</h2>
      <button
        class="close-btn"
        onclick={() => contextPanel.toggle()}
        aria-label="Close panel"
      >
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"/>
          <line x1="6" y1="6" x2="18" y2="18"/>
        </svg>
      </button>
    </div>

    <div class="context-content">
      {#if children}
        {@render children()}
      {:else if context.type === 'dashboard'}
        <DashboardContext />
      {:else if context.type === 'builder' && context.data}
        {@const builderData = context.data as { construction?: ConstructionOption | null; levels: number; factors: number; strength: number }}
        <BuilderContext
          construction={builderData.construction}
          levels={builderData.levels}
          factors={builderData.factors}
          strength={builderData.strength}
        />
      {:else if context.type === 'expert'}
        {@const expertData = context.data as { algorithm?: string | null } | null}
        <ExpertContext algorithm={expertData?.algorithm} />
      {:else if context.type === 'catalogue-preview' && context.data}
        <CataloguePreview data={context.data as StandardArrayInfo} />
      {:else if context.type === 'array-details' && context.data}
        {@const arrayContext = context.data as { array: OAData; selectedRow?: number | null; selectedCol?: number | null; onEdit?: () => void }}
        <ArrayDetails
          data={arrayContext.array}
          selectedRow={arrayContext.selectedRow}
          selectedCol={arrayContext.selectedCol}
          onEdit={arrayContext.onEdit}
        />
      {:else if context.type === 'experiment-config' && context.data}
        {@const configContext = context.data as { config: DOEConfig; array?: OAData | null }}
        <ExperimentConfig
          config={configContext.config}
          array={configContext.array}
        />
      {:else if context.type === 'help'}
        <div class="help-content">
          <h3 class="help-title">Quick Help</h3>
          <div class="help-section">
            <h4>Keyboard Shortcuts</h4>
            <ul class="shortcut-list">
              <li><kbd>Cmd</kbd> + <kbd>K</kbd> Command palette</li>
              <li><kbd>Cmd</kbd> + <kbd>N</kbd> New array</li>
              <li><kbd>Cmd</kbd> + <kbd>S</kbd> Save experiment</li>
              <li><kbd>Cmd</kbd> + <kbd>E</kbd> Export</li>
            </ul>
          </div>
          <div class="help-section">
            <h4>Navigation</h4>
            <ul class="shortcut-list">
              <li><kbd>G</kbd> then <kbd>C</kbd> Go to Catalogue</li>
              <li><kbd>G</kbd> then <kbd>B</kbd> Go to Builder</li>
              <li><kbd>G</kbd> then <kbd>D</kbd> Go to DOE</li>
              <li><kbd>G</kbd> then <kbd>E</kbd> Go to Expert</li>
            </ul>
          </div>
        </div>
      {:else}
        <div class="empty-state">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.3">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="16" x2="12" y2="12"/>
            <line x1="12" y1="8" x2="12.01" y2="8"/>
          </svg>
          <p class="text-muted text-sm">Select an item to view details</p>
        </div>
      {/if}
    </div>
  </aside>
{/if}

<style>
  .context-panel {
    width: var(--context-panel-width);
    background-color: var(--color-bg-secondary);
    border-left: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    height: 100vh;
    position: sticky;
    top: 0;
    overflow: hidden;
  }

  .context-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-4);
    border-bottom: 1px solid var(--color-border);
    min-height: 64px;
  }

  .context-title {
    font-size: var(--text-base);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .close-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-md);
    color: var(--color-text-muted);
    transition: all var(--duration-fast);
  }

  .close-btn:hover {
    background-color: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .context-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-4);
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--space-3);
    padding: var(--space-8) var(--space-4);
    text-align: center;
  }

  .help-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .help-title {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .help-section {
    background-color: var(--color-bg-primary);
    border-radius: var(--radius-md);
    padding: var(--space-3);
  }

  .help-section h4 {
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: var(--space-3);
  }

  .shortcut-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .shortcut-list li {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
  }

  kbd {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 2px 6px;
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    color: var(--color-text-secondary);
  }

  @media (max-width: 1152px) {
    .context-panel {
      display: none;
    }
  }
</style>
