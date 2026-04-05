<template>
  <div class="container py-4">
    <div class="d-flex justify-content-between align-items-center mb-4">
      <h3>目標</h3>
      <button class="btn btn-primary" @click="openAdd">+ 新規追加</button>
    </div>

    <div v-if="goalStore.loading" class="text-center py-5">
      <div class="spinner-border text-primary"></div>
    </div>

    <template v-else>
      <EmptyState v-if="goalStore.goals.length === 0" message="まだ目標が設定されていません。" />

      <template v-for="section in sections" :key="section.key">
        <div v-if="section.goals.length > 0" class="mb-4">
          <h5 class="d-flex align-items-center gap-2">
            {{ section.label }}
            <span class="badge bg-secondary">{{ section.goals.length }}</span>
          </h5>
          <GoalCard
            v-for="goal in section.goals"
            :key="goal.id"
            :goal="goal"
            @edit="openEdit"
            @delete="confirmDelete"
          />
        </div>
      </template>
    </template>

    <FormModal
      :show="showForm"
      :title="editing ? '目標を編集' : '目標を追加'"
      :submit-text="editing ? '更新' : '追加'"
      @close="showForm = false"
      @submit="handleSubmit"
    >
      <GoalForm ref="goalFormRef" :initial="formInitial" />
    </FormModal>

    <ConfirmModal
      :show="showConfirm"
      message="この目標を削除しますか？"
      @confirm="handleDelete"
      @cancel="showConfirm = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useGoalStore } from '../stores/goal'
import { useCertificationStore } from '../stores/certification'
import { useToast } from '../composables/useToast'
import GoalCard from '../components/goal/GoalCard.vue'
import GoalForm from '../components/goal/GoalForm.vue'
import FormModal from '../components/common/FormModal.vue'
import ConfirmModal from '../components/common/ConfirmModal.vue'
import EmptyState from '../components/common/EmptyState.vue'
import type { Goal, GoalForm as GoalFormType } from '../types'

const goalStore = useGoalStore()
const certStore = useCertificationStore()
const toast = useToast()

const showForm = ref(false)
const showConfirm = ref(false)
const editing = ref(false)
const editingId = ref('')
const deletingId = ref('')
const goalFormRef = ref<InstanceType<typeof GoalForm> | null>(null)
const formInitial = ref<Partial<GoalFormType> | undefined>(undefined)

onMounted(() => goalStore.fetchAll())

const sections = computed(() => [
  { key: 'studying', label: '学習中', goals: goalStore.studyingGoals },
  { key: 'scheduled', label: '受験予定', goals: goalStore.scheduledGoals },
  { key: 'achieved', label: '達成', goals: goalStore.achievedGoals },
  { key: 'suspended', label: '中断', goals: goalStore.suspendedGoals },
])

function openAdd() {
  editing.value = false
  formInitial.value = undefined
  showForm.value = true
}

function openEdit(goal: Goal) {
  editing.value = true
  editingId.value = goal.id
  formInitial.value = {
    certification_name: goal.certification_name,
    master_id: goal.master_id,
    target_date: goal.target_date,
    status: goal.status,
    memo: goal.memo || '',
  }
  showForm.value = true
}

function confirmDelete(goal: Goal) {
  deletingId.value = goal.id
  showConfirm.value = true
}

async function handleSubmit() {
  const form = goalFormRef.value?.form
  if (!form || !form.certification_name || !form.target_date) return

  try {
    if (editing.value) {
      const prev = goalStore.goals.find((g) => g.id === editingId.value)
      await goalStore.update(editingId.value, {
        target_date: form.target_date,
        status: form.status,
        memo: form.memo,
      })
      // If status changed to achieved, offer to add to holdings
      if (prev && prev.status !== 'achieved' && form.status === 'achieved') {
        if (confirm('達成おめでとうございます！所持資格に追加しますか？')) {
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
    showForm.value = false
  } catch (e: any) {
    toast.show(e.response?.data?.error || 'エラーが発生しました', 'error')
  }
}

async function handleDelete() {
  try {
    await goalStore.remove(deletingId.value)
    toast.show('目標を削除しました', 'success')
  } catch {
    toast.show('削除に失敗しました', 'error')
  }
  showConfirm.value = false
}
</script>
