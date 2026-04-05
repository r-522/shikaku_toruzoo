import apiClient from './client'
import type { CommunityUser, CommunityUserDetail, PaginatedResponse } from '../types'

export const communityApi = {
  listUsers(page = 1, perPage = 20) {
    return apiClient.get<PaginatedResponse<CommunityUser>>('/api/community/users', {
      params: { page, per_page: perPage },
    })
  },

  getUser(id: string) {
    return apiClient.get<CommunityUserDetail>(`/api/community/users/${id}`)
  },
}
