<script lang="ts">
  import { Inbox } from 'lucide-svelte';

  type InboxItem = {
    id: string;
    type: 'NEEDS_INPUT' | 'REVIEW' | 'FYI' | 'ACKNOWLEDGED';
    title: string;
    body: string;
    createdAt: Date;
    acknowledgedAt?: Date;
  };

  let items = $state<InboxItem[]>([]);

  const typeConfig: Record<
    InboxItem['type'],
    { border: string; badge: string; label: string }
  > = {
    NEEDS_INPUT: {
      border: 'border-error',
      badge: 'badge-error',
      label: 'NEEDS INPUT',
    },
    REVIEW: {
      border: 'border-warning',
      badge: 'badge-warning',
      label: 'REVIEW',
    },
    FYI: {
      border: 'border-info',
      badge: 'badge-info',
      label: 'FYI',
    },
    ACKNOWLEDGED: {
      border: 'border-base-300',
      badge: 'badge-ghost',
      label: 'DONE',
    },
  };

  function relativeTime(date: Date): string {
    const diff = Math.floor((Date.now() - date.getTime()) / 1000);
    if (diff < 60) return 'just now';
    if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
    if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
    return `${Math.floor(diff / 86400)}d ago`;
  }

  const unreadCount = $derived(
    items.filter((i) => i.type !== 'ACKNOWLEDGED').length
  );

  function markAllReviewed() {
    items = items.map((i) => ({
      ...i,
      type: 'ACKNOWLEDGED' as const,
      acknowledgedAt: new Date(),
    }));
  }
</script>

{#if items.length === 0}
  <!-- Empty state -->
  <div class="flex flex-col items-center justify-center h-full gap-3">
    <Inbox size={40} class="text-primary" />
    <p class="text-sm text-base-content/50">Nothing needs your attention right now.</p>
  </div>
{:else}
  <!-- Header row -->
  <div class="flex items-center justify-between px-4 pt-4 pb-2">
    <div class="flex items-center gap-2">
      <span class="text-lg font-semibold">Inbox</span>
      {#if unreadCount > 0}
        <span class="badge badge-error badge-sm">{unreadCount} unread</span>
      {/if}
    </div>
    <button class="btn btn-ghost btn-xs" onclick={markAllReviewed}>
      Mark all as reviewed
    </button>
  </div>

  <!-- Item list -->
  <div class="overflow-y-auto pb-4">
    {#each items as item (item.id)}
      {@const cfg = typeConfig[item.type]}
      <div class="card bg-base-200 mb-3 mx-4 border-l-4 {cfg.border}">
        <div class="card-body p-4">
          <div class="flex items-start justify-between">
            <div class="flex items-center gap-2">
              <span class="badge badge-sm {cfg.badge}">{cfg.label}</span>
              <span class="text-xs text-base-content/50">{relativeTime(item.createdAt)}</span>
            </div>
            <div class="flex gap-2">
              {#if item.type === 'NEEDS_INPUT'}
                <button class="btn btn-primary btn-xs">Resume task</button>
                <button class="btn btn-ghost btn-xs">Dismiss</button>
              {:else if item.type === 'REVIEW'}
                <button class="btn btn-secondary btn-xs">View log</button>
                <button class="btn btn-ghost btn-xs">Dismiss</button>
              {:else if item.type === 'FYI'}
                <button class="btn btn-info btn-xs">Remember it</button>
                <button class="btn btn-ghost btn-xs">Dismiss</button>
              {:else if item.type === 'ACKNOWLEDGED'}
                <button class="btn btn-ghost btn-xs">View</button>
              {/if}
            </div>
          </div>
          <p class="text-sm font-medium mt-1">{item.title}</p>
          <p class="text-xs text-base-content/60 mt-1 line-clamp-2">{item.body}</p>
        </div>
      </div>
    {/each}
  </div>
{/if}
