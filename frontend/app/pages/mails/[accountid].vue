<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
const route = useRoute();
// Todo : add settings
const DEFAULT_BOX = "INBOX";

const emails = invoke(
  "get_email_headers_for_box",
  route.params.accountid,
  DEFAULT_BOX,
);
</script>

<template></template>
