<script lang="ts">
  import '../app.css';
  import { activeView, lmBaseUrl, lmConnected, inboxUnread } from '$lib/stores/ui';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { Zap, Inbox, Clock, Brain, Settings2 } from 'lucide-svelte';

  let { children } = $props();

  type NavItem = {
    id: import('$lib/stores/ui').View;
    label: string;
    icon: typeof Zap;
  };

  const navItems: NavItem[] = [
    { id: 'goals',   label: 'Goals',   icon: Zap },
    { id: 'inbox',   label: 'Inbox',   icon: Inbox },
    { id: 'history', label: 'History', icon: Clock },
    { id: 'memory',  label: 'Memory',  icon: Brain },
  ];

  async function checkLmStudio() {
    try {
      $lmConnected = await invoke<boolean>('check_lm_studio', { baseUrl: $lmBaseUrl });
    } catch {
      $lmConnected = false;
    }
  }

  onMount(() => {
    checkLmStudio();
    const interval = setInterval(checkLmStudio, 10_000);
    return () => clearInterval(interval);
  });

  // Persist URL to localStorage when it changes and re-check connection
  $effect(() => {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('zelfi_lm_url', $lmBaseUrl);
    }
    checkLmStudio();
  });
</script>

<div class="flex flex-col h-screen bg-base-100 text-base-content overflow-hidden">

  <!-- Topbar -->
  <header class="flex items-center justify-between px-4 h-12 flex-shrink-0 bg-base-200 border-b border-base-300">
    <!-- Wordmark -->
    <span class="text-primary font-semibold text-lg tracking-tight">Zelfi</span>

    <!-- LM Studio status row -->
    <div class="flex items-center gap-2">
      <!-- Status dot -->
      <span
        class="w-2 h-2 rounded-full flex-shrink-0 {$lmConnected ? 'bg-success' : 'bg-error'}"
        title={$lmConnected ? 'LM Studio connected' : 'LM Studio not found'}
      ></span>

      <!-- URL input -->
      <input
        type="text"
        class="input input-ghost input-xs w-56 font-mono text-xs"
        placeholder="http://127.0.0.1:1234"
        bind:value={$lmBaseUrl}
        aria-label="LM Studio base URL"
      />
    </div>
  </header>

  <!-- Body: sidebar + content -->
  <div class="flex flex-1 overflow-hidden">

    <!-- Sidebar -->
    <nav class="flex flex-col items-center w-14 flex-shrink-0 bg-base-200 border-r border-base-300 py-2">

      <!-- Main nav items -->
      {#each navItems as item}
        <div class="tooltip tooltip-right" data-tip={item.label}>
          <button
            class="btn btn-ghost btn-square w-11 h-11 relative {$activeView === item.id ? 'bg-base-300 text-primary' : ''}"
            onclick={() => ($activeView = item.id)}
            aria-label={item.label}
          >
            <item.icon size={20} />

            <!-- Inbox unread badge -->
            {#if item.id === 'inbox' && $inboxUnread > 0}
              <span class="badge badge-error badge-xs absolute top-1 right-1 pointer-events-none">
                {$inboxUnread}
              </span>
            {/if}
          </button>
        </div>
      {/each}

      <!-- Settings pinned to bottom -->
      <div class="tooltip tooltip-right mt-auto" data-tip="Settings">
        <button
          class="btn btn-ghost btn-square w-11 h-11 {$activeView === 'settings' ? 'bg-base-300 text-primary' : ''}"
          onclick={() => ($activeView = 'settings')}
          aria-label="Settings"
        >
          <Settings2 size={20} />
        </button>
      </div>

    </nav>

    <!-- Main content area -->
    <main class="flex-1 overflow-hidden">
      {@render children()}
    </main>

  </div>
</div>
