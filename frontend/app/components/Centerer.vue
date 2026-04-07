<template>
  <div class="centerer">
    <slot />
  </div>
</template>

<style>
.centerer {
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
</style>
