<template>
  <div class="container py-4">
    <!-- Summary Cards -->
    <div class="row g-3 mb-4">
      <div class="col-4">
        <div class="card text-center shadow-sm">
          <div class="card-body py-2">
            <div class="fs-3 fw-bold" style="color: #1A73E8">{{ certCount }}</div>
            <div class="text-muted small">所持資格</div>
          </div>
        </div>
      </div>
      <div class="col-4">
        <div class="card text-center shadow-sm">
          <div class="card-body py-2">
            <div class="fs-3 fw-bold" style="color: #2E7D32">{{ passedCount }}</div>
            <div class="text-muted small">合格</div>
          </div>
        </div>
      </div>
      <div class="col-4">
        <div class="card text-center shadow-sm">
          <div class="card-body py-2">
            <div class="fs-3 fw-bold" style="color: #F57F17">{{ totalStudyHours }}</div>
            <div class="text-muted small">勉強時間(h)</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Add Button -->
    <div class="d-flex justify-content-end mb-3">
      <div class="dropdown" style="position: relative">
        <button class="btn btn-primary dropdown-toggle" type="button" @click="showDropdown = !showDropdown">
          + 新規登録
        </button>
        <ul v-if="showDropdown" class="dropdown-menu show" style="position: absolute; right: 0; top: 100%">
          <li><a class="dropdown-item" href="#" @click.prevent="openAddCert">資格を登録</a></li>
          <li><a class="dropdown-item" href="#" @click.prevent="openAddGoal">目標を追加</a></li>
        </ul>
      </div>
    </div>

    <!-- Certifications Section -->
    <div class="mb-4">
      <h5 class="mb-3">所持資格</h5>
      <div v-if="certStore.certifications.length === 0" class="text-muted text-center py-3">
        まだ資格が登録されていません
      </div>
      <CertCard
        v-for="cert in certStore.certifications"
        :key="cert.id"
        :cert="cert"
        @edit="openEditCert"
        @delete="confirmDeleteCert"
      />
    </div>

    <!-- Goals Section -->
    <div class="mb-4">
      <h5 class="mb-3">目標</h5>
      <div v-if="goalStore.goals.length === 0" class="text-muted text-center py-3">
        まだ目標が設定されていません
      </div>
      <template v-for="section in goalSections" :key="section.key">
        <div v-if="section.goals.length > 0" class="mb-3">
          <h6 class="d-flex align-items-center gap-2">
            {{ section.label }}
            <span class="badge bg-secondary">{{ section.goals.length }}</span>
          </h6>
          <GoalCard
            v-for="goal in section.goals"
            :key="goal.id"
            :goal="goal"
            @edit="openEditGoal"
            @delete="confirmDeleteGoal"
          />
        </div>
      </template>
    </div>

    <!-- Community Section -->
    <div class="mb-4">
      <h5 class="mb-3">コミュニティ</h5>
      <div v-if="communityStore.loading" class="text-center py-3">
        <div class="spinner-border spinner-border-sm text-primary"></div>
      </div>
      <template v-else>
        <div v-if="communityStore.users.length === 0" class="text-muted text-center py-3">
          まだユーザーがいません
        </div>
        <template v-else>
          <div v-if="favorites.length > 0" class="mb-2">
            <UserCard
              v-for="u in favorites"
              :key="u.id"
              :user="u"
              @toggle-favorite="handleToggleFavorite"
            />
          </div>
          <div v-if="favorites.length > 0 && others.length > 0" class="text-center my-2" style="color: #BDBDBD">
            <small>&#x2500;&#x2500;&#x2500; みんなの状況 &#x2500;&#x2500;&#x2500;</small>
          </div>
          <UserCard
            v-for="u in others"
            :key="u.id"
            :user="u"
            @toggle-favorite="handleToggleFavorite"
          />
          <Pagination
            :total="communityStore.total"
            :per-page="communityStore.perPage"
            :current-page="communityStore.currentPage"
            @page-change="communityStore.fetchUsers"
          />
        </template>
      </template>
    </div>

    <!-- Cert Form Modal -->
    <FormModal
      :show="showCertForm"
      :title="editingCert ? '資格を編集' : '資格を登録'"
      :submit-text="editingCert ? '更新' : '登録'"
      @close="showCertForm = false"
      @submit="handleCertSubmit"
    >
      <CertForm ref="certFormRef" :initial="certFormInitial" />
    </FormModal>

    <!-- Goal Form Modal -->
    <FormModal
      :show="showGoalForm"
      :title="editingGoal ? '目標を編集' : '目標を追加'"
      :submit-text="editingGoal ? '更新' : '追加'"
      @close="showGoalForm = false"
      @submit="handleGoalSubmit"
    >
      <GoalForm ref="goalFormRef" :initial="goalFormInitial" />
    </FormModal>

    <!-- Confirm Modal -->
    <ConfirmModal
      :show="showConfirm"
      :message="confirmMessage"
      @confirm="handleConfirmDelete"
      @cancel="showConfirm = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useCertificationStore } from '../stores/certification'
import { useGoalStore } from '../stores/goal'
import { useCommunityStore } from '../stores/community'
import { useFavoriteStore } from '../stores/favorite'
import { useToast } from '../composables/useToast'
import CertCard from '../components/certification/CertCard.vue'
import CertForm from '../components/certification/CertForm.vue'
import GoalCard from '../components/goal/GoalCard.vue'
import GoalForm from '../components/goal/GoalForm.vue'
import UserCard from '../components/community/UserCard.vue'
import Pagination from '../components/common/Pagination.vue'
import FormModal from '../components/common/FormModal.vue'
import ConfirmModal from '../components/common/ConfirmModal.vue'
import type { Certification, CertificationForm, Goal, GoalForm as GoalFormType, CommunityUser } from '../types'

