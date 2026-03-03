<script>
  import logo from './claunux.png';

  export let onSelect;
</script>

<div class="login-screen">
  <div class="card">
    <img src={logo} alt="Claunux" class="logo" />
    <h1>Claunux</h1>
    <p class="sub">Native Linux desktop app for Claude AI</p>

    <div class="modes">
      <button class="mode-btn primary" on:click={() => onSelect('webview')}>
        <span class="icon">🌐</span>
        <div class="text">
          <strong>Log in with claude.ai</strong>
          <small>Anyone with a Claude account — Free, Pro, Team</small>
        </div>
      </button>

      <button class="mode-btn secondary" on:click={() => onSelect('api')}>
        <span class="icon">🔑</span>
        <div class="text">
          <strong>Use API key</strong>
          <small>Developers and power users</small>
        </div>
      </button>
    </div>
  </div>
</div>

<style>
  .login-screen {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    background: #1C1C1E;
  }

  .card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    padding: 40px 32px;
    background: #111113;
    border: 1px solid #2E2E30;
    border-radius: 16px;
    width: 340px;
  }

  .logo {
    width: 64px;
    height: 64px;
    border-radius: 14px;
  }

  h1 {
    font-size: 22px;
    font-weight: 600;
    color: #E8E3DD;
    margin: 0;
  }

  .sub {
    font-size: 13px;
    color: #888;
    margin: 0;
    text-align: center;
  }

  .modes {
    display: flex;
    flex-direction: column;
    gap: 10px;
    width: 100%;
    margin-top: 8px;
  }

  .mode-btn {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 14px 16px;
    border-radius: 10px;
    border: 1px solid #2E2E30;
    cursor: pointer;
    text-align: left;
    width: 100%;
    transition: background 0.15s, border-color 0.15s;
  }

  .mode-btn.primary {
    background: #CC785C22;
    border-color: #CC785C55;
  }
  .mode-btn.primary:hover {
    background: #CC785C33;
    border-color: #CC785C99;
  }

  .mode-btn.secondary {
    background: #2A2A2C;
  }
  .mode-btn.secondary:hover {
    background: #333336;
  }

  .icon {
    font-size: 20px;
    flex-shrink: 0;
  }

  .text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  strong {
    font-size: 14px;
    font-weight: 600;
    color: #E8E3DD;
  }

  small {
    font-size: 12px;
    color: #888;
  }
</style>
