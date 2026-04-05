<template>
  <span v-if="hasGoodMark" class="badge rounded-pill" style="background-color: #2E7D32; font-size: 0.75rem">
    &#10003;
  </span>
</template>

<script setup lang="ts">
defineProps<{
  hasGoodMark: boolean
}>()
</script>
