<!-- ============================================================
views/SignUpView.vue — サインアップ（新規登録）画面
============================================================
このファイルは新規ユーザー登録の画面を提供する。

【バリデーションの二層構造】
1. フロントエンド側（validate 関数）: ユーザー名・メール・パスワードの形式チェック
   → 素早いフィードバックでサーバーへの無駄なリクエストを減らす
2. バックエンド側（auth_service.rs）: 重複チェック・複雑性チェック
   → 最終的な検証はサーバーが担保する

【エラー表示パターン】
`errors` は `Record<string, string>` 型のリアクティブオブジェクト。
キーはフィールド名（'username', 'email', 'password', 'general'）で
値がエラーメッセージ。`v-if="errors.xxx"` で該当フィールドのエラーだけ表示する。

【try/catch/finally パターン】
- try: API 呼び出しを実行
- catch: エラーレスポンスのメッセージを取り出して表示
- finally: ローディング状態を確実に解除（成功/失敗に関わらず）
============================================================ -->
<template>
  <div class="container py-5">
    <!-- 中央揃えの 1 列レイアウト（Bootstrap の行コラムシステム） -->
    <div class="row justify-content-center">
      <div class="col-md-5">
        <div class="card shadow-sm">
          <div class="card-body p-4">
            <!-- ブランド名 -->
            <h4 class="text-center mb-4" style="color: #1A73E8">資格取るぞー！</h4>
            <h5 class="text-center mb-4">アカウント作成</h5>

            <!-- `@submit.prevent` でページリロードなしにフォーム送信を処理 -->
            <form @submit.prevent="handleSubmit">
              <!-- ユーザー名フィールド -->
              <div class="mb-3">
                <label class="form-label">ユーザー名</label>
                <input v-model="form.username" type="text" class="form-control" required />
                <!-- `v-if="errors.username"` でエラーがある時だけ表示 -->
                <div v-if="errors.username" class="text-danger small mt-1">{{ errors.username }}</div>
              </div>

              <!-- メールアドレスフィールド -->
              <div class="mb-3">
                <label class="form-label">メールアドレス</label>
                <input v-model="form.email" type="email" class="form-control" required />
                <div v-if="errors.email" class="text-danger small mt-1">{{ errors.email }}</div>
              </div>

              <!-- パスワードフィールド -->
              <div class="mb-3">
                <label class="form-label">パスワード</label>
                <input v-model="form.password" type="password" class="form-control" required />
                <div v-if="errors.password" class="text-danger small mt-1">{{ errors.password }}</div>
              </div>

              <!-- 全般エラー（重複メール等） -->
              <div v-if="errors.general" class="alert alert-danger">{{ errors.general }}</div>

              <!-- 送信ボタン（ローディング中は無効化してテキストを変える） -->
              <button type="submit" class="btn btn-primary w-100" :disabled="loading">
                <!-- 三項演算子でローディング中のラベルを切り替え -->
                {{ loading ? '作成中...' : 'サインアップ' }}
              </button>
            </form>

            <!-- サインインページへのリンク -->
            <p class="text-center mt-3 mb-0">
              <router-link to="/signin">アカウントをお持ちの方はこちら</router-link>
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// reactive: オブジェクト、ref: 単一値のリアクティブ変数
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useToast } from '../composables/useToast'

const auth = useAuthStore()
const router = useRouter()
const toast = useToast()

// フォームデータ（reactive でオブジェクト全体をリアクティブ化）
const form = reactive({ username: '', email: '', password: '' })
// エラーメッセージ（キー: フィールド名、値: エラーメッセージ）
const errors = reactive<Record<string, string>>({})
const loading = ref(false)

/**
 * フロントエンド側のバリデーション
 *
 * @returns 全てのバリデーションが通れば true、失敗があれば false
 */
function validate(): boolean {
  // 既存のエラーをクリア（forEach でキーを列挙して delete）
  Object.keys(errors).forEach((k) => delete errors[k])

  if (form.username.length < 3 || form.username.length > 30) {
    errors.username = 'ユーザー名は3〜30文字で入力してください'
  }
  // `includes('@')` は最低限の形式チェック（厳密なメール検証はサーバー側が担当）
  if (!form.email.includes('@')) {
    errors.email = '有効なメールアドレスを入力してください'
  }
  if (form.password.length < 8) {
    errors.password = 'パスワードは8文字以上で入力してください'
  }

  // errors オブジェクトのキーが 0 個 = エラーなし
  return Object.keys(errors).length === 0
}

/**
 * フォーム送信ハンドラ
 */
async function handleSubmit() {
  // バリデーション失敗なら API を呼ばずに終了
  if (!validate()) return

  loading.value = true
  try {
    await auth.signup(form)
    toast.show('アカウントが作成されました', 'success')
    // 登録成功後はサインインページへ（自動ログインはしない）
    router.push('/signin')
  } catch (e: any) {
    // `e.response?.data?.error` は Axios のエラーオブジェクトからメッセージを取り出す
    // Axios エラーの構造: { response: { data: { error: "メッセージ" } } }
    errors.general = e.response?.data?.error || 'エラーが発生しました'
  } finally {
    loading.value = false
  }
}
</script>
