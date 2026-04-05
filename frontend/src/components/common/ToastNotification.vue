<template>
  <div
    v-if="toast.visible.value"
    class="position-fixed top-0 end-0 p-3"
    style="z-index: 9999"
  >
    <div
      class="toast show"
      :class="{
        'bg-success text-white': toast.type.value === 'success',
        'bg-danger text-white': toast.type.value === 'error',
        'bg-info text-white': toast.type.value === 'info',
      }"
    >
      <div class="toast-body d-flex justify-content-between align-items-center">
        <span>{{ toast.message.value }}</span>
        <button type="button" class="btn-close btn-close-white ms-2" @click="toast.hide()"></button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useToast } from '../../composables/useToast'

const toast = useToast()
</script>
