<!-- ============================================================
views/DashboardView.vue — ダッシュボード画面
============================================================
このファイルはアプリのメイン画面。所持資格・目標・コミュニティをすべて表示する。

【このページの役割】
- 3 つのサマリーカード（所持資格数・合格数・総勉強時間）
- 所持資格一覧（CertCard）と CRUD モーダル
- 目標一覧（ステータス別セクション）と CRUD モーダル
- コミュニティ（お気に入り優先表示、ページネーション付き）

【ref を使った子コンポーネントへのアクセス】
`ref<InstanceType<typeof CertForm> | null>(null)` は
CertForm コンポーネントのインスタンスへの参照を持つ ref。
`certFormRef.value?.form` でコンポーネントが defineExpose した form データを取得できる。

【合格時の連携処理】
目標のステータスを "passed" に更新した場合、
confirm ダイアログで「所持資格にも追加しますか？」を確認し、
OK なら自動的に certStore.add() を呼ぶ連携処理を行う。

【コミュニティのお気に入り優先表示】
お気に入りユーザー（is_favorite: true）を上部に、
それ以外を下部に分けて表示する（computed でフィルタリング）。
============================================================ -->
<template>
  <div class="container py-4">
    <!-- サマリーカード（3 分割） -->
    <div class="row g-3 mb-4">
      <!-- 所持資格数 -->
      <div class="col-4">
        <div class="card text-center shadow-sm">
          <div class="card-body py-2 px-1 px-sm-3">
            <div class="fs-5 fs-sm-3 fw-bold" style="color: #1A73E8">{{ certCount }}</div>
            <div class="text-muted small text-truncate">所持資格</div>
          </div>
        </div>
      </div>
      <!-- 合格数 -->
      <div class="col-4">
        <div class="card text-center shadow-sm">
          <div class="card-body py-2 px-1 px-sm-3">
            <div class="fs-5 fs-sm-3 fw-bold" style="color: #2E7D32">{{ passedCount }}</div>
            <div class="text-muted small text-truncate">合格</div>
          </div>
        </div>
      </div>
      <!-- 総勉強時間 -->
      <div class="col-4">
        <div class="card text-center shadow-sm">
          <div class="card-body py-2 px-1 px-sm-3">
            <div class="fs-5 fs-sm-3 fw-bold" style="color: #F57F17">{{ totalStudyHours }}</div>
            <div class="text-muted small text-truncate">勉強時間(h)</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 新規登録ドロップダウンボタン -->
    <div class="d-flex justify-content-end mb-3">
      <div class="dropdown" style="position: relative">
        <!-- `showDropdown = !showDropdown` でドロップダウンの開閉をトグル -->
        <button class="btn btn-primary dropdown-toggle" type="button" @click="showDropdown = !showDropdown">
          + 新規登録
        </button>
        <!-- `v-if` でドロップダウンの表示/非表示を制御 -->
        <ul v-if="showDropdown" class="dropdown-menu show" style="position: absolute; right: 0; top: 100%">
          <li><a class="dropdown-item" href="#" @click.prevent="openAddCert">資格を登録</a></li>
          <li><a class="dropdown-item" href="#" @click.prevent="openAddGoal">目標を追加</a></li>
        </ul>
      </div>
    </div>

    <!-- 所持資格セクション -->
    <div class="mb-4">
      <h5 class="mb-3">所持資格</h5>
      <!-- 空の場合のメッセージ -->
      <div v-if="certStore.certifications.length === 0" class="text-muted text-center py-3">
        まだ資格が登録されていません
      </div>
      <!-- `v-for` で各資格カードを描画 -->
      <!-- `@edit` と `@delete` は CertCard コンポーネントが発火するイベントを受け取る -->
      <CertCard
        v-for="cert in certStore.certifications"
        :key="cert.id"
        :cert="cert"
        @edit="openEditCert"
        @delete="confirmDeleteCert"
      />
    </div>

    <!-- 目標セクション（ステータス別） -->
    <div class="mb-4">
      <h5 class="mb-3">目標</h5>
      <div v-if="goalStore.goals.length === 0" class="text-muted text-center py-3">
        まだ目標が設定されていません
      </div>
      <!-- goalSections はステータス別にグループ化した配列（computed） -->
      <!-- `<template v-for>` は実際の DOM 要素を生成しないグループ化用のタグ -->
      <template v-for="section in goalSections" :key="section.key">
        <!-- 目標が 1 件以上あるセクションのみ表示 -->
        <div v-if="section.goals.length > 0" class="mb-3">
          <h6 class="d-flex align-items-center gap-2">
            {{ section.label }}
            <!-- 件数バッジ -->
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

    <!-- コミュニティセクション -->
    <div class="mb-4">
      <h5 class="mb-3">コミュニティ</h5>
      <!-- ローディング中はスピナーを表示 -->
      <div v-if="communityStore.loading" class="text-center py-3">
        <!-- Bootstrap のスピナー（円形のローディングアニメーション） -->
        <div class="spinner-border spinner-border-sm text-primary"></div>
      </div>
      <!-- ローディング完了後 -->
      <template v-else>
        <div v-if="communityStore.users.length === 0" class="text-muted text-center py-3">
          まだユーザーがいません
        </div>
        <template v-else>
          <!-- お気に入りユーザーを上部に表示（is_favorite = true のユーザーのみ） -->
          <div v-if="favorites.length > 0" class="mb-2">
            <UserCard
              v-for="u in favorites"
              :key="u.id"
              :user="u"
              @toggle-favorite="handleToggleFavorite"
            />
          </div>
          <!-- お気に入りとその他の区切り線（両方存在する場合のみ） -->
          <div v-if="favorites.length > 0 && others.length > 0" class="text-center my-2" style="color: #BDBDBD">
            <!-- &#x2500; = ─（水平線文字） -->
            <small>&#x2500;&#x2500;&#x2500; みんなの状況 &#x2500;&#x2500;&#x2500;</small>
          </div>
          <!-- お気に入り以外のユーザー -->
          <UserCard
            v-for="u in others"
            :key="u.id"
            :user="u"
            @toggle-favorite="handleToggleFavorite"
          />
          <!-- ページネーション（総件数・1ページ件数・現在ページを渡す） -->
          <Pagination
            :total="communityStore.total"
            :per-page="communityStore.perPage"
            :current-page="communityStore.currentPage"
            @page-change="communityStore.fetchUsers"
          />
        </template>
      </template>
    </div>

    <!-- 所持資格フォームモーダル（追加/編集で共用） -->
    <FormModal
      :show="showCertForm"
      :title="editingCert ? '資格を編集' : '資格を登録'"
      :submit-text="editingCert ? '更新' : '登録'"
      @close="showCertForm = false"
      @submit="handleCertSubmit"
    >
      <!-- CertForm を <slot> に挿入する。ref で form データにアクセスできる -->
      <CertForm ref="certFormRef" :initial="certFormInitial" />
    </FormModal>

    <!-- 目標フォームモーダル -->
    <FormModal
      :show="showGoalForm"
      :title="editingGoal ? '目標を編集' : '目標を追加'"
      :submit-text="editingGoal ? '更新' : '追加'"
      @close="showGoalForm = false"
      @submit="handleGoalSubmit"
    >
      <GoalForm ref="goalFormRef" :initial="goalFormInitial" />
    </FormModal>

    <!-- 削除確認モーダル -->
    <ConfirmModal
      :show="showConfirm"
      :message="confirmMessage"
      @confirm="handleConfirmDelete"
      @cancel="showConfirm = false"
    />
  </div>
