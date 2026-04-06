// ============================================================
// api/client.ts — Axios HTTP クライアントの設定
// ============================================================
// このファイルはすべての API リクエストの基盤となる Axios インスタンスを作成する。
//
// 【Axios とは】
// HTTP リクエストを簡単に行うための JavaScript ライブラリ。
// fetch API の上位互換で、インターセプタや型付きレスポンスが使いやすい。
//
// 【withCredentials とは】
// `withCredentials: true` により、クロスオリジンリクエストでも
// Cookie を自動的に送受信する。セッション Cookie の認証に必要。
//
// 【X-Requested-With ヘッダー】
// CSRF（クロスサイトリクエストフォージェリ）対策のカスタムヘッダー。
// ブラウザから直接送るリクエストは自動でこのヘッダーを付けられないため、
// 偽造リクエストを区別できる。
//
// 【インターセプタとは】
// リクエスト/レスポンスの送受信を「横断的に」処理する仕組み。
// ここでは 401 レスポンスを受けたら自動的にサインインページへリダイレクトする。

import axios from 'axios'
// Vue Router インスタンス（ページ遷移に使用）
import router from '../router'

// Axios のカスタムインスタンスを作成
const apiClient = axios.create({
  // 開発時は Vite のプロキシを使うため空文字列
  // 本番時は環境変数 VITE_API_BASE_URL に設定されたURLを使う（省略時は空文字列）
  baseURL: import.meta.env.VITE_API_BASE_URL || '',
  // Cookie（セッショントークン）を自動送信するために必要
  withCredentials: true,
  headers: {
    // CSRF 対策ヘッダー（バックエンドはこのヘッダーの存在を確認する）
    'X-Requested-With': 'XMLHttpRequest',
  },
})

// ---- レスポンスインターセプタの設定 ----
// `interceptors.response.use(成功時の処理, 失敗時の処理)` の形式
apiClient.interceptors.response.use(
  // 成功時: レスポンスをそのまま返す（何も変えない）
  (response) => response,
  // 失敗時: エラーレスポンスを受け取って処理する
  (error) => {
    // 401 Unauthorized の場合はセッション切れと判断してサインインページへ
    if (error.response?.status === 401) {
      // `router.push('/signin')` でプログラム的にページ遷移する
      router.push('/signin')
    }
    // エラーを呼び出し元に伝播させる（Promise.reject で失敗状態の Promise を返す）
    return Promise.reject(error)
  }
)

// このインスタンスを他のファイルから `import apiClient from './client'` で使う
export default apiClient
