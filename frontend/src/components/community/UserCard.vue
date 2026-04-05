<template>
  <div class="card shadow-sm mb-2">
    <div class="card-body py-3">
      <div class="d-flex justify-content-between align-items-center">
        <div class="d-flex align-items-center gap-2">
          <FavoriteStar :is-favorite="user.is_favorite" @toggle="$emit('toggleFavorite', user)" />
          <router-link :to="`/community/${user.id}`" class="fw-bold text-decoration-none text-dark">
            {{ user.username }}
          </router-link>
          <GoodMark :has-good-mark="user.has_good_mark" />
        </div>
        <div class="text-end small text-muted">
          <span class="me-3">資格 {{ user.certification_count }}</span>
          <span class="me-3">目標 {{ user.goal_count }}</span>
          <span>達成 {{ user.achieved_count }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import FavoriteStar from './FavoriteStar.vue'
import GoodMark from './GoodMark.vue'
import type { CommunityUser } from '../../types'

defineProps<{
  user: CommunityUser
}>()

defineEmits<{
  toggleFavorite: [user: CommunityUser]
}>()
</script>
