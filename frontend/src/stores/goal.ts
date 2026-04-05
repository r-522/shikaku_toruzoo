import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { goalApi } from '../api/goal'
import type { Goal, GoalForm } from '../types'

export const useGoalStore = defineStore('goal', () => {
  const goals = ref<Goal[]>([])
  const loading = ref(false)

  const studyingGoals = computed(() => goals.value.filter((g) => g.status === 'studying'))
  const scheduledGoals = computed(() => goals.value.filter((g) => g.status === 'scheduled'))
  const achievedGoals = computed(() => goals.value.filter((g) => g.status === 'achieved'))
  const suspendedGoals = computed(() => goals.value.filter((g) => g.status === 'suspended'))

  async function fetchAll() {
    loading.value = true
    try {
      const { data } = await goalApi.list()
      goals.value = data.goals
    } finally {
      loading.value = false
    }
  }

  async function add(form: Partial<GoalForm>) {
    const { data } = await goalApi.create(form)
    goals.value.unshift(data)
  }

  async function update(id: string, form: Partial<GoalForm>) {
    const { data } = await goalApi.update(id, form)
    const idx = goals.value.findIndex((g) => g.id === id)
    if (idx !== -1) goals.value[idx] = data
  }

  async function remove(id: string) {
    await goalApi.remove(id)
    goals.value = goals.value.filter((g) => g.id !== id)
  }

  return {
    goals, loading,
    studyingGoals, scheduledGoals, achievedGoals, suspendedGoals,
    fetchAll, add, update, remove,
  }
})
