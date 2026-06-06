<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy, tick } from 'svelte';

  // LM Studio base URL — stored in localStorage, editable in the UI
  let lmBaseUrl = $state(
    typeof localStorage !== 'undefined'
      ? (localStorage.getItem('zelfi_lm_url') ?? 'http://127.0.0.1:1234')
      : 'http://127.0.0.1:1234'
  );

  // LM Studio connection status
  let lmConnected = $state(false);
  let checking = $state(true);

  // Chat state
  let prompt = $state('');
  let streaming = $state(false);
  let steps: Array<{ text: string; complete: boolean; error?: string }> = $state([]);
  let logEl: HTMLElement;

  // Tauri event unlisten callbacks
  let unlistenToken: (() => void) | null = null;
  let unlistenDone: (() => void) | null = null;
  let unlistenError: (() => void) | null = null;

  async function checkLmStudio() {
    checking = true;
    try {
      lmConnected = await invoke<boolean>('check_lm_studio', { baseUrl: lmBaseUrl });
    } catch {
      lmConnected = false;
    } finally {
      checking = false;
    }
  }

  // Persist URL to localStorage and re-check connection whenever URL changes
  $effect(() => {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('zelfi_lm_url', lmBaseUrl);
    }
    checkLmStudio();
  });

  async function scrollToBottom() {
    await tick();
    if (logEl) {
      logEl.scrollTop = logEl.scrollHeight;
    }
  }

  async function sendMessage() {
    if (!prompt.trim() || streaming) return;

    const userPrompt = prompt.trim();
    prompt = '';
    streaming = true;

    // Add a new step
    steps.push({ text: '', complete: false });
    const stepIdx = steps.length - 1;
    await scrollToBottom();

    // Set up event listeners
    unlistenToken = await listen<{ token: string }>('zelfi://token', (event) => {
      steps[stepIdx].text += event.payload.token;
      scrollToBottom();
    });

    unlistenDone = await listen('zelfi://done', () => {
      steps[stepIdx].complete = true;
      streaming = false;
      cleanup();
    });

    unlistenError = await listen<{ message: string }>('zelfi://error', (event) => {
      steps[stepIdx].error = event.payload.message;
      steps[stepIdx].complete = true;
      streaming = false;
      cleanup();
    });

    try {
      await invoke('send_message', { prompt: userPrompt, baseUrl: lmBaseUrl });
    } catch (err) {
      steps[stepIdx].error = String(err);
      steps[stepIdx].complete = true;
      streaming = false;
      cleanup();
    }
  }

  function cleanup() {
    unlistenToken?.();
    unlistenDone?.();
    unlistenError?.();
    unlistenToken = null;
    unlistenDone = null;
    unlistenError = null;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
      sendMessage();
    }
  }

  onMount(() => {
    // Recheck every 10 seconds so the indicator stays live
    const interval = setInterval(checkLmStudio, 10_000);
    return () => clearInterval(interval);
  });

  onDestroy(() => {
    cleanup();
  });
</script>

<div class="min-h-screen bg-base-100 flex flex-col" data-theme="dark">
  <!-- Header -->
  <header class="navbar bg-base-200 border-b border-base-300 px-4 py-2 flex-shrink-0">
    <div class="flex-1">
      <span class="text-2xl font-bold tracking-tight text-primary">Zelfi</span>
      <span class="ml-2 text-xs text-base-content/50 italic">Your local AI agent</span>
    </div>
    <div class="flex-none flex items-center gap-3">
      <!-- LM Studio URL input -->
      <div class="flex items-center gap-1.5">
        <label for="lm-url" class="text-xs text-base-content/50 whitespace-nowrap">LM Studio URL</label>
        <input
          id="lm-url"
          type="text"
          class="input input-bordered input-sm w-64 text-xs"
          placeholder="http://127.0.0.1:1234"
          bind:value={lmBaseUrl}
        />
      </div>
      <!-- LM Studio status indicator -->
      <div class="flex items-center gap-1.5" title={lmConnected ? 'LM Studio connected' : 'LM Studio not found'}>
        {#if checking}
          <span class="loading loading-ring loading-xs text-warning"></span>
          <span class="text-xs text-base-content/60">Checking…</span>
        {:else if lmConnected}
          <span class="w-2.5 h-2.5 rounded-full bg-success inline-block"></span>
          <span class="text-xs text-success">LM Studio connected</span>
        {:else}
          <span class="w-2.5 h-2.5 rounded-full bg-error inline-block"></span>
          <span class="text-xs text-error">LM Studio not found</span>
        {/if}
      </div>
    </div>
  </header>

  <!-- Agent log area -->
  <main
    bind:this={logEl}
    class="flex-1 overflow-y-auto p-4 space-y-3"
  >
    {#if steps.length === 0}
      <div class="flex flex-col items-center justify-center h-full text-base-content/30 select-none pt-16">
        <div class="text-5xl mb-4">🤖</div>
        <p class="text-lg font-medium">Give Zelfi a goal to get started.</p>
        <p class="text-sm mt-1">Your local AI agent — thinks, acts, remembers.</p>
      </div>
    {/if}

    {#each steps as step, i}
      <div class="card bg-base-200 border border-base-300 shadow-sm">
        <div class="card-body p-4">
          <div class="flex items-center gap-2 mb-2">
            <span class="badge badge-primary badge-sm">Step {i + 1}</span>
            {#if step.error}
              <span class="badge badge-error badge-sm">Error</span>
            {:else if step.complete}
              <span class="badge badge-success badge-sm">Done</span>
            {:else}
              <span class="loading loading-dots loading-xs text-primary"></span>
            {/if}
          </div>

          {#if step.error}
            <div role="alert" class="alert alert-error text-sm py-2 px-3">
              <span>{step.error}</span>
            </div>
          {:else}
            <p class="text-sm text-base-content whitespace-pre-wrap leading-relaxed">
              {step.text || (streaming && !step.complete ? '▌' : '')}
            </p>
          {/if}
        </div>
      </div>
    {/each}
  </main>

  <!-- Input area -->
  <footer class="bg-base-200 border-t border-base-300 p-4 flex-shrink-0">
    <div class="flex gap-2 items-end">
      <textarea
        class="textarea textarea-bordered flex-1 resize-none text-sm min-h-[60px] max-h-[160px]"
        placeholder="Give Zelfi a goal… (⌘+Enter to send)"
        bind:value={prompt}
        onkeydown={handleKeydown}
        rows="2"
        disabled={streaming}
      ></textarea>

      <!-- Disabled voice button -->
      <div class="tooltip tooltip-top" data-tip="Voice input — coming soon">
        <button
          class="btn btn-square btn-ghost btn-sm opacity-40 cursor-not-allowed"
          disabled
          aria-label="Voice input — coming soon"
        >
          🎤
        </button>
      </div>

      <button
        class="btn btn-primary btn-sm h-[60px] px-5"
        onclick={sendMessage}
        disabled={streaming || !prompt.trim()}
      >
        {#if streaming}
          <span class="loading loading-spinner loading-xs"></span>
        {:else}
          Send
        {/if}
      </button>
    </div>
    <p class="text-xs text-base-content/30 mt-1.5 pl-1">Powered by LM Studio · Gemma 4 E4B-it MLX · 100% local</p>
  </footer>
</div>
