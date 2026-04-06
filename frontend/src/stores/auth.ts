// ============================================================
// stores/auth.ts — 認証状態管理ストア
// ============================================================
// このファイルは Pinia を使って認証状態（ログイン中ユーザー情報）を管理する。
//
// 【Pinia とは】
// Vue 3 向けの状態管理ライブラリ。
// コンポーネントをまたいで「同じデータ」を共有するために使う。
// 例: ヘッダーとダッシュボードが同じ「ログイン中ユーザー名」を参照できる。
//
// 【defineStore の第 2 引数（Composition API スタイル）】
// `ref()` でリアクティブな状態を作り、
// `computed()` で派生値を作り、
// 関数としてアクションを定義して `return` する。
// Vue コンポーネントの `setup()` と同じ書き方。
//
// 【isAuthenticated の計算】
// `user.value !== null` の場合に認証済みとみなす。
// computed にすることで user が変わると自動的���再計算される。

import { defineStore } from 'pinia'
// ref: リアクティブな変数を作成、computed: 依存値が変わると自動再計算
import { ref, computed } from 'vue'
import { authApi } from '../api/auth'
import type { User, SignUpForm, SignInForm } from '../types'

export const useAuthStore = defineStore('auth', () => {
  // ---- 状態（State）----
  // `ref<User | null>(null)` = User 型または null を取りうるリアクティブ変数
  const user = ref<User | null>(null)
  // ローディング状態（API 呼び出し中は true）
  const loading = ref(false)

  // ---- 算出プロパティ（Computed）----
  // user が null でなければ認証済み
  // computed はリアクティブ: user.value が変わると isAuthenticated も自動更新される
  const isAuthenticated = computed(() => user.value !== null)

  // ---- アクション（Actions）----

  /**
   * 新規ユーザー登録
   *
   * @param form - ユーザー名・メールアドレス・パスワード
   */
  async function signup(form: SignUpForm) {
    // signup 後は自動ログインしない（signin ページに誘導する）
    await authApi.signup(form)
  }

  /**
   * ログイン
   *
   * API 成功後にユーザー情報をストアに保存する。
   * @param form - メールアドレス・パスワード
   */
  async function signin(form: SignInForm) {
    const { data } = await authApi.signin(form)
    // `user.value = ...` でリアクティブな変数を更新
    // 更新すると isAuthenticated が自動的に true になる
    user.value = {
      id: data.user.id,
      username: data.user.username,
      created_at: '', // signin レスポンスに��含まれないため空文字列
    }
  }

  /**
   * ログアウト
   *
   * API でセッションを削除し、ストアのユーザー情報をクリアする。
   */
  async function signout() {
    await authApi.signout()
    // user を null にすることで isAuthenticated が false になる
    user.value = null
  }

  /**
   * ログイン状態を確認する（ページリロード時に呼び出す）
   *
   * Cookie に有効なセッションがあれば user.value を設定する。
   * 401 エラー（未認証）の場合は user.value を null のままにする。
   * `try/catch/finally` でどんな結果でも loading を false に戻す。
   */
  async function fetchMe() {
    try {
      loading.value = true
      const { data } = await authApi.fetchMe()
      user.value = data // 認証済み: ユーザー情報を保存
    } catch {
      // 未認証（401）の場合は user.value = null のまま
      // エラーは握りつぶす（router.beforeEach がリダイレクトを処理する）
      user.value = null
    } finally {
      // `finally` は成功・失敗に関わらず必ず実行される
      loading.value = false
    }
  }

  // このストアから外部に公開する状態とアクション
  return { user, loading, isAuthenticated, signup, signin, signout, fetchMe }
})
