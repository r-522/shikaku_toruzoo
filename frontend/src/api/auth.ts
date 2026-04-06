// ============================================================
// api/auth.ts — 認証 API 呼び出し関数
// ============================================================
// このファイルは認証に関するバックエンド API を呼び出す関数をまとめる。
//
// 【役割の分担】
// api/*.ts: バックエンドとの通信（URL・メソッド・型）
// stores/*.ts: 状態管理（API 結果をリアクティブな変数に保存）
// views/*.vue: 画面表示とユーザー操作のハンドリング

import apiClient from './client'
// TypeScript の型定義を import（型のみなので実行時には除去される）
import type { SignUpForm, SignInForm, User } from '../types'

export const authApi = {
  /**
   * 新規ユーザー登録
   *
   * POST /api/auth/signup
   * @param form - ユーザー名・メールアドレス・パスワード
   * @returns 成功時は `{ message: "アカウントが作成されました" }`
   */
  signup(form: SignUpForm) {
    return apiClient.post('/api/auth/signup', form)
  },

  /**
   * ログイン
   *
   * POST /api/auth/signin
   * 成功時にサーバーが `session_token` Cookie を発行する。
   * @param form - メールアドレス・パスワード
   * @returns `{ user: { id: string, username: string } }`
   */
  signin(form: SignInForm) {
    // `<{ user: { id: string; username: string } }>` はレスポンスボディの型
    return apiClient.post<{ user: { id: string; username: string } }>('/api/auth/signin', form)
  },

  /**
   * ログアウト
   *
   * POST /api/auth/signout
   * サーバー側でセッションを削除し、Cookie を無効化する。
   */
  signout() {
    return apiClient.post('/api/auth/signout')
  },

  /**
   * ログイン中ユーザー情報取得
   *
   * GET /api/auth/me
   * ページ読み込み時に「誰がログインしているか」を確認するために使う。
   * 未認証の場合は 401 エラー（インターセプタが自動でリダイレクト）。
   * @returns `User` 型（id, username, created_at）
   */
  fetchMe() {
    return apiClient.get<User>('/api/auth/me')
  },
}
