<script lang="ts">
  import { Chart } from 'svelte-echarts';
  import { init, use } from 'echarts/core';
  import { HeatmapChart } from 'echarts/charts';
  import { GridComponent, TooltipComponent, VisualMapComponent } from 'echarts/components';
  import { CanvasRenderer } from 'echarts/renderers';
  import type { EChartsOption } from 'echarts';
  import type { CorrelationData } from '$lib/types';

  // Register ECharts components
  use([HeatmapChart, GridComponent, TooltipComponent, VisualMapComponent, CanvasRenderer]);

  interface Props {
    data: CorrelationData;
    size?: number;
  }

  let {
    data,
    size = 400,
  }: Props = $props();

  // Build heatmap data
  let heatmapData = $derived((() => {
    const result: [number, number, number][] = [];
    for (let i = 0; i < data.factors; i++) {
      for (let j = 0; j < data.factors; j++) {
        result.push([j, i, data.matrix[i][j]]);
      }
    }
    return result;
  })());

  // Factor labels
  let factorLabels = $derived(
    Array.from({ length: data.factors }, (_, i) => `F${i + 1}`)
  );

  // Build chart options
  let options = $derived<EChartsOption>({
    tooltip: {
      position: 'top',
      formatter: (params: unknown) => {
        const p = params as { data: [number, number, number] };
        if (p && p.data) {
          const [col, row, value] = p.data;
          return `F${row + 1} × F${col + 1}<br/>Correlation: ${value.toFixed(3)}`;
        }
        return '';
      },
    },
    grid: {
      left: 50,
      right: 80,
      top: 30,
      bottom: 50,
    },
    xAxis: {
      type: 'category',
      data: factorLabels,
      position: 'top',
      axisLabel: {
        fontSize: 11,
        color: 'var(--color-text-secondary)',
      },
      axisLine: { show: false },
      axisTick: { show: false },
      splitArea: { show: true },
    },
    yAxis: {
      type: 'category',
      data: factorLabels,
      inverse: true,
      axisLabel: {
        fontSize: 11,
        color: 'var(--color-text-secondary)',
      },
      axisLine: { show: false },
      axisTick: { show: false },
      splitArea: { show: true },
    },
    visualMap: {
      min: -1,
      max: 1,
      calculable: true,
      orient: 'vertical',
      right: 10,
      top: 'center',
      itemHeight: size * 0.6,
      textStyle: {
        fontSize: 10,
        color: 'var(--color-text-secondary)',
      },
      inRange: {
        color: ['#3b82f6', '#f3f4f6', '#f59e0b'],
      },
    },
    series: [
      {
        type: 'heatmap',
        data: heatmapData,
        label: {
          show: data.factors <= 10,
          fontSize: 10,
          formatter: (params: unknown) => {
            const p = params as { data: [number, number, number] };
            return p.data[2].toFixed(2);
          },
        },
        emphasis: {
          itemStyle: {
            shadowBlur: 10,
            shadowColor: 'rgba(0, 0, 0, 0.5)',
          },
        },
      },
    ],
  });
</script>

<div class="heatmap-chart">
  <div class="chart-container" style="width: {size}px; height: {size}px">
    <Chart {init} {options} />
  </div>

  <div class="legend-note">
    <span class="note-text">Blue = Negative correlation, Orange = Positive correlation</span>
  </div>
</div>

<style>
  .heatmap-chart {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-3);
  }

  .chart-container {
    background-color: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .legend-note {
    text-align: center;
  }

  .note-text {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }
</style>
