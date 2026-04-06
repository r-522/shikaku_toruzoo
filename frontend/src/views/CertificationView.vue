<!-- ============================================================
views/CertificationView.vue — 所持資格専用画面
============================================================
このファイルは所持資格の一覧・追加・編集・削除を行う画面。

【ダッシュボードとの関係】
このビューは `/certifications` パスから `/dashboard` にリダイレクトされるため、
現時点では直接アクセスされることはない（router/index.ts 参照）。
将来的に独立した資格管理ページとして使える状態で実装されている。

【CertificationView が DashboardView と異なる点】
- コミュニティセクションがない
- 目標セクションがない
- 資格管理のみに特化したシンプルな画面

【ローディング中のスピナー表示】
`certStore.loading` が true の間は spinner-border を表示し、
データ取得完了後に `v-else` でコンテンツを表示する。
============================================================ -->
<template>
  <div class="container py-4">
    <!-- ヘッダー: タイトルと新規登録ボタン -->
    <div class="d-flex justify-content-between align-items-center mb-4">
      <h3>所持資格</h3>
      <button class="btn btn-primary" @click="openAdd">+ 新規登録</button>
    </div>

    <!-- ローディング中はスピナーを表示 -->
    <div v-if="certStore.loading" class="text-center py-5">
      <div class="spinner-border text-primary"></div>
    </div>

    <!-- ローディング完了後 -->
    <template v-else>
      <!-- 空の場合のメッセージ -->
      <EmptyState v-if="certStore.certifications.length === 0" message="まだ資格が登録されていません。" />
      <!-- 資格カードのリスト -->
      <CertCard
        v-for="cert in certStore.certifications"
        :key="cert.id"
        :cert="cert"
        @edit="openEdit"
        @delete="confirmDelete"
      />
    </template>

    <!-- 追加/編集フォームモーダル（editing で追加/編集を切り替え） -->
    <FormModal
      :show="showForm"
      :title="editing ? '資格を編集' : '資格を登録'"
      :submit-text="editing ? '更新' : '登録'"
      @close="showForm = false"
      @submit="handleSubmit"
    >
      <CertForm ref="certFormRef" :initial="formInitial" />
    </FormModal>

    <!-- 削除確認モーダル -->
    <ConfirmModal
      :show="showConfirm"
      message="この資格を削除しますか？"
      @confirm="handleDelete"
      @cancel="showConfirm = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useCertificationStore } from '../stores/certification'
import { useToast } from '../composables/useToast'
import CertCard from '../components/certification/CertCard.vue'
import CertForm from '../components/certification/CertForm.vue'
import FormModal from '../components/common/FormModal.vue'
import ConfirmModal from '../components/common/ConfirmModal.vue'
import EmptyState from '../components/common/EmptyState.vue'
import type { Certification, CertificationForm } from '../types'

const certStore = useCertificationStore()
const toast = useToast()

// ---- UI 状態 ----
const showForm = ref(false)        // フォームモーダルの表示/非表示
const showConfirm = ref(false)     // 削除確認モーダルの表示/非表示
const editing = ref(false)         // true = 編集モード
const editingId = ref('')          // 編集中のID
const deletingId = ref('')         // 削除対象のID
// CertForm コンポーネントへの参照（form データ取得に使う）
const certFormRef = ref<InstanceType<typeof CertForm> | null>(null)
// フォームの初期値（編集時は資格データ、追加時は undefined）
const formInitial = ref<CertificationForm | undefined>(undefined)

// ページ表示時に資格一覧を取得
onMounted(() => certStore.fetchAll())

/** 追加モーダルを開く */
function openAdd() {
  editing.value = false
  formInitial.value = undefined // 空フォームで開く
  showForm.value = true
}

/** 編集モーダルを開く（既存の資格データを初期値として設定） */
function openEdit(cert: Certification) {
  editing.value = true
  editingId.value = cert.id
  formInitial.value = {
    certification_name: cert.certification_name,
    master_id: cert.master_id,
    acquired_date: cert.acquired_date || '',
  }
  showForm.value = true
}

/** 削除確認モーダルを開く */
function confirmDelete(cert: Certification) {
  deletingId.value = cert.id
  showConfirm.value = true
}

/** フォーム送信（追加/更新） */
async function handleSubmit() {
  const form = certFormRef.value?.form
  if (!form || !form.certification_name) return // 資格名は必須

  try {
    if (editing.value) {
      await certStore.update(editingId.value, form)
      toast.show('資格を更新しました', 'success')
    } else {
      await certStore.add(form)
      toast.show('資格を登録しました', 'success')
    }
    showForm.value = false
  } catch (e: any) {
    toast.show(e.response?.data?.error || 'エラーが発生しました', 'error')
  }
}

/** 削除実行（確認モーダルで「削除する」を押した後） */
async function handleDelete() {
  try {
    await certStore.remove(deletingId.value)
    toast.show('資格を削除しました', 'success')
  } catch (e: any) {
    toast.show('削除に失敗しました', 'error')
  }
  showConfirm.value = false
}
</script>
