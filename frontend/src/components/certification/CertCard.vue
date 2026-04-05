<template>
  <div class="card shadow-sm mb-2">
    <div class="card-body d-flex justify-content-between align-items-center py-3">
      <div>
        <div class="fw-bold">{{ cert.certification_name }}</div>
        <small class="text-muted">{{ cert.acquired_date || '日付未設定' }}</small>
      </div>
      <div>
        <button class="btn btn-outline-secondary btn-sm me-1" @click="$emit('edit', cert)">編集</button>
        <button class="btn btn-outline-danger btn-sm" @click="$emit('delete', cert)">削除</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Certification } from '../../types'

defineProps<{
  cert: Certification
}>()

defineEmits<{
  edit: [cert: Certification]
  delete: [cert: Certification]
}>()
</script>
