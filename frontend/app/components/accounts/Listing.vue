<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

const props = defineProps(["id", "name"]);

const link = "/mails/" + props.id;

const deleteAccount = () => {
  invoke("accounts_delete", {
    id: parseInt(props.id),
  })
    .then(async (_) => await refreshNuxtData())
    .catch((err) => console.error(err));
};
</script>

<template>
  <div class="listing-holder">
    <a :href="link">{{ props.name }}</a>
    <button @click="() => deleteAccount()">Delete</button>
  </div>
</template>

<style>
.listing-holder {
  width: 100%;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
}
</style>
