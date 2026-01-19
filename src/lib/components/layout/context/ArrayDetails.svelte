<script lang="ts">
  import { goto } from '$app/navigation';
  import { Button } from '$lib/components/form';
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import type { OAData } from '$lib/types';

  interface Props {
    data: OAData;
    selectedRow?: number | null;
    selectedCol?: number | null;
    onEdit?: () => void;
  }

  let { data, selectedRow = null, selectedCol = null, onEdit }: Props = $props();
  let showDeleteConfirm = $state(false);

  function formatDate(isoString?: string): string {
    if (!isoString) return 'Unknown';
    return new Date(isoString).toLocaleDateString(undefined, {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  async function duplicateArray() {
    const duplicated = await arrayStore.duplicate(data.id);
    if (duplicated) {
      goto(`/viewer/${duplicated.id}`);
    }
  }

  async function deleteArray() {
    await arrayStore.remove(data.id);
    goto('/catalogue');
  }

  function startExperiment() {
    goto(`/doe/${data.id}/config`);
  }

  function verifyArray() {
    goto(`/analysis/${data.id}`);
  }
</script>

<div class="details-content">
  <!-- Array Properties -->
  <section class="info-section">
    <h3 class="section-title">Array Properties</h3>
    <dl class="property-list">
      <div class="property">
        <dt>Runs (N)</dt>
        <dd>{data.runs}</dd>
      </div>
      <div class="property">
        <dt>Factors (k)</dt>
        <dd>{data.factors}</dd>
      </div>
      <div class="property">
        <dt>Levels (s)</dt>
        <dd>{data.levels.length === 1 ? data.levels[0] : data.levels.join(', ')}</dd>
      </div>
      <div class="property">
        <dt>Strength (t)</dt>
        <dd>{data.strength}</dd>
      </div>
      <div class="property">
        <dt>Index (&lambda;)</dt>
        <dd>{data.runs / Math.pow(data.levels[0], data.strength)}</dd>
      </div>
    </dl>
  </section>

  <!-- Metadata -->
  <section class="info-section">
    <h3 class="section-title">Metadata</h3>
    <dl class="property-list">
      <div class="property">
        <dt>Algorithm</dt>
        <dd>{data.metadata.algorithm}</dd>
      </div>
      <div class="property">
        <dt>Created</dt>
        <dd>{formatDate(data.metadata.createdAt)}</dd>
      </div>
      {#if data.metadata.notes}
        <div class="property notes-property">
          <dt>Notes</dt>
          <dd class="notes">{data.metadata.notes}</dd>
        </div>
      {/if}
    </dl>
  </section>

  <!-- Selection Details -->
  {#if selectedRow !== null || selectedCol !== null}
    <section class="info-section">
      <h3 class="section-title">Selection</h3>
      <dl class="property-list">
        {#if selectedRow !== null}
          <div class="property">
            <dt>Row</dt>
            <dd>Run {selectedRow + 1}</dd>
          </div>
          <div class="property">
            <dt>Values</dt>
            <dd class="mono">[{data.data[selectedRow].join(', ')}]</dd>
          </div>
        {/if}
        {#if selectedCol !== null}
          <div class="property">
            <dt>Column</dt>
            <dd>Factor {selectedCol + 1}</dd>
          </div>
          <div class="property">
            <dt>Levels</dt>
            <dd>{data.levels[selectedCol] ?? data.levels[0]}</dd>
          </div>
        {/if}
      </dl>
    </section>
  {/if}

  <!-- Actions -->
  <section class="info-section actions">
    <Button variant="primary" onclick={startExperiment} fullWidth>
      {#snippet children()}
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M8 1v14M1 8h14"/>
        </svg>
        Start Experiment
      {/snippet}
    </Button>

    <Button variant="secondary" onclick={verifyArray} fullWidth>
      {#snippet children()}
        Verify Array
      {/snippet}
    </Button>

    {#if onEdit}
      <Button variant="secondary" onclick={onEdit} fullWidth>
        {#snippet children()}
          Edit Metadata
        {/snippet}
      </Button>
    {/if}

    <Button variant="secondary" onclick={duplicateArray} fullWidth>
      {#snippet children()}
        Duplicate
      {/snippet}
    </Button>

    {#if showDeleteConfirm}
      <div class="delete-confirm">
        <span class="delete-warning">Delete this array?</span>
        <div class="delete-actions">
          <Button variant="ghost" size="sm" onclick={() => showDeleteConfirm = false}>
            {#snippet children()}
              Cancel
            {/snippet}
          </Button>
          <Button variant="primary" size="sm" onclick={deleteArray}>
            {#snippet children()}
              Delete
            {/snippet}
          </Button>
        </div>
      </div>
    {:else}
      <Button variant="ghost" onclick={() => showDeleteConfirm = true} fullWidth>
        {#snippet children()}
          Delete
        {/snippet}
      </Button>
    {/if}
  </section>
</div>

<style>
  .details-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .info-section {
    background-color: var(--color-bg-primary);
    border-radius: var(--radius-md);
    padding: var(--space-3);
  }

  .section-title {
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: var(--space-3);
  }

  .property-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .property {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: var(--space-2);
  }

  .property dt {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    flex-shrink: 0;
  }

  .property dd {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
    text-align: right;
  }

  .property dd.mono {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
  }

  .notes-property {
    flex-direction: column;
    gap: var(--space-1);
  }

  .notes-property dd {
    text-align: left;
    font-weight: normal;
    color: var(--color-text-secondary);
    line-height: var(--leading-relaxed);
  }

  .actions {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    background-color: transparent;
    padding: 0;
  }

  .delete-confirm {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-3);
    background-color: var(--color-bg-primary);
    border-radius: var(--radius-md);
    border: 1px solid var(--color-border);
  }

  .delete-warning {
    font-size: var(--text-sm);
    color: var(--color-text-primary);
    font-weight: var(--font-medium);
  }

  .delete-actions {
    display: flex;
    gap: var(--space-2);
  }
</style>
