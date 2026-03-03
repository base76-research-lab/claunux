<script>
  import { invoke } from '@tauri-apps/api/core';
  import { afterUpdate } from 'svelte';

  export let messages = [];
  export let inputText = '';
  export let isLoading = false;

  let textarea;
  let messageList;
  let isDragging = false;
  let attachedFiles = []; // [{ name, path, content }]

  async function sendMessage() {
    const trimmed = inputText.trim();
    if ((!trimmed && attachedFiles.length === 0) || isLoading) return;

    // Build prompt: file contents first, then user message
    let fullPrompt = '';
    for (const f of attachedFiles) {
      fullPrompt += `<file name="${f.name}">\n${f.content}\n</file>\n\n`;
    }
    fullPrompt += trimmed;

    const displayText = [
      ...attachedFiles.map(f => `📎 ${f.name}`),
      trimmed
    ].filter(Boolean).join('\n');

    messages = [...messages, { role: 'user', content: displayText }];
    inputText = '';
    attachedFiles = [];
    isLoading = true;
    adjustHeight();

    try {
      const reply = await invoke('send_message', { prompt: fullPrompt });
      messages = [...messages, { role: 'assistant', content: reply }];
    } catch (e) {
      messages = [...messages, { role: 'assistant', content: `Fel: ${e}` }];
    } finally {
      isLoading = false;
    }
  }

  function handleKeydown(e) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  }

  function adjustHeight() {
    if (!textarea) return;
    textarea.style.height = 'auto';
    const lineHeight = parseInt(getComputedStyle(textarea).lineHeight);
    textarea.style.height = Math.min(textarea.scrollHeight, lineHeight * 6) + 'px';
  }

  // ── Drag & drop ──

  function onDragOver(e) {
    e.preventDefault();
    isDragging = true;
  }

  function onDragLeave(e) {
    // Only clear if leaving the chat-view itself
    if (!e.currentTarget.contains(e.relatedTarget)) {
      isDragging = false;
    }
  }

  async function onDrop(e) {
    e.preventDefault();
    isDragging = false;

    const files = Array.from(e.dataTransfer.files);
    for (const file of files) {
      const path = file.path; // Tauri exposes full path on desktop
      if (!path) continue;
      try {
        const content = await invoke('read_file_content', { path });
        attachedFiles = [...attachedFiles, { name: file.name, path, content }];
      } catch (err) {
        attachedFiles = [...attachedFiles, {
          name: file.name,
          path,
          content: `[Could not read file: ${err}]`
        }];
      }
    }
  }

  function removeAttachment(idx) {
    attachedFiles = attachedFiles.filter((_, i) => i !== idx);
  }

  afterUpdate(() => {
    if (messageList) messageList.scrollTop = messageList.scrollHeight;
  });
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class="chat-view"
  class:dragging={isDragging}
  on:dragover={onDragOver}
  on:dragleave={onDragLeave}
  on:drop={onDrop}
>
  {#if isDragging}
    <div class="drop-overlay">
      <div class="drop-hint">
        <span class="drop-icon">📂</span>
        <span>Drop files to add to context</span>
      </div>
    </div>
  {/if}

  <div class="messages" bind:this={messageList}>
    {#each messages as { role, content }}
      <div class="message {role}">
        <div class="bubble">{content}</div>
      </div>
    {/each}
    {#if isLoading}
      <div class="loading"><div class="dot"></div></div>
    {/if}
  </div>

  {#if attachedFiles.length > 0}
    <div class="attachments">
      {#each attachedFiles as file, i}
        <div class="attachment">
          <span class="att-icon">📄</span>
          <span class="att-name">{file.name}</span>
          <button class="att-remove" on:click={() => removeAttachment(i)}>✕</button>
        </div>
      {/each}
    </div>
  {/if}

  <div class="input-area">
    <textarea
      bind:this={textarea}
      bind:value={inputText}
      rows="1"
      placeholder="Write a message… or drop a file"
      on:input={adjustHeight}
      on:keydown={handleKeydown}
    ></textarea>
    <button class="send" on:click={sendMessage}>Send</button>
  </div>
</div>

<style>
  .chat-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: #1a1a1a;
    color: #E8E3DD;
    position: relative;
  }

  /* Drop overlay */
  .drop-overlay {
    position: absolute;
    inset: 0;
    background: #CC785C18;
    border: 2px dashed #CC785C;
    border-radius: 4px;
    z-index: 10;
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
  }
  .drop-hint {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    color: #CC785C;
    font-size: 15px;
    font-weight: 500;
  }
  .drop-icon { font-size: 36px; }

  .messages {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .message { display: flex; }
  .message.user { justify-content: flex-end; }
  .message.assistant { justify-content: flex-start; }

  .bubble {
    max-width: 70%;
    padding: 0.6rem 0.9rem;
    border-radius: 0.8rem;
    line-height: 1.4;
    white-space: pre-wrap;
  }
  .message.user .bubble { background: #CC785C; color: #fff; }
  .message.assistant .bubble { background: #F5F0EB; color: #1a1a1a; }

  .loading { display: flex; justify-content: flex-start; padding-left: 0.2rem; }
  .dot {
    width: 10px; height: 10px;
    background: #CC785C; border-radius: 50%;
    animation: pulse 1.5s infinite;
  }
  @keyframes pulse {
    0%, 100% { transform: scale(1); }
    50% { transform: scale(1.4); }
  }

  /* Attachments bar */
  .attachments {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    padding: 6px 8px 2px;
    border-top: 1px solid #333;
  }
  .attachment {
    display: flex;
    align-items: center;
    gap: 5px;
    background: #2A2A2C;
    border: 1px solid #3A3A3C;
    border-radius: 6px;
    padding: 4px 8px;
    font-size: 12px;
    color: #E8E3DD;
  }
  .att-icon { font-size: 13px; }
  .att-name { max-width: 160px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .att-remove {
    background: none; border: none; color: #666;
    cursor: pointer; font-size: 11px; padding: 0 0 0 4px;
  }
  .att-remove:hover { color: #CC785C; }

  .input-area {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem;
    border-top: 1px solid #444;
  }
  textarea {
    flex: 1; resize: none; overflow: hidden;
    border: none; border-radius: 0.5rem; padding: 0.5rem;
    font-family: inherit; font-size: 1rem;
    background: #fff; color: #000;
  }
  textarea:focus { outline: none; }
  button.send {
    background: #CC785C; color: #fff;
    border: none; border-radius: 0.5rem; padding: 0 1rem; cursor: pointer;
  }
  button.send:hover { opacity: 0.9; }
</style>
