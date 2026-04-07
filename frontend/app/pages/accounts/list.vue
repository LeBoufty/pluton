<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { useTauriInvoke } from "~/composables/useTauriInvoke";

// const accounts: { [key: number]: Account } = await invoke("accounts_get_all");*
const {
  data: accounts,
  error: accounts_error,
  pending: accounts_pending,
} = await useTauriInvoke<{
  [key: number]: Account;
}>("accounts_get_all");
</script>

<script lang="ts"></script>

<template>
  <Centerer>
    <h1 class="accounts-title">Accounts</h1>
    <div class="accounts-holder">
      <div v-if="accounts_pending">Loading...</div>
      <div v-else-if="accounts_error">Error, check console</div>
      <AccountsListing
        v-else-if="accounts"
        v-for="[id, acc] in Object.entries(accounts)"
        :key="id"
        :id="id"
        :name="acc.username"
      />
    </div>
    <button class="accounts-create" @click="navigateTo('/accounts/new')">
      Create new account
    </button></Centerer
  >
</template>

<style>
.accounts-holder {
  height: 80%;
  width: 400px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  background-color: #181825;
}
</style>