</template>

<script setup lang="ts">
// ref: 単一値、computed: 算出プロパティ、onMounted: コンポーネント表示時の処理
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
// GoalForm は types/index.ts の GoalForm と名前が衝突するため as でリネーム
import type { Certification, CertificationForm, Goal, GoalForm as GoalFormType, CommunityUser } from '../types'

// ---- ストアの取得 ----
const certStore = useCertificationStore()
const goalStore = useGoalStore()
const communityStore = useCommunityStore()
const favoriteStore = useFavoriteStore()
const toast = useToast()

// ドロップダウンの開閉状態
const showDropdown = ref(false)

// ---- ページ初期化 ----
// `onMounted` はコンポーネントが DOM にマウントされた直後に実行される
onMounted(() => {
  certStore.fetchAll()        // 所持資格を取得
  goalStore.fetchAll()        // 目標を取得
  communityStore.fetchUsers() // コミュニティユーザーを取得
})

// ---- サマリー計算（computed）----
const certCount = computed(() => certStore.certifications.length)
const passedCount = computed(() => goalStore.passedGoals.length)
// `reduce` で全目標の勉強時間を合計する（アキュムレーター パターン）
const totalStudyHours = computed(() =>
  goalStore.goals.reduce((sum, g) => sum + (g.study_hours || 0), 0)
)

