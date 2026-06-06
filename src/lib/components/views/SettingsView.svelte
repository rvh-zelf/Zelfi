<script lang="ts">
  import { lmBaseUrl, lmConnected } from '$lib/stores/ui';

  const VERSION = '2026.06.06.10';
</script>

<div class="p-6 max-w-lg">
  <h2 class="text-lg font-semibold mb-6">Settings</h2>

  <div class="form-control mb-4">
    <label class="label">
      <span class="label-text">LM Studio URL</span>
      <span class="label-text-alt text-base-content/50">The local server address</span>
    </label>
    <input
      type="text"
      class="input input-bordered input-sm font-mono"
      bind:value={$lmBaseUrl}
      placeholder="http://127.0.0.1:1234"
    />
    <label class="label">
      <span class="label-text-alt {$lmConnected ? 'text-success' : 'text-error'}">
        {$lmConnected ? '● Connected' : '● Not connected'}
      </span>
    </label>
  </div>

  <div class="divider">About</div>
  <p class="text-xs text-base-content/50">Zelfi · {VERSION} · Local-first autonomous AI agent</p>
</div>
