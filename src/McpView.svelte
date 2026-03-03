<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let servers = {};
  let loading = true;
  let saving = false;
  let error = '';

  // Add-server form
  let showForm = false;
  let newName = '';
  let newCommand = '';
  let newArgs = '';

  onMount(async () => {
    await loadServers();
  });

  async function loadServers() {
    loading = true;
    error = '';
    try {
      servers = await invoke('get_mcp_servers');
    } catch (e) {
      error = e;
    } finally {
      loading = false;
    }
  }

  async function addServer() {
    if (!newName.trim() || !newCommand.trim()) return;
    const args = newArgs.trim() ? newArgs.trim().split(/\s+/) : [];
    servers = {
      ...servers,
      [newName.trim()]: { command: newCommand.trim(), args, env: {} }
    };
    await persist();
    newName = ''; newCommand = ''; newArgs = '';
    showForm = false;
  }

  async function removeServer(name) {
    const updated = { ...servers };
    delete updated[name];
    servers = updated;
    await persist();
  }

  async function persist() {
    saving = true;
    error = '';
    try {
      await invoke('save_mcp_servers', { servers });
    } catch (e) {
      error = e;
    } finally {
      saving = false;
    }
  }

  function handleAddKey(e) {
    if (e.key === 'Enter') addServer();
    if (e.key === 'Escape') { showForm = false; }
  }
</script>

<div class="mcp-view">
  <div class="header">
    <h2>MCP Servers</h2>
    <button class="add-btn" on:click={() => showForm = !showForm}>
      {showForm ? '✕' : '+ Add server'}
    </button>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if showForm}
    <div class="form">
      <input
        bind:value={newName}
        placeholder="Server name (e.g. conductor-graph)"
        on:keydown={handleAddKey}
      />
      <input
        bind:value={newCommand}
        placeholder="Command (e.g. python3)"
        on:keydown={handleAddKey}
      />
      <input
        bind:value={newArgs}
        placeholder="Args (space-separated, e.g. /path/to/server.py)"
        on:keydown={handleAddKey}
      />
      <button class="save-btn" on:click={addServer}>Add</button>
    </div>
  {/if}

  {#if loading}
    <div class="empty">Loading…</div>
  {:else if Object.keys(servers).length === 0}
    <div class="empty">
      <p>No MCP servers configured.</p>
      <small>MCP servers extend Claude with tools — filesystem, search, APIs, and more.</small>
    </div>
  {:else}
    <div class="server-list">
      {#each Object.entries(servers) as [name, server]}
        <div class="server-row">
          <div class="server-info">
            <div class="server-name">
              <span class="dot"></span>
              {name}
            </div>
            <div class="server-cmd">
              {server.command}{server.args.length ? ' ' + server.args.join(' ') : ''}
            </div>
          </div>
          <button class="remove-btn" on:click={() => removeServer(name)}>Remove</button>
        </div>
      {/each}
    </div>
  {/if}

  {#if saving}
    <div class="status">Saving…</div>
  {/if}

  <div class="config-path">
    Config: ~/.config/claunux/mcp_servers.json
  </div>
</div>

<style>
  .mcp-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 20px;
    background: #1C1C1E;
    color: #E8E3DD;
    gap: 16px;
    overflow-y: auto;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  h2 {
    font-size: 16px;
    font-weight: 600;
    color: #E8E3DD;
  }

  .add-btn {
    background: #CC785C;
    color: #fff;
    border: none;
    border-radius: 6px;
    padding: 6px 12px;
    font-size: 13px;
    cursor: pointer;
  }
  .add-btn:hover { opacity: 0.85; }

  .form {
    display: flex;
    flex-direction: column;
    gap: 8px;
    background: #111113;
    border: 1px solid #2E2E30;
    border-radius: 10px;
    padding: 14px;
  }

  input {
    background: #2A2A2C;
    border: 1px solid #3A3A3C;
    border-radius: 6px;
    padding: 8px 10px;
    color: #E8E3DD;
    font-size: 13px;
    font-family: inherit;
  }
  input:focus { outline: none; border-color: #CC785C55; }
  input::placeholder { color: #666; }

  .save-btn {
    background: #2A2A2C;
    border: 1px solid #3A3A3C;
    color: #E8E3DD;
    border-radius: 6px;
    padding: 7px 14px;
    font-size: 13px;
    cursor: pointer;
    align-self: flex-end;
  }
  .save-btn:hover { background: #333336; }

  .empty {
    text-align: center;
    padding: 40px 20px;
    color: #666;
    font-size: 14px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .empty small { font-size: 12px; }

  .server-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .server-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: #111113;
    border: 1px solid #2E2E30;
    border-radius: 10px;
    padding: 12px 14px;
  }

  .server-info {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  .server-name {
    font-size: 14px;
    font-weight: 500;
    color: #E8E3DD;
    display: flex;
    align-items: center;
    gap: 7px;
  }

  .dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #4CAF50;
    flex-shrink: 0;
  }

  .server-cmd {
    font-size: 12px;
    color: #666;
    font-family: monospace;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 380px;
  }

  .remove-btn {
    background: transparent;
    border: 1px solid #3A3A3C;
    color: #888;
    border-radius: 6px;
    padding: 5px 10px;
    font-size: 12px;
    cursor: pointer;
    flex-shrink: 0;
  }
  .remove-btn:hover { border-color: #CC785C; color: #CC785C; }

  .error {
    background: #3A1A1A;
    border: 1px solid #CC4444;
    border-radius: 8px;
    padding: 10px 14px;
    font-size: 13px;
    color: #FF6B6B;
  }

  .status {
    font-size: 12px;
    color: #666;
    text-align: center;
  }

  .config-path {
    margin-top: auto;
    font-size: 11px;
    color: #444;
    text-align: center;
    font-family: monospace;
  }
</style>
