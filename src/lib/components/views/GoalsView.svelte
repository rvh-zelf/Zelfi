<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onDestroy, tick } from 'svelte';
  import { lmBaseUrl } from '$lib/stores/ui';
  import { Zap, Mic, Pause, Square } from 'lucide-svelte';

  // ── Types ─────────────────────────────────────────────────────────────────

  type StepType = 'PLAN' | 'TOOL' | 'RESULT' | 'EVAL' | 'ERROR';

  type Step = {
    id: string;
    type: StepType;
    content: string;
    timestamp: string;
    complete: boolean;
  };

  // ── State ─────────────────────────────────────────────────────────────────

  let steps = $state<Step[]>([]);
  let isRunning = $state(false);
  let paused = $state(false);
  let currentGoal = $state('');
  let unlisten: (() => void) | null = null;

  let logEl: HTMLElement;

  // ── Step styling maps ─────────────────────────────────────────────────────

  const borderColour: Record<StepType, string> = {
    PLAN:   'border-primary',
    TOOL:   'border-warning',
    RESULT: 'border-success',
    EVAL:   'border-secondary',
    ERROR:  'border-error',
  };

  const badgeClass: Record<StepType, string> = {
    PLAN:   'badge-primary',
    TOOL:   'badge-warning',
    RESULT: 'badge-success',
    EVAL:   'badge-secondary',
    ERROR:  'badge-error',
  };

  const fontClass: Record<StepType, string> = {
    PLAN:   '',
    TOOL:   'font-mono',
    RESULT: '',
    EVAL:   '',
    ERROR:  '',
  };

  // ── Helpers ───────────────────────────────────────────────────────────────

  function makeId(): string {
    return Math.random().toString(36).slice(2, 9);
  }

  function makeTimestamp(): string {
    return new Date().toLocaleTimeString('en-ZA', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
  }

  async function scrollToBottom() {
    await tick();
    if (logEl) {
      logEl.scrollTop = logEl.scrollHeight;
    }
  }

  function cleanup() {
    unlisten?.();
    unlisten = null;
  }

  // ── Goal submission ───────────────────────────────────────────────────────

  async function sendGoal() {
    if (!currentGoal.trim() || isRunning) return;

    const prompt = currentGoal.trim();
    currentGoal = '';
    isRunning = true;
    paused = false;

    // Create the new RESULT step card
    const newStep: Step = {
      id: makeId(),
      type: 'RESULT',
      content: '',
      timestamp: makeTimestamp(),
      complete: false,
    };
    steps.push(newStep);
    const stepIdx = steps.length - 1;
    await scrollToBottom();

    // Register event listeners
    const unlistenToken = await listen<{ token: string }>('zelfi://token', (event) => {
      steps[stepIdx].content += event.payload.token;
      scrollToBottom();
    });

    const unlistenDone = await listen('zelfi://done', () => {
      steps[stepIdx].complete = true;
      isRunning = false;
      cleanup();
    });

    const unlistenError = await listen<{ message: string }>('zelfi://error', (event) => {
      // Replace the RESULT card with an ERROR card
      steps[stepIdx] = {
        ...steps[stepIdx],
        type: 'ERROR',
        content: event.payload.message,
        complete: true,
      };
      isRunning = false;
      cleanup();
    });

    // Combine all unlisteners
    unlisten = () => {
      unlistenToken();
      unlistenDone();
      unlistenError();
    };

    try {
      await invoke('send_message', { prompt, baseUrl: $lmBaseUrl });
    } catch (err) {
      steps[stepIdx] = {
        ...steps[stepIdx],
        type: 'ERROR',
        content: String(err),
        complete: true,
      };
      isRunning = false;
      cleanup();
    }
  }

  // ── Pause / Stop ──────────────────────────────────────────────────────────

  function handlePause() {
    paused = !paused;
    // Actual pause logic deferred to a future iteration
  }

  function handleStop() {
    const activeIdx = steps.findIndex((s) => !s.complete);
    if (activeIdx !== -1) {
      steps[activeIdx] = {
        ...steps[activeIdx],
        content: steps[activeIdx].content + '\n\n[Stopped by user]',
        complete: true,
      };
    }
    isRunning = false;
    paused = false;
    cleanup();
  }

  // ── Keyboard shortcut ─────────────────────────────────────────────────────

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      sendGoal();
    }
  }

  // ── Cleanup on destroy ────────────────────────────────────────────────────

  onDestroy(() => {
    cleanup();
  });
