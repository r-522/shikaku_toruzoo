<template>
  <div v-if="show" class="modal d-block" tabindex="-1" style="background-color: rgba(0,0,0,0.5)">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title">{{ title }}</h5>
          <button type="button" class="btn-close" @click="$emit('close')"></button>
        </div>
        <form @submit.prevent="$emit('submit')">
          <div class="modal-body">
            <slot></slot>
          </div>
          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" @click="$emit('close')">キャンセル</button>
            <button type="submit" class="btn btn-primary" :disabled="loading">
              {{ submitText }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  show: boolean
  title: string
  submitText?: string
  loading?: boolean
}>(), {
  submitText: '保存',
  loading: false,
})

defineEmits<{
  close: []
  submit: []
}>()
</script>
