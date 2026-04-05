import apiClient from './client'

export const favoriteApi = {
  add(userId: string) {
    return apiClient.post(`/api/favorites/${userId}`)
  },

  remove(userId: string) {
    return apiClient.delete(`/api/favorites/${userId}`)
  },

  list() {
    return apiClient.get('/api/favorites')
  },
}
