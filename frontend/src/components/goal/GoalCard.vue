<template>
  <div class="card shadow-sm mb-2">
    <div class="card-body py-3">
      <div class="d-flex justify-content-between align-items-start">
        <div class="flex-grow-1">
          <div class="d-flex align-items-center gap-2 mb-1">
            <span class="fw-bold">{{ goal.certification_name }}</span>
            <StatusBadge :status="goal.status" />
          </div>
          <div class="d-flex gap-3">
            <small class="text-muted">目標日: {{ goal.target_date }}</small>
            <small :style="{ color: daysColor }">{{ daysText }}</small>
            <small v-if="goal.study_hours > 0" class="text-muted">{{ goal.study_hours }}h 勉強</small>
          </div>
          <div v-if="goal.memo" class="text-muted small mt-1 text-truncate" style="max-width: 400px">
            {{ goal.memo }}
          </div>
        </div>
        <div class="ms-2">
          <button class="btn btn-outline-secondary btn-sm me-1" @click="$emit('edit', goal)">編集</button>
          <button class="btn btn-outline-danger btn-sm" @click="$emit('delete', goal)">削除</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import StatusBadge from './StatusBadge.vue'
import type { Goal } from '../../types'

const props = defineProps<{
  goal: Goal
}>()

defineEmits<{
  edit: [goal: Goal]
  delete: [goal: Goal]
}>()

const daysRemaining = computed(() => {
  const target = new Date(props.goal.target_date)
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  return Math.ceil((target.getTime() - today.getTime()) / (1000 * 60 * 60 * 24))
})

const daysText = computed(() => {
  const d = daysRemaining.value
  if (d < 0) return `${Math.abs(d)}日超過`
  if (d === 0) return '本日'
  return `残り${d}日`
})

const daysColor = computed(() => daysRemaining.value < 0 ? '#E65100' : '#757575')
</script>
