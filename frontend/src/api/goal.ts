import apiClient from './client'
import type { Goal, GoalForm } from '../types'

export const goalApi = {
  list() {
    return apiClient.get<{ goals: Goal[] }>('/api/goals')
  },

  create(form: Partial<GoalForm>) {
    return apiClient.post<Goal>('/api/goals', form)
  },

  update(id: string, form: Partial<GoalForm>) {
    return apiClient.put<Goal>(`/api/goals/${id}`, form)
  },

  remove(id: string) {
    return apiClient.delete(`/api/goals/${id}`)
  },
}
