<script lang="ts">
  import { Chart } from 'svelte-echarts';
  import { init, use } from 'echarts/core';
  import { BarChart } from 'echarts/charts';
  import { GridComponent, TooltipComponent, MarkLineComponent } from 'echarts/components';
  import { CanvasRenderer } from 'echarts/renderers';
  import type { EChartsOption } from 'echarts';
  import type { BalanceData } from '$lib/types';

  // Register ECharts components
  use([BarChart, GridComponent, TooltipComponent, MarkLineComponent, CanvasRenderer]);

  interface Props {
    data: BalanceData;
    factorIndex?: number;
    height?: number;
  }

  let {
    data,
    factorIndex = 0,
    height = 200,
  }: Props = $props();

  // Check if factor is balanced
  let isBalanced = $derived(data.factorBalance[factorIndex] ?? false);

  // Build chart options
  let options = $derived<EChartsOption>((() => {
    const counts = data.levelCounts[factorIndex];
    if (!counts) return {};

    const entries = Object.entries(counts).sort(([a], [b]) => Number(a) - Number(b));
    const levels = entries.map(([level]) => `Level ${level}`);
    const values = entries.map(([, count]) => count);

    return {
      tooltip: {
        trigger: 'axis',
        axisPointer: { type: 'shadow' },
        formatter: (params: unknown) => {
          const p = params as Array<{ name: string; value: number }>;
          if (p && p[0]) {
            return `${p[0].name}<br/>Count: ${p[0].value}<br/>Expected: ${data.expectedCount}`;
          }
          return '';
        },
      },
      grid: {
        left: 50,
        right: 20,
        top: 20,
        bottom: 30,
      },
      xAxis: {
        type: 'category',
        data: levels,
        axisLabel: {
          fontSize: 11,
          color: 'var(--color-text-secondary)',
        },
        axisLine: {
          lineStyle: { color: 'var(--color-border)' },
        },
      },
      yAxis: {
        type: 'value',
        axisLabel: {
          fontSize: 11,
          color: 'var(--color-text-secondary)',
        },
        splitLine: {
          lineStyle: { color: 'var(--color-border-light)' },
        },
      },
      series: [
        {
          type: 'bar',
          data: values,
          itemStyle: {
            color: 'var(--color-accent)',
            borderRadius: [4, 4, 0, 0],
          },
          markLine: {
            silent: true,
            symbol: 'none',
            lineStyle: {
              color: 'var(--color-warning)',
              type: 'dashed',
              width: 2,
            },
            data: [
              {
                yAxis: data.expectedCount,
                label: {
                  formatter: `Expected: ${data.expectedCount}`,
                  position: 'end',
                  fontSize: 10,
                  color: 'var(--color-warning)',
                },
              },
            ],
          },
        },
      ],
    };
  })());
</script>

<div class="balance-chart">
  <div class="chart-header">
    <span class="chart-title">Factor {factorIndex + 1} Level Distribution</span>
    <span class="balance-badge" class:balanced={isBalanced} class:unbalanced={!isBalanced}>
      {isBalanced ? 'Balanced' : 'Unbalanced'}
    </span>
  </div>

  <div class="chart-container" style="height: {height}px">
    <Chart {init} {options} />
  </div>
</div>

<style>
  .balance-chart {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .chart-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .chart-title {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
  }

  .balance-badge {
    padding: 2px 8px;
    border-radius: var(--radius-full);
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
  }

  .balance-badge.balanced {
    background-color: var(--color-success-subtle);
    color: var(--color-success);
  }

  .balance-badge.unbalanced {
    background-color: var(--color-warning-subtle);
    color: var(--color-text-primary);
  }

  .chart-container {
    background-color: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    overflow: hidden;
  }
</style>
