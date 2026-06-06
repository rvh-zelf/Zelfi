<script lang="ts">
  import { activeView } from '$lib/stores/ui';
  import GoalsView from '$lib/components/views/GoalsView.svelte';
  import InboxView from '$lib/components/views/InboxView.svelte';
  import HistoryView from '$lib/components/views/HistoryView.svelte';
  import MemoryView from '$lib/components/views/MemoryView.svelte';
  import SettingsView from '$lib/components/views/SettingsView.svelte';
</script>

<div class="h-full overflow-hidden">
  {#if $activeView === 'goals'}
    <GoalsView />
  {:else if $activeView === 'inbox'}
    <InboxView />
  {:else if $activeView === 'history'}
    <HistoryView />
  {:else if $activeView === 'memory'}
    <MemoryView />
  {:else if $activeView === 'settings'}
    <SettingsView />
  {/if}
</div>
