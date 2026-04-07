<template>
  <div class="global-holder">
    <Titlebar />
    <div class="content">
      <slot />
    </div>
  </div>
</template>

<style>
* {
  margin: 0px;
  padding: 0px;
  color: #cdd6f4;
}

.global-holder {
  border: 1px solid #313244;
}

.content {
  background-color: #1e1e2e;
  width: calc(100vw - 2px);
  height: calc(100vh - 24px - 2px);
  padding: 2px;
  overflow-y: scroll;
}
</style>
