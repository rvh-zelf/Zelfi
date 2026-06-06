<script lang="ts">
  import { activeView } from '$lib/stores/ui';
  import GoalsView from '$lib/components/views/GoalsView.svelte';
  // Other views imported below as they are built
</script>

{#if $activeView === 'goals'}
  <GoalsView />
{:else}
  <div class="flex items-center justify-center h-full text-base-content/30 text-sm">
    {$activeView} — coming soon
  </div>
{/if}
