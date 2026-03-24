<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
</script>

<main class="dashboard">
  <header class="header">
    <h1>IronSight Pro</h1>
    <p class="subtitle">Endpoint Detection & Response Dashboard (Tauri + SvelteKit)</p>
  </header>

  <div class="grid">
    <div class="card">
      <h2 class="card-title">Sentinel Watchdog</h2>
      <p class="card-description">System monitoring active and stable.</p>
    </div>
    <div class="card">
      <h2 class="card-title">Memory Scanner</h2>
      <p class="card-description">Awaiting manual or heuristic trigger.</p>
    </div>
    <div class="card">
      <h2 class="card-title">Network Forensics</h2>
      <p class="card-description">0 active anomalous connections.</p>
    </div>
  </div>
</main>

<style>
  .dashboard {
    padding: 32px;
    display: flex;
    flex-direction: column;
    gap: 32px;
    height: 100vh;
    box-sizing: border-box;
  }
  .header h1 {
    font-size: 28px;
    font-weight: 700;
    margin-bottom: 8px;
    color: var(--foreground-bright);
    letter-spacing: -0.5px;
  }
  .header .subtitle {
    color: var(--foreground-muted);
    font-size: 14px;
    margin: 0;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 24px;
  }
  .card {
    background: var(--card);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 24px;
    box-shadow: var(--shadow);
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .card-title {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--foreground-bright);
    margin: 0;
  }
  .card-description {
    font-size: 0.875rem;
    color: var(--foreground-muted);
    margin: 0;
  }
</style>
