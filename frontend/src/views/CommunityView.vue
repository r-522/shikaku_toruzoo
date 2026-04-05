<template>
  <div class="container py-4">
    <h3 class="mb-4">コミュニティ</h3>

    <div v-if="communityStore.loading" class="text-center py-5">
      <div class="spinner-border text-primary"></div>
    </div>

    <template v-else>
      <EmptyState v-if="communityStore.users.length === 0" message="まだユーザーがいません。" />

      <template v-else>
        <!-- Favorites section -->
        <div v-if="favorites.length > 0" class="mb-3">
          <UserCard
            v-for="u in favorites"
            :key="u.id"
            :user="u"
            @toggle-favorite="handleToggleFavorite"
          />
        </div>

        <div v-if="favorites.length > 0 && others.length > 0" class="text-center my-3" style="color: #BDBDBD">
          <span>&#x2500;&#x2500;&#x2500; みんなの状況 &#x2500;&#x2500;&#x2500;</span>
        </div>

        <!-- Others section -->
        <div class="mb-3" style="display: flex; flex-direction: column; gap: 12px">
          <UserCard
            v-for="u in others"
            :key="u.id"
            :user="u"
            @toggle-favorite="handleToggleFavorite"
          />
        </div>

        <Pagination
          :total="communityStore.total"
          :per-page="communityStore.perPage"
          :current-page="communityStore.currentPage"
          @page-change="communityStore.fetchUsers"
        />
      </template>
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useCommunityStore } from '../stores/community'
import { useFavoriteStore } from '../stores/favorite'
import { useToast } from '../composables/useToast'
import UserCard from '../components/community/UserCard.vue'
import Pagination from '../components/common/Pagination.vue'
import EmptyState from '../components/common/EmptyState.vue'
import type { CommunityUser } from '../types'

const communityStore = useCommunityStore()
const favoriteStore = useFavoriteStore()
const toast = useToast()

onMounted(() => communityStore.fetchUsers())

const favorites = computed(() => communityStore.users.filter((u) => u.is_favorite))
const others = computed(() => communityStore.users.filter((u) => !u.is_favorite))

async function handleToggleFavorite(user: CommunityUser) {
  try {
    await favoriteStore.toggle(user.id, user.is_favorite)
  } catch {
    toast.show('エラーが発生しました', 'error')
  }
}
</script>
