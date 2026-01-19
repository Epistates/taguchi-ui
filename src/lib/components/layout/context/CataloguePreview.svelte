<script lang="ts">
  import { goto } from '$app/navigation';
  import { Button } from '$lib/components/form';
  import { catalogueStore } from '$lib/stores/catalogue.svelte';
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import type { StandardArrayInfo } from '$lib/types';

  interface Props {
    data: StandardArrayInfo;
  }

  let { data }: Props = $props();
  let loading = $state(false);

  async function useArray() {
    loading = true;
    try {
      const oa = await catalogueStore.getArray(data.name);
      await arrayStore.add(oa);
      arrayStore.setCurrent(oa.id);
      goto(`/viewer/${oa.id}`);
    } catch (e) {
      console.error('Failed to load array:', e);
    } finally {
      loading = false;
    }
  }
</script>

<div class="preview-content">
  <div class="preview-header">
    <h3 class="preview-name">{data.name}</h3>
    <span class="preview-notation">
      OA({data.runs}, {data.factors}, {data.levels}, {data.strength})
    </span>
  </div>

  <p class="preview-desc">{data.description}</p>

  <div class="preview-stats">
    <div class="stat-row">
      <span class="stat-label">Runs (N)</span>
      <span class="stat-value">{data.runs}</span>
    </div>
    <div class="stat-row">
      <span class="stat-label">Factors (k)</span>
      <span class="stat-value">{data.factors}</span>
    </div>
    <div class="stat-row">
      <span class="stat-label">Levels (s)</span>
      <span class="stat-value">{data.levels}</span>
    </div>
    <div class="stat-row">
      <span class="stat-label">Strength (t)</span>
      <span class="stat-value">{data.strength}</span>
    </div>
    <div class="stat-row">
      <span class="stat-label">Index (&lambda;)</span>
      <span class="stat-value">
        {data.runs / Math.pow(data.levels, data.strength)}
      </span>
    </div>
  </div>

  <div class="preview-actions">
    <Button variant="primary" {loading} onclick={useArray} fullWidth>
      {#snippet children()}
        Use This Array
      {/snippet}
    </Button>
  </div>

  <p class="hint">Double-click card to use directly</p>
</div>

<style>
  .preview-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .preview-header {
    text-align: center;
  }

  .preview-name {
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
    color: var(--color-accent);
    margin-bottom: var(--space-1);
  }

  .preview-notation {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--color-text-muted);
  }

  .preview-desc {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    text-align: center;
    line-height: var(--leading-relaxed);
  }

  .preview-stats {
    background-color: var(--color-bg-primary);
    border-radius: var(--radius-md);
    padding: var(--space-3);
  }

  .stat-row {
    display: flex;
    justify-content: space-between;
    padding: var(--space-2) 0;
    border-bottom: 1px solid var(--color-border-light);
  }

  .stat-row:last-child {
    border-bottom: none;
  }

  .stat-label {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
  }

  .stat-value {
    font-family: var(--font-mono);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .preview-actions {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .hint {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    text-align: center;
  }
</style>