// ---- 目標のステータス別グループ化 ----
const goalSections = computed(() => [
  { key: 'exam_date', label: '受験日',  goals: goalStore.examDateGoals },
  { key: 'passed',    label: '合格',    goals: goalStore.passedGoals },
  { key: 'failed',    label: '不合格',  goals: goalStore.failedGoals },
  { key: 'abandoned', label: '断念',    goals: goalStore.abandonedGoals },
])

// ---- コミュニティのお気に入り/その他の分離 ----
const favorites = computed(() => communityStore.users.filter((u) => u.is_favorite))
const others = computed(() => communityStore.users.filter((u) => !u.is_favorite))

/**
 * お気に入りトグルハンドラ
 */
async function handleToggleFavorite(user: CommunityUser) {
  try {
    await favoriteStore.toggle(user.id, user.is_favorite)
  } catch {
    toast.show('エラーが発生しました', 'error')
  }
}

// ================================================================
// 所持資格 CRUD
// ================================================================

const showCertForm = ref(false)
const editingCert = ref(false)   // true = 編集モード、false = 追加モード
const editingCertId = ref('')
// `ref<InstanceType<typeof CertForm> | null>(null)` で CertForm コンポーネントの参照
const certFormRef = ref<InstanceType<typeof CertForm> | null>(null)
const certFormInitial = ref<CertificationForm | undefined>(undefined)

/** 資格追加モーダルを開く */
function openAddCert() {
  showDropdown.value = false
  editingCert.value = false
  certFormInitial.value = undefined // 初期値なし（空フォーム）
  showCertForm.value = true
}

/** 資格編集モーダルを開く */
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

/** 資格フォームの送信（追加/更新） */
async function handleCertSubmit() {
  // `certFormRef.value?.form` で子コンポーネントの form データを取得
  const form = certFormRef.value?.form
  if (!form || !form.certification_name) return // バリデーション（資格名必須）
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

// ================================================================
// 目標 CRUD
// ================================================================

const showGoalForm = ref(false)
const editingGoal = ref(false)
const editingGoalId = ref('')
const goalFormRef = ref<InstanceType<typeof GoalForm> | null>(null)
const goalFormInitial = ref<Partial<GoalFormType> | undefined>(undefined)

/** 目標追加モーダルを開く */
function openAddGoal() {
  showDropdown.value = false
  editingGoal.value = false
  goalFormInitial.value = undefined
  showGoalForm.value = true
}

/** 目標編集モーダルを開く */
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

/** 目標フォームの送信（追加/更新） */
async function handleGoalSubmit() {
  const form = goalFormRef.value?.form
  if (!form || !form.certification_name || !form.target_date) return

  try {
    if (editingGoal.value) {
      // 更新前の目標を取得（ステータス変更の検出に使う）
      const prev = goalStore.goals.find((g) => g.id === editingGoalId.value)
      await goalStore.update(editingGoalId.value, {
        target_date: form.target_date,
        status: form.status,
        memo: form.memo,
        study_hours: form.study_hours,
      })

      // ステータスが「合格」に変更された場合: 所持資格への自動追加を確認
      // `prev.status !== 'passed'` で「今回初めて合格になった」場合のみ確認
      if (prev && prev.status !== 'passed' && form.status === 'passed') {
        if (confirm('合格おめでとうございます！所持資格に追加しますか？')) {
          await certStore.add({
            certification_name: form.certification_name,
            master_id: form.master_id,
            // `toISOString().split('T')[0]` で現在日付を YYYY-MM-DD 形式に変換
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

// ================================================================
// 削除確認
// ================================================================

const showConfirm = ref(false)
const confirmMessage = ref('')
const deletingType = ref<'cert' | 'goal'>('cert') // 削除対象の種別
const deletingId = ref('')

/** 資格削除の確認モーダルを開く */
function confirmDeleteCert(cert: Certification) {
  deletingType.value = 'cert'
  deletingId.value = cert.id
  confirmMessage.value = 'この資格を削除しますか？'
  showConfirm.value = true
}

/** 目標削除の確認モーダルを開く */
function confirmDeleteGoal(goal: Goal) {
  deletingType.value = 'goal'
  deletingId.value = goal.id
  confirmMessage.value = 'この目標を削除しますか？'
  showConfirm.value = true
}

/** 削除確認モーダルで「削除する」が押された時の処理 */
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
