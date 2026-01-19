/**
 * Global application state using Svelte 5 runes
 */

// Theme state
let _theme = $state<'light' | 'dark'>('light');

export const theme = {
  get value() { return _theme; },
  set value(v: 'light' | 'dark') {
    _theme = v;
    if (typeof document !== 'undefined') {
      document.documentElement.setAttribute('data-theme', v);
      localStorage.setItem('taguchi-theme', v);
    }
  },
  toggle() {
    this.value = _theme === 'light' ? 'dark' : 'light';
  },
  init() {
    if (typeof window === 'undefined') return;
    const stored = localStorage.getItem('taguchi-theme') as 'light' | 'dark' | null;
    if (stored) {
      _theme = stored;
    } else if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      _theme = 'dark';
    }
  }
};

// Sidebar state
let _sidebarOpen = $state(true);
let _sidebarCollapsed = $state(false);

export const sidebar = {
  get open() { return _sidebarOpen; },
  set open(v: boolean) { _sidebarOpen = v; },
  get collapsed() { return _sidebarCollapsed; },
  set collapsed(v: boolean) { _sidebarCollapsed = v; },
  toggle() { _sidebarOpen = !_sidebarOpen; },
  toggleCollapse() { _sidebarCollapsed = !_sidebarCollapsed; }
};

// Context panel state
let _contextPanelOpen = $state(true);

export const contextPanel = {
  get open() { return _contextPanelOpen; },
  set open(v: boolean) { _contextPanelOpen = v; },
  toggle() { _contextPanelOpen = !_contextPanelOpen; }
};

// Command palette state
let _commandPaletteOpen = $state(false);

export const commandPalette = {
  get open() { return _commandPaletteOpen; },
  set open(v: boolean) { _commandPaletteOpen = v; },
  toggle() { _commandPaletteOpen = !_commandPaletteOpen; }
};

// Current view context (for context panel)
export type ContextType =
  | 'none'
  | 'help'
  | 'dashboard'          // Dashboard stats and getting started
  | 'builder'            // Builder construction info and notation
  | 'expert'             // Expert mode algorithm reference
  | 'catalogue-preview'  // StandardArrayInfo preview in catalogue
  | 'array-details'      // OAData details in viewer
  | 'experiment-config'  // DOEConfig summary in analysis
  | 'algorithm-info';    // Algorithm reference

let _contextType = $state<ContextType>('none');
let _contextData = $state<unknown>(null);

export const context = {
  get type() { return _contextType; },
  get data() { return _contextData; },
  set(type: ContextType, data?: unknown) {
    _contextType = type;
    _contextData = data ?? null;
  },
  clear() {
    _contextType = 'none';
    _contextData = null;
  }
};
