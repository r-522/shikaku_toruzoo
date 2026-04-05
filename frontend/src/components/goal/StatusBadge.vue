<template>
  <span class="badge" :style="{ backgroundColor: color }">{{ label }}</span>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { GoalStatus } from '../../types'

const props = defineProps<{
  status: GoalStatus
}>()

const statusMap: Record<GoalStatus, { label: string; color: string }> = {
  exam_date: { label: '受験日', color: '#1565C0' },
  passed: { label: '合格', color: '#2E7D32' },
  failed: { label: '不合格', color: '#E65100' },
  abandoned: { label: '断念', color: '#616161' },
}

const label = computed(() => statusMap[props.status]?.label ?? props.status)
const color = computed(() => statusMap[props.status]?.color ?? '#757575')
</script>
