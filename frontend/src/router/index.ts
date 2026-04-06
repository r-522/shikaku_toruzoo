// ============================================================
// router/index.ts — Vue Router の設定
// ============================================================
// このファイルはシングルページアプリケーション（SPA）のルーティング設定を行う。
//
// 【Vue Router とは】
// URL のパス（/dashboard, /community/:id 等）に応じて
// 表示するコンポーネント（View）を切り替える仕組み。
// サーバーへのリクエストなしにページ遷移できる（SPA の核心）。
//
// 【ルートガード（beforeEach）について】
// 各ページへのアクセス前に認証状態を確認する。
// - 認証必要なページ（meta.requiresAuth: true）に未認証でアクセス → /signin へリダイレクト
// - 認証済みなのにサインインページへ → /dashboard へリダイレクト
//
// 【lazy loading（遅延読み込み）】
// `() => import(...)` の形式は「ページにアクセスした時だけ読み込む」設定。
// 初回読み込みを軽くするための最適化。

import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = createRouter({
  // `createWebHistory()` は HTML5 History API を使ったルーティング
  // URL が `/dashboard` のようにクリーンな形になる
  // （`createWebHashHistory()` の場合は `/#/dashboard` のようなハッシュ形式）
  history: createWebHistory(),
  routes: [
    // ルート `/` へのアクセスはダッシュボードへリダイレクト
    { path: '/', redirect: '/dashboard' },

    // サインアップ（認証不要 = requiresAuth: false）
    {
      path: '/signup',
      name: 'SignUp',
      component: () => import('../views/SignUpView.vue'), // 遅延読み込み
      meta: { requiresAuth: false }
    },

    // サインイン（認証不要）
    {
      path: '/signin',
      name: 'SignIn',
      component: () => import('../views/SignInView.vue'),
      meta: { requiresAuth: false }
    },

    // ダッシュボード（認証必要）
    {
      path: '/dashboard',
      name: 'Dashboard',
      component: () => import('../views/DashboardView.vue'),
      meta: { requiresAuth: true }
    },

    // 旧URLへのアクセスをダッシュボードへリダイレクト（後方互換性）
    { path: '/certifications', redirect: '/dashboard' },
    { path: '/goals', redirect: '/dashboard' },
    { path: '/community', redirect: '/dashboard' },

    // ユーザー詳細（コミュニティ機能。`:id` はパスパラメータ）
    {
      path: '/community/:id',
      name: 'UserDetail',
      component: () => import('../views/UserDetailView.vue'),
      meta: { requiresAuth: true }
    },
  ],
})

// 初期化フラグ: 最初の 1 回だけ fetchMe（認証状態の確認）を実行するため
let initialized = false

// ---- ルートガード: すべてのページ遷移前に実行される ----
// `to`: 遷移先のルート情報
router.beforeEach(async (to) => {
  // Pinia ストアを取得（ルーターのコンテキスト外でもアクセスできる）
  const auth = useAuthStore()

  // 最初のアクセス時のみ「現在のセッションが有効かどうか」を確認する
  // （ページをリロードした後もログイン状態を維持するため）
  if (!initialized) {
    await auth.fetchMe() // Cookie のセッションで認証状態を確認
    initialized = true
  }

  // 認証が必要なページに未認証でアクセスした場合 → サインインページへ
  if (to.meta.requiresAuth && !auth.isAuthenticated) {
    return '/signin' // 文字列を返すとそのパスへリダイレクトする
  }

  // 認証済みなのにサインイン/サインアップペー���へのアクセス → ダッシュボードへ
  if (!to.meta.requiresAuth && auth.isAuthenticated && (to.name === 'SignIn' || to.name === 'SignUp')) {
    return '/dashboard'
  }

  // 上記に該当しない場合はそのまま遷移を許可（undefined を返す = 遷移許可）
})

export default router
