<script lang="ts" setup>
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted } from "vue";

const appWindow = getCurrentWindow();

onMounted(() => {
  document
    .getElementById("titlebar-minimize")
    ?.addEventListener("click", () => appWindow.minimize());
  document
    .getElementById("titlebar-maximize")
    ?.addEventListener("click", () => appWindow.toggleMaximize());
  document
    .getElementById("titlebar-close")
    ?.addEventListener("click", () => appWindow.close());
});
</script>

<template>
  <div class="titlebar">
    <div class="drag-region" data-tauri-drag-region></div>
    <p class="title">Pluton</p>
    <img class="logo" src="~/assets/img/pluton.png" />
    <div class="controls">
      <button id="titlebar-minimize" title="minimize">
        <!-- https://api.iconify.design/mdi:window-minimize.svg -->
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        >
          <path fill="#4c4f69" d="M19 13H5v-2h14z" />
        </svg>
      </button>
      <button id="titlebar-maximize" title="maximize">
        <!-- https://api.iconify.design/mdi:window-maximize.svg -->
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        >
          <path fill="#4c4f69" d="M4 4h16v16H4zm2 4v10h12V8z" />
        </svg>
      </button>
      <button id="titlebar-close" title="close">
        <!-- https://api.iconify.design/mdi:close.svg -->
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        >
          <path
            fill="#4c4f69"
            d="M13.46 12L19 17.54V19h-1.46L12 13.46L6.46 19H5v-1.46L10.54 12L5 6.46V5h1.46L12 10.54L17.54 5H19v1.46z"
          />
        </svg>
      </button>
    </div>
  </div>
</template>

<style>
.titlebar > .drag-region {
  position: absolute;
  width: 100%;
  height: var(--title-bar-height);
  z-index: 5;
}

.titlebar > .title {
  position: absolute;
  display: flex;
  justify-content: center;
  width: 100%;
}

.titlebar > .logo {
  height: 100%;
}

.titlebar {
  height: var(--title-bar-height);
  background: #1e1e2e;
  border-bottom: 1px solid #313244;
  user-select: none;
  display: flex;
  align-items: center;
  justify-content: space-between;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar > .controls {
  display: flex;
  z-index: 10;
}
.titlebar button {
  appearance: none;
  padding: 0;
  margin: 0;
  border: none;
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  background-color: transparent;
}
.titlebar button:hover {
  background: #313244;
}
.titlebar button:active {
  background: #45475a;
}
</style>
