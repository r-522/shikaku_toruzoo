<template>
  <div class="container py-4">
    <div v-if="communityStore.loading" class="text-center py-5">
      <div class="spinner-border text-primary"></div>
    </div>

    <template v-else-if="detail">
      <div class="d-flex align-items-center gap-2 mb-4">
        <router-link to="/community" class="btn btn-outline-secondary btn-sm">&larr; 戻る</router-link>
        <h3 class="mb-0">{{ detail.username }}</h3>
        <GoodMark :has-good-mark="detail.has_good_mark" />
      </div>

      <ul class="nav nav-tabs mb-3">
        <li class="nav-item">
          <button class="nav-link" :class="{ active: tab === 'certs' }" @click="tab = 'certs'">
            所持資格
          </button>
        </li>
        <li class="nav-item">
          <button class="nav-link" :class="{ active: tab === 'goals' }" @click="tab = 'goals'">
            目標
          </button>
        </li>
      </ul>

      <div v-if="tab === 'certs'">
        <div v-if="detail.certifications.length === 0" class="text-muted text-center py-4">
          所持資格はありません
        </div>
        <div v-for="cert in detail.certifications" :key="cert.id || String(Math.random())" class="card shadow-sm mb-2">
          <div class="card-body py-2">
            <div class="fw-bold">{{ getCertName(cert) }}</div>
            <small class="text-muted">{{ getCertDate(cert) }}</small>
          </div>
        </div>
      </div>

      <div v-if="tab === 'goals'">
        <div v-if="detail.goals.length === 0" class="text-muted text-center py-4">
          目標はありません
        </div>
        <div v-for="goal in detail.goals" :key="goal.id || String(Math.random())" class="card shadow-sm mb-2">
          <div class="card-body py-2">
            <div class="d-flex justify-content-between">
              <span class="fw-bold">{{ getGoalName(goal) }}</span>
              <StatusBadge :status="getGoalStatus(goal)" />
            </div>
            <small class="text-muted">目標日: {{ getGoalDate(goal) }}</small>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useCommunityStore } from '../stores/community'
import GoodMark from '../components/community/GoodMark.vue'
import StatusBadge from '../components/goal/StatusBadge.vue'
import type { GoalStatus } from '../types'

const route = useRoute()
const communityStore = useCommunityStore()
const tab = ref<'certs' | 'goals'>('certs')

const detail = ref(communityStore.userDetail)

onMounted(async () => {
  await communityStore.fetchUserDetail(route.params.id as string)
  detail.value = communityStore.userDetail
})

// Helper functions to handle both raw Supabase data and typed data
function getCertName(cert: any): string {
  return cert.certification_name || cert.TBL_MASTER?.masnm || ''
}

function getCertDate(cert: any): string {
  return cert.acquired_date || cert.holdt || '日付未設定'
}

function getGoalName(goal: any): string {
  return goal.certification_name || goal.TBL_MASTER?.masnm || ''
}

function getGoalStatus(goal: any): GoalStatus {
  return (goal.status || goal.goast || 'exam_date') as GoalStatus
}

function getGoalDate(goal: any): string {
  return goal.target_date || goal.goatd || ''
}
</script>
