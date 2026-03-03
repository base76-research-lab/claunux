<script>
  // Mode B: embed claude.ai directly in the app
  // The user logs in via WebView — sessions, cookies etc handled by WebKit
  const CLAUDE_URL = 'https://claude.ai';
</script>

<div class="webview-wrap">
  <webview
    src={CLAUDE_URL}
    class="claude-webview"
    allowpopups
  ></webview>
</div>

<style>
  .webview-wrap {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .claude-webview {
    flex: 1;
    width: 100%;
    height: 100%;
    border: none;
  }
</style>
