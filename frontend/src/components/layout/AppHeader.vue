<!-- ============================================================
components/layout/AppHeader.vue — ナビゲーションバー
============================================================
このファイルはアプリ全体に表示されるナビゲーションバーを定義する。

【表示条件】
- `v-if="auth.isAuthenticated"`: 認証済みの場合のみユーザー名とサインアウトボタンを表示
- 未認証の場合（サインイン・サインアップページ）はブランドロゴのみ

【Bootstrap クラスの補足】
- navbar-dark: ダークテーマ（テキストとアイコンを白に）
- navbar-expand-lg: lg（960px 以上）でナビゲーション項目を展開
- ms-auto: margin-start: auto（右寄せ）
============================================================ -->
<template>
  <!-- Bootstrap のナビゲーションバー。背景色はブランドカラー -->
  <nav class="navbar navbar-expand-lg navbar-dark" style="background-color: #1A73E8">
    <div class="container">
      <!-- ブランドロゴ（クリックでダッシュボードへ）
           router-link は <a> タグに変換されるが、ページリロードなしで遷移する -->
      <router-link class="navbar-brand fw-bold" to="/dashboard">資格取るぞー！</router-link>

      <!-- 認証済みの場合のみユーザー名とサインアウトボタンを表示 -->
      <!-- `v-if` は条件が true の場合のみ DOM にレンダリングする -->
      <!-- min-width:0: flex アイテムとして縮小可能にし、長いユーザー名でのナビバー崩れを防ぐ -->
      <div v-if="auth.isAuthenticated" class="d-flex align-items-center ms-auto" style="min-width: 0">
        <!-- text-truncate + min-width:0: ユーザー名が長くても省略表示（ボタンは常に表示） -->
        <span class="text-white me-2 text-truncate" style="min-width: 0">{{ auth.user?.username }}</span>
        <!-- flex-shrink-0: サインアウトボタンは縮小しない -->
        <button class="btn btn-outline-light btn-sm flex-shrink-0" @click="handleSignout">
          サインアウト
        </button>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { useAuthStore } from '../../stores/auth'
import { useRouter } from 'vue-router'

// Pinia の auth ストアを取得（ユーザー情報・認証状態を参照）
const auth = useAuthStore()
// プログラム的なページ遷移のために router を取得
const router = useRouter()

/**
 * サインアウトしてサインインページへリダイレクトする
 */
async function handleSignout() {
  // auth.signout(): API でセッション削除 + ストアの user を null に
  await auth.signout()
  // サインインページへ遷移
  router.push('/signin')
}
</script>
