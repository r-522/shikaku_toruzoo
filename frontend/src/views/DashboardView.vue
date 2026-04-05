<template>
  <div class="container py-4">
    <h3 class="mb-4">ダッシュボード</h3>

    <div class="row g-3 mb-4">
      <div class="col-md-4">
        <div class="card text-center shadow-sm">
          <div class="card-body">
            <div class="fs-2 fw-bold" style="color: #1A73E8">{{ certCount }}</div>
            <div class="text-muted small">所持資格数</div>
          </div>
        </div>
      </div>
      <div class="col-md-4">
        <div class="card text-center shadow-sm">
          <div class="card-body">
            <div class="fs-2 fw-bold" style="color: #2E7D32">{{ achievedCount }}</div>
            <div class="text-muted small">達成目標数</div>
          </div>
        </div>
      </div>
      <div class="col-md-4">
        <div class="card text-center shadow-sm">
          <div class="card-body">
            <div class="fs-2 fw-bold" style="color: #F57F17">{{ activeGoalCount }}</div>
            <div class="text-muted small">進行中目標数</div>
          </div>
        </div>
      </div>
    </div>

    <div class="row g-3">
      <div class="col-md-6">
        <div class="card shadow-sm">
          <div class="card-header d-flex justify-content-between align-items-center">
            <span>最近の所持資格</span>
            <router-link to="/certifications" class="text-decoration-none small">もっと見る</router-link>
          </div>
          <ul class="list-group list-group-flush">
            <li v-for="cert in recentCerts" :key="cert.id" class="list-group-item">
              <div class="fw-bold">{{ cert.certification_name }}</div>
              <small class="text-muted">{{ cert.acquired_date || '日付未設定' }}</small>
            </li>
            <li v-if="recentCerts.length === 0" class="list-group-item text-muted text-center">
              まだ資格が登録されていません
            </li>
          </ul>
        </div>
      </div>
      <div class="col-md-6">
        <div class="card shadow-sm">
          <div class="card-header d-flex justify-content-between align-items-center">
            <span>進行中の目標</span>
            <router-link to="/goals" class="text-decoration-none small">もっと見る</router-link>
          </div>
          <ul class="list-group list-group-flush">
            <li v-for="goal in activeGoals" :key="goal.id" class="list-group-item">
              <div class="d-flex justify-content-between">
                <span class="fw-bold">{{ goal.certification_name }}</span>
                <StatusBadge :status="goal.status" />
              </div>
              <small class="text-muted">目標日: {{ goal.target_date }}</small>
            </li>
            <li v-if="activeGoals.length === 0" class="list-group-item text-muted text-center">
              まだ目標が設定されていません
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useCertificationStore } from '../stores/certification'
import { useGoalStore } from '../stores/goal'
import StatusBadge from '../components/goal/StatusBadge.vue'

const certStore = useCertificationStore()
const goalStore = useGoalStore()

onMounted(() => {
  certStore.fetchAll()
  goalStore.fetchAll()
})

const certCount = computed(() => certStore.certifications.length)
const achievedCount = computed(() => goalStore.achievedGoals.length)
const activeGoalCount = computed(() => goalStore.studyingGoals.length + goalStore.scheduledGoals.length)
const recentCerts = computed(() => certStore.certifications.slice(0, 3))
const activeGoals = computed(() =>
  goalStore.goals
    .filter((g) => g.status === 'studying' || g.status === 'scheduled')
    .slice(0, 3)
)
</script>
