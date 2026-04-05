import { defineStore } from 'pinia'
import { ref } from 'vue'
import { communityApi } from '../api/community'
import type { CommunityUser, CommunityUserDetail } from '../types'

export const useCommunityStore = defineStore('community', () => {
  const users = ref<CommunityUser[]>([])
  const total = ref(0)
  const currentPage = ref(1)
  const perPage = ref(20)
  const loading = ref(false)
  const userDetail = ref<CommunityUserDetail | null>(null)

  async function fetchUsers(page = 1) {
    loading.value = true
    try {
      const { data } = await communityApi.listUsers(page, perPage.value)
      users.value = data.users
      total.value = data.total
      currentPage.value = data.page
    } finally {
      loading.value = false
    }
  }

  async function fetchUserDetail(id: string) {
    loading.value = true
    try {
      const { data } = await communityApi.getUser(id)
      userDetail.value = data
    } finally {
      loading.value = false
    }
  }

  return { users, total, currentPage, perPage, loading, userDetail, fetchUsers, fetchUserDetail }
})