const certStore = useCertificationStore()
const goalStore = useGoalStore()
const communityStore = useCommunityStore()
const favoriteStore = useFavoriteStore()
const toast = useToast()
const showDropdown = ref(false)

onMounted(() => {
  certStore.fetchAll()
  goalStore.fetchAll()
  communityStore.fetchUsers()
})

// Summary
const certCount = computed(() => certStore.certifications.length)
const passedCount = computed(() => goalStore.passedGoals.length)
const totalStudyHours = computed(() =>
  goalStore.goals.reduce((sum, g) => sum + (g.study_hours || 0), 0)
)

// Goal sections
const goalSections = computed(() => [
  { key: 'exam_date', label: '受験日', goals: goalStore.examDateGoals },
  { key: 'passed', label: '合格', goals: goalStore.passedGoals },
  { key: 'failed', label: '不合格', goals: goalStore.failedGoals },
  { key: 'abandoned', label: '断念', goals: goalStore.abandonedGoals },
])

// Community
const favorites = computed(() => communityStore.users.filter((u) => u.is_favorite))
const others = computed(() => communityStore.users.filter((u) => !u.is_favorite))

async function handleToggleFavorite(user: CommunityUser) {
  try {
    await favoriteStore.toggle(user.id, user.is_favorite)
  } catch {
    toast.show('エラーが発生しました', 'error')
  }
}

// === Cert CRUD ===
const showCertForm = ref(false)
const editingCert = ref(false)
const editingCertId = ref('')
const certFormRef = ref<InstanceType<typeof CertForm> | null>(null)
const certFormInitial = ref<CertificationForm | undefined>(undefined)

function openAddCert() {
  showDropdown.value = false
  editingCert.value = false
  certFormInitial.value = undefined
  showCertForm.value = true
}

function openEditCert(cert: Certification) {
  editingCert.value = true
  editingCertId.value = cert.id
  certFormInitial.value = {
    certification_name: cert.certification_name,
    master_id: cert.master_id,
    acquired_date: cert.acquired_date || '',
  }
  showCertForm.value = true
}

async function handleCertSubmit() {
  const form = certFormRef.value?.form
  if (!form || !form.certification_name) return
  try {
    if (editingCert.value) {
      await certStore.update(editingCertId.value, form)
      toast.show('資格を更新しました', 'success')
    } else {
      await certStore.add(form)
      toast.show('資格を登録しました', 'success')
    }
    showCertForm.value = false
  } catch (e: any) {
    toast.show(e.response?.data?.error || 'エラーが発生しました', 'error')
  }
}

// === Goal CRUD ===
const showGoalForm = ref(false)
const editingGoal = ref(false)
const editingGoalId = ref('')
const goalFormRef = ref<InstanceType<typeof GoalForm> | null>(null)
const goalFormInitial = ref<Partial<GoalFormType> | undefined>(undefined)

function openAddGoal() {
  showDropdown.value = false
  editingGoal.value = false
  goalFormInitial.value = undefined
  showGoalForm.value = true
}

function openEditGoal(goal: Goal) {
  editingGoal.value = true
  editingGoalId.value = goal.id
  goalFormInitial.value = {
    certification_name: goal.certification_name,
    master_id: goal.master_id,
    target_date: goal.target_date,
    status: goal.status,
    memo: goal.memo || '',
    study_hours: goal.study_hours || 0,
  }
  showGoalForm.value = true
}

async function handleGoalSubmit() {
  const form = goalFormRef.value?.form
  if (!form || !form.certification_name || !form.target_date) return
  try {
    if (editingGoal.value) {
      const prev = goalStore.goals.find((g) => g.id === editingGoalId.value)
      await goalStore.update(editingGoalId.value, {
        target_date: form.target_date,
        status: form.status,
        memo: form.memo,
        study_hours: form.study_hours,
      })
      if (prev && prev.status !== 'passed' && form.status === 'passed') {
        if (confirm('合格おめでとうございます！所持資格に追加しますか？')) {
          await certStore.add({
            certification_name: form.certification_name,
            master_id: form.master_id,
            acquired_date: new Date().toISOString().split('T')[0],
          })
          toast.show('所持資格にも追加しました', 'success')
        }
      }
      toast.show('目標を更新しました', 'success')
    } else {
      await goalStore.add(form)
      toast.show('目標を追加しました', 'success')
    }
    showGoalForm.value = false
  } catch (e: any) {
    toast.show(e.response?.data?.error || 'エラーが発生しました', 'error')
  }
}

// === Delete ===
const showConfirm = ref(false)
const confirmMessage = ref('')
const deletingType = ref<'cert' | 'goal'>('cert')
const deletingId = ref('')

function confirmDeleteCert(cert: Certification) {
  deletingType.value = 'cert'
  deletingId.value = cert.id
  confirmMessage.value = 'この資格を削除しますか？'
  showConfirm.value = true
}

function confirmDeleteGoal(goal: Goal) {
  deletingType.value = 'goal'
  deletingId.value = goal.id
  confirmMessage.value = 'この目標を削除しますか？'
  showConfirm.value = true
}

async function handleConfirmDelete() {
  try {
    if (deletingType.value === 'cert') {
      await certStore.remove(deletingId.value)
      toast.show('資格を削除しました', 'success')
    } else {
      await goalStore.remove(deletingId.value)
      toast.show('目標を削除しました', 'success')
    }
  } catch {
    toast.show('削除に失敗しました', 'error')
  }
  showConfirm.value = false
}
</script>
