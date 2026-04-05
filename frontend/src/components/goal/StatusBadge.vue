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
  studying: { label: '学習中', color: '#1565C0' },
  scheduled: { label: '受験予定', color: '#F57F17' },
  achieved: { label: '達成', color: '#2E7D32' },
  suspended: { label: '中断', color: '#616161' },
}

const label = computed(() => statusMap[props.status]?.label ?? props.status)
const color = computed(() => statusMap[props.status]?.color ?? '#757575')
</script>
