<!-- ============================================================
views/GoalView.vue — 目標専用画面
============================================================
このファイルは学習目標の一覧・追加・編集・削除を行う画面。

【ルーティングとの関係】
`/goals` は `/dashboard` にリダイレクトされるため（router/index.ts）、
現時点では直接アクセスされることはない。
将来的に独立した目標管理ページとして使える状態で実装されている。

【ステータス別セクション表示】
ゴールストアの computed（examDateGoals, passedGoals 等）を使って
ステータス別にグループ化して表示する。
セクション設定は `sections` computed で管理し、
`v-for` で動的に描画する。

【合格時の所持資格への自動追加】
ステータスを "passed" に更新した際に
「所持資格に追加しますか？」を確認して自動追加する処理が含まれる。
certStore との連携がある点がCertificationView との違い。
============================================================ -->
<template>
  <div class="container py-4">
    <!-- ヘッダー: タイトルと新規追加ボタン -->
    <div class="d-flex justify-content-between align-items-center mb-4">
      <h3>目標</h3>
      <button class="btn btn-primary" @click="openAdd">+ 新規追加</button>
    </div>

    <!-- ローディング中はスピナーを表示 -->
    <div v-if="goalStore.loading" class="text-center py-5">
      <div class="spinner-border text-primary"></div>
    </div>

    <!-- ローディング完了後 -->
    <template v-else>
      <!-- 目標が 0 件の場合 -->
      <EmptyState v-if="goalStore.goals.length === 0" message="まだ目標が設定されていません。" />

      <!-- ステータス別セクション -->
      <template v-for="section in sections" :key="section.key">
        <!-- そのステータスの目標が 1 件以上ある場合のみ表示 -->
        <div v-if="section.goals.length > 0" class="mb-4">
          <!-- セクションタイトルと件数バッジ -->
          <h5 class="d-flex align-items-center gap-2">
            {{ section.label }}
            <span class="badge bg-secondary">{{ section.goals.length }}</span>
          </h5>
          <!-- 各目標カード -->
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

    <!-- 目標フォームモーダル（追加/編集を切り替え） -->
    <FormModal
      :show="showForm"
      :title="editing ? '目標を編集' : '目標を追加'"
      :submit-text="editing ? '更新' : '追加'"
      @close="showForm = false"
      @submit="handleSubmit"
    >
      <GoalForm ref="goalFormRef" :initial="formInitial" />
    </FormModal>

    <!-- 削除確認モーダル -->
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
// 合格時に所持資格に自動追加するためにインポート
import { useCertificationStore } from '../stores/certification'
import { useToast } from '../composables/useToast'
import GoalCard from '../components/goal/GoalCard.vue'
import GoalForm from '../components/goal/GoalForm.vue'
import FormModal from '../components/common/FormModal.vue'
import ConfirmModal from '../components/common/ConfirmModal.vue'
import EmptyState from '../components/common/EmptyState.vue'
// GoalForm は types/index.ts と名前が衝突するため `as` でリネーム
import type { Goal, GoalForm as GoalFormType } from '../types'

const goalStore = useGoalStore()
const certStore = useCertificationStore()
const toast = useToast()

// ---- UI 状態 ----
const showForm = ref(false)
const showConfirm = ref(false)
const editing = ref(false)       // true = 編集モード
const editingId = ref('')
const deletingId = ref('')
const goalFormRef = ref<InstanceType<typeof GoalForm> | null>(null)
const formInitial = ref<Partial<GoalFormType> | undefined>(undefined)

// ページ表示時に目標一覧を取得
onMounted(() => goalStore.fetchAll())

// ステータス別セクション設定（computed でストアの値と連動）
const sections = computed(() => [
  { key: 'exam_date', label: '受験日',  goals: goalStore.examDateGoals },
  { key: 'passed',    label: '合格',    goals: goalStore.passedGoals },
  { key: 'failed',    label: '不合格',  goals: goalStore.failedGoals },
  { key: 'abandoned', label: '断念',    goals: goalStore.abandonedGoals },
])

/** 追加モーダルを開く */
function openAdd() {
  editing.value = false
  formInitial.value = undefined
  showForm.value = true
}

/** 編集モーダルを開く */
function openEdit(goal: Goal) {
  editing.value = true
  editingId.value = goal.id
  formInitial.value = {
    certification_name: goal.certification_name,
    master_id: goal.master_id,
    target_date: goal.target_date,
    status: goal.status,
    memo: goal.memo || '',
    study_hours: goal.study_hours || 0,
  }
  showForm.value = true
}

/** 削除確認モーダルを開く */
function confirmDelete(goal: Goal) {
  deletingId.value = goal.id
  showConfirm.value = true
}

/** 目標フォームの送信（追加/更新） */
async function handleSubmit() {
  const form = goalFormRef.value?.form
  if (!form || !form.certification_name || !form.target_date) return

  try {
    if (editing.value) {
      // 更新前の目標データ（ステータス変化の検出に使う）
      const prev = goalStore.goals.find((g) => g.id === editingId.value)
      await goalStore.update(editingId.value, {
        target_date: form.target_date,
        status: form.status,
        memo: form.memo,
        study_hours: form.study_hours,
      })

      // 「不合格/断念 → 合格」に変更された場合: 所持資格への追加を確認
      if (prev && prev.status !== 'passed' && form.status === 'passed') {
        if (confirm('合格おめでとうございます！所持資格に追加しますか？')) {
          await certStore.add({
            certification_name: form.certification_name,
            master_id: form.master_id,
            // 現在の日付を YYYY-MM-DD 形式で取得
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

/** 目標の削除実行 */
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
