<!-- ============================================================
views/SignInView.vue — サインイン（ログイン）画面
============================================================
このファイルはログイン画面を提供する。

【ログイン後の処理】
auth.signin() が成功すると Pinia ストアの user が設定され、
router.push('/dashboard') でダッシュボードへ遷移する。
サーバー側は session_token Cookie を発行するため、
以降のすべての API リクエストに Cookie が自動的に付与される。

【エラーメッセージについて】
「メールアドレスまたはパスワードが正しくありません」という表現は
セキュリティ上の意図的な設計。
「メールアドレスが間違っています」と教えると、
攻撃者がメールアドレス列挙に使える危険がある。
============================================================ -->
<template>
  <div class="container py-3 py-md-5">
    <div class="row justify-content-center">
      <div class="col-12 col-md-5">
        <div class="card shadow-sm">
          <div class="card-body p-4">
            <h4 class="text-center mb-4" style="color: #1A73E8">資格取るぞー！</h4>
            <h5 class="text-center mb-4">サインイン</h5>

            <form @submit.prevent="handleSubmit">
              <div class="mb-3">
                <label class="form-label">メールアドレス</label>
                <input v-model="form.email" type="email" class="form-control" required />
              </div>
              <div class="mb-3">
                <label class="form-label">パスワード</label>
                <input v-model="form.password" type="password" class="form-control" required />
              </div>

              <!-- 認証エラーメッセージ（どちらが間違っているか教えない） -->
              <div v-if="error" class="alert alert-danger">{{ error }}</div>

              <!-- ローディング中はボタンを無効化 -->
              <button type="submit" class="btn btn-primary w-100" :disabled="loading">
                {{ loading ? 'サインイン中...' : 'サインイン' }}
              </button>
            </form>

            <p class="text-center mt-3 mb-0">
              <router-link to="/signup">アカウントを作成する</router-link>
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const auth = useAuthStore()
const router = useRouter()

// フォームデータ（メール・パスワード）
const form = reactive({ email: '', password: '' })
// エラーメッセージ（単一の文字列: 詳細なフィールド別エラーは不要）
const error = ref('')
const loading = ref(false)

/**
 * ログインフォーム送信ハンドラ
 *
 * 成功 → ダッシュボードへ
 * 失敗 → エラーメッセージを表示（フィールド別エラーは不要）
 */
async function handleSubmit() {
  error.value = '' // 前回のエラーをクリア
  loading.value = true
  try {
    await auth.signin(form)
    router.push('/dashboard') // ログイン成功 → ダッシュボードへ
  } catch (e: any) {
    // API エラーのメッセージを取り出す
    // バックエンドが "メールアドレスまたはパスワードが正しくありません" を返す
    error.value = e.response?.data?.error || 'メールアドレスまたはパスワードが正しくありません'
  } finally {
    loading.value = false
  }
}
</script>
