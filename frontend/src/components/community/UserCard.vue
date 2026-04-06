<template>
  <div class="card shadow-sm mb-2">
    <div class="card-body py-3">
      <!-- Header -->
      <div class="d-flex justify-content-between align-items-center mb-2">
        <div class="d-flex align-items-center gap-2">
          <FavoriteStar :is-favorite="user.is_favorite" @toggle="$emit('toggleFavorite', user)" />
          <router-link :to="`/community/${user.id}`" class="fw-bold text-decoration-none text-dark">
            {{ user.username }}
          </router-link>
          <GoodMark :has-good-mark="user.has_good_mark" />
        </div>
        <div class="small fw-bold" style="color: #1A73E8">
          合計 {{ user.total_study_hours }}h
        </div>
      </div>

      <!-- Certifications -->
      <div v-if="user.certifications.length > 0" class="ps-4 mb-1">
        <div class="text-muted small fw-bold mb-1">所持資格</div>
        <div
          v-for="(cert, idx) in user.certifications"
          :key="'c' + idx"
          class="d-flex align-items-center gap-2 small"
          style="line-height: 1.7"
        >
          <span class="badge" style="background-color: #2E7D32">取得</span>
          <span>{{ cert.certification_name }}</span>
          <span v-if="cert.acquired_date" class="text-muted">({{ cert.acquired_date }})</span>
        </div>
      </div>

      <!-- Goals -->
      <div v-if="user.goals.length > 0" class="ps-4">
        <div class="text-muted small fw-bold mb-1">目標</div>
        <div
          v-for="(goal, idx) in user.goals"
          :key="'g' + idx"
          class="d-flex align-items-center gap-2 small"
          style="line-height: 1.7"
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
