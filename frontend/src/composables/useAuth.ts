// ============================================================
// composables/useAuth.ts — 認証コンポーザブル
// ============================================================
// このファイルは認証に関連するコンポーネントロジックをまとめた「コンポーザブル」。
//
// 【コンポーザブル（Composable）とは】
// `use` で始まる関数として定義した「再利用可能なロジックの塊」。
// 複数のコンポーネントで同じロジックを使い回したい場合に作る。
// React の Custom Hooks に相当する Vue 3 の設計パターン。
//
// 【useAuth の目的】
// auth ストアへのアクセスとルーター連携（ログアウト後のリダイレクト）を
// 1 つの関数にまとめることで、ヘッダーコンポーネント等から簡単に使える。

import { useAuthStore } from '../stores/auth'
// Vue Router のナビゲーション関数（useRouter は setup() 内でしか使えない）
import { useRouter } from 'vue-router'

/**
 * 認証機能へのアクセスを提供するコンポーザブル
 *
 * コンポーネントの `<script setup>` 内で呼び出して使う:
 * ```vue
 * <script setup>
 * const { user, handleSignout } = useAuth()
 * </script>
 * ```
 *
 * @returns ユーザー情報・認証状態・ログアウト関数
 */
export function useAuth() {
  // Pinia ストアのインスタンスを取得
  const store = useAuthStore()
  // Vue Router のインスタンスを取得（ページ遷移に使用）
  const router = useRouter()

  /**
   * ログアウトしてサインインページへ遷移する
   *
   * サインアウト API を呼び出してストアをクリアした後、
   * `/signin` へプログラム的にリダイレクトする。
   */
  async function handleSignout() {
    await store.signout()         // API 呼び出し + ストアの user を null に
    router.push('/signin')        // サインインページへ遷移
  }

  return {
    user: store.user,             // ログイン中ユーザー情報（または null）
    isAuthenticated: store.isAuthenticated, // 認証状態（true/false）
    handleSignout,                // ログアウト関数
  }
}
