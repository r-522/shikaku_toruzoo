<template>
  <div class="card shadow-sm mb-2">
    <div class="card-body py-3">
      <div class="d-flex justify-content-between align-items-center mb-2">
        <div class="d-flex align-items-center gap-2">
          <FavoriteStar :is-favorite="user.is_favorite" @toggle="$emit('toggleFavorite', user)" />
          <router-link :to="`/community/${user.id}`" class="fw-bold text-decoration-none text-dark">
            {{ user.username }}
          </router-link>
          <GoodMark :has-good-mark="user.has_good_mark" />
        </div>
        <div class="text-end small text-muted">
          <span class="me-2">資格 {{ user.certification_count }}</span>
          <span class="me-2">合格 {{ user.achieved_count }}</span>
          <span>計 {{ user.total_study_hours }}h</span>
        </div>
      </div>
      <div v-if="user.goals.length > 0" class="ps-4">
        <div
          v-for="(goal, idx) in user.goals"
          :key="idx"
          class="d-flex align-items-center gap-2 small"
          style="line-height: 1.8"
        >
          <StatusBadge :status="goal.status" />
          <span>{{ goal.certification_name }}</span>
          <span v-if="goal.study_hours > 0" class="text-muted">({{ goal.study_hours }}h)</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import FavoriteStar from './FavoriteStar.vue'
import GoodMark from './GoodMark.vue'
import StatusBadge from '../goal/StatusBadge.vue'
import type { CommunityUser } from '../../types'

defineProps<{
  user: CommunityUser
}>()

defineEmits<{
  toggleFavorite: [user: CommunityUser]
}>()
</script>
