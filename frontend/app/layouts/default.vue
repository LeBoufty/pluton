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
  border: 0px;
  color: #cdd6f4;
  overflow: hidden;
}

button,
input,
select,
option {
  background-color: #1e1e2e;
  border: 2px solid #313244;
  padding: 4px;
}

.global-holder {
  border: 1px solid #313244;
}

.content {
  background-color: #11111b;
  width: calc(100vw - 6px);
  height: calc(100vh - var(--title-bar-height) - 7px);
  padding: 2px;
  overflow-y: scroll;
  overflow-x: hidden;
}
</style>
