<script>
  import ChatView from './ChatView.svelte';
  import WebView from './WebView.svelte';
  import LoginView from './LoginView.svelte';
  import McpView from './McpView.svelte';
  import logo from './claunux.png';

  // Persist mode across restarts
  let mode = localStorage.getItem('claunux_mode') || null;
  let activeTab = 'chat'; // chat | mcp

  function selectMode(selected) {
    mode = selected;
    localStorage.setItem('claunux_mode', selected);
  }

  function resetMode() {
    mode = null;
    localStorage.removeItem('claunux_mode');
  }
</script>

<style>
  :root {
    --accent:       #CC785C;
    --bg:           #1C1C1E;
    --bg-sidebar:   #111113;
    --bg-input:     #2A2A2C;
    --text:         #E8E3DD;
    --text-muted:   #888888;
    --border:       #2E2E30;
  }

  * { box-sizing: border-box; margin: 0; padding: 0; }

  body {
    background: var(--bg);
    color: var(--text);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
    height: 100vh;
    overflow: hidden;
  }

  .shell {
    display: flex;
    height: 100vh;
    border-radius: 10px;
    overflow: hidden;
    border: 1px solid var(--border);
  }

  /* ── Sidebar ── */
  .sidebar {
    width: 52px;
    background: var(--bg-sidebar);
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 12px 0;
    gap: 8px;
    border-right: 1px solid var(--border);
    -webkit-app-region: drag;
  }

  .logo {
    -webkit-app-region: no-drag;
    width: 34px;
    height: 34px;
    border-radius: 8px;
    overflow: hidden;
    margin-bottom: 8px;
  }
  .logo img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .nav-icon {
    -webkit-app-region: no-drag;
    width: 34px;
    height: 34px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 16px;
    transition: background 0.15s, color 0.15s;
  }
  .nav-icon:hover { background: #222224; color: var(--text); }
  .nav-icon.active { background: #2A2A2C; color: var(--accent); }

  .spacer { flex: 1; }

  /* ── Main ── */
  .main {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: var(--bg);
    min-width: 0;
  }

  /* custom titlebar drag region */
  .titlebar {
    -webkit-app-region: drag;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }
  .titlebar span {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-muted);
    letter-spacing: 0.03em;
    -webkit-app-region: no-drag;
  }

  .chat-wrap {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }

  footer {
    text-align: center;
    padding: 5px;
    font-size: 11px;
    color: var(--text-muted);
    border-top: 1px solid var(--border);
    flex-shrink: 0;
  }
</style>

{#if !mode}
  <LoginView onSelect={selectMode} />
{:else}
<div class="shell">
  <!-- Sidebar -->
  <aside class="sidebar">
    <div class="logo">
      <img src={logo} alt="Claunux" />
    </div>
    <div class="nav-icon" class:active={activeTab === 'chat'} title="Chat" on:click={() => activeTab = 'chat'} role="button" tabindex="0" on:keydown={e => e.key === 'Enter' && (activeTab = 'chat')}>💬</div>
    <div class="nav-icon" title="Filer">📁</div>
    <div class="nav-icon" class:active={activeTab === 'mcp'} title="MCP Servers" on:click={() => activeTab = 'mcp'} role="button" tabindex="0" on:keydown={e => e.key === 'Enter' && (activeTab = 'mcp')}>🔌</div>
    <div class="spacer"></div>
    <div class="nav-icon" title="Byt inloggning" on:click={resetMode} role="button" tabindex="0" on:keydown={e => e.key === 'Enter' && resetMode()}>⚙️</div>
  </aside>

  <!-- Main content -->
  <div class="main">
    <div class="titlebar"><span>Claunux</span></div>
    <div class="chat-wrap">
      {#if activeTab === 'mcp'}
        <McpView />
      {:else if mode === 'webview'}
        <WebView />
      {:else}
        <ChatView />
      {/if}
    </div>
    {#if mode === 'api'}
      <footer>Produced by Base76 Research Lab</footer>
    {/if}
  </div>
</div>
{/if}
