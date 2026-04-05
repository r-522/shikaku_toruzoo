import { defineStore } from 'pinia'
import { favoriteApi } from '../api/favorite'
import { useCommunityStore } from './community'

export const useFavoriteStore = defineStore('favorite', () => {
  async function toggle(userId: string, isFavorite: boolean) {
    if (isFavorite) {
      await favoriteApi.remove(userId)
    } else {
      await favoriteApi.add(userId)
    }
    // Refresh community list
    const communityStore = useCommunityStore()
    await communityStore.fetchUsers(communityStore.currentPage)
  }

  return { toggle }
})
