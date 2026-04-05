<template>
  <div class="text-center py-5" style="color: #9E9E9E">
    <p class="fs-5">{{ message }}</p>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  message: string
}>()
</script>
