<template>
  <button
    class="btn btn-sm p-0 border-0"
    @click="$emit('toggle')"
    :style="{ color: isFavorite ? '#F9A825' : '#BDBDBD', fontSize: '1.4rem' }"
  >
    {{ isFavorite ? '\u2605' : '\u2606' }}
  </button>
</template>

<script setup lang="ts">
defineProps<{
  isFavorite: boolean
}>()

defineEmits<{
  toggle: []
}>()
</script>