</script>

<!-- Full-height flex column filling the content area -->
<div class="flex flex-col h-full overflow-hidden">

  <!-- ── Agent log (scrollable, flex-1) ──────────────────────────────── -->
  <div
    bind:this={logEl}
    class="flex-1 overflow-y-auto py-4"
    aria-label="Agent log"
    aria-live="polite"
  >
    {#if steps.length === 0}
      <!-- Empty state -->
      <div class="flex flex-col items-center justify-center h-full gap-3 select-none text-base-content/30">
        <Zap size={48} class="text-primary opacity-60" />
        <p class="text-base font-medium">Give Zelfi a goal</p>
        <p class="text-sm">and watch it work.</p>
      </div>
    {:else}
      {#each steps as step (step.id)}
        <div class="card bg-base-200 border-l-4 {borderColour[step.type]} mb-3 mx-4 shadow-sm">
          <div class="card-body p-3">
            <div class="flex items-center gap-2 mb-1">
              <span class="badge badge-sm {badgeClass[step.type]} font-mono text-xs">{step.type}</span>
              <span class="text-xs text-base-content/50">{step.timestamp}</span>
            </div>
            <p class="text-sm {fontClass[step.type]} whitespace-pre-wrap leading-relaxed">
              {step.content}{#if !step.complete && isRunning}<span class="blinking-cursor">█</span>{/if}
            </p>
          </div>
        </div>
      {/each}
    {/if}
  </div>

  <!-- ── Pause / Stop row (shown only when running) ──────────────────── -->
  <div
    class="flex items-center gap-2 px-4 py-2 border-t border-base-300 bg-base-100 transition-all duration-300 overflow-hidden {isRunning ? 'opacity-100 max-h-16' : 'opacity-0 max-h-0 py-0 border-t-0'}"
    aria-hidden={!isRunning}
  >
    <button
      class="btn btn-warning btn-sm gap-1.5"
      onclick={handlePause}
      disabled={!isRunning}
      aria-label={paused ? 'Resume' : 'Pause'}
    >
      <Pause size={14} />
      {paused ? 'Resume' : 'Pause'}
    </button>
    <button
      class="btn btn-error btn-sm gap-1.5"
      onclick={handleStop}
      disabled={!isRunning}
      aria-label="Stop"
    >
      <Square size={14} />
      Stop
    </button>
  </div>

  <!-- ── Goal input area ─────────────────────────────────────────────── -->
  <div class="bg-base-200 border-t border-base-300 p-3 flex-shrink-0">
    <div class="flex items-end gap-2">
      <!-- Textarea -->
      <textarea
        class="textarea textarea-bordered flex-1 resize-none text-sm"
        placeholder="Give Zelfi a goal…"
        bind:value={currentGoal}
        onkeydown={handleKeydown}
        rows={3}
        disabled={isRunning}
        aria-label="Goal input"
      ></textarea>

      <!-- Right-side button column -->
      <div class="flex flex-col gap-1.5 items-center">
        <!-- Send button -->
        <button
          class="btn btn-primary btn-sm gap-1.5"
          onclick={sendGoal}
          disabled={isRunning || !currentGoal.trim()}
          aria-label="Send goal"
        >
          <Zap size={14} />
        </button>

        <!-- Mic button — disabled, future feature -->
        <div class="tooltip tooltip-left" data-tip="Voice input — coming soon">
          <button
            class="btn btn-ghost btn-sm btn-circle opacity-40 cursor-not-allowed"
            disabled
            aria-label="Voice input — coming soon"
          >
            <Mic size={14} />
          </button>
        </div>
      </div>
    </div>

    <p class="text-xs text-base-content/30 mt-1.5 pl-1">
      ⌘+Enter to send · 100% local
    </p>
  </div>

</div>

<style>
  .blinking-cursor {
    animation: blink 1s step-start infinite;
    display: inline-block;
    line-height: 1;
    vertical-align: middle;
    margin-left: 1px;
  }

  @keyframes blink {
    0%, 100% { opacity: 1; }
    50%       { opacity: 0; }
  }
</style>
