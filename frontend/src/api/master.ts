// ============================================================
// api/master.ts — 資格マスタ API 呼び出し関数
// ============================================================
// このファイルは資格マスタ（TBL_MASTER）の検索 API を呼び出す。
// オートコンプリート（入力補完）に使用する。
//
// 【インクリメンタル検索とは】
// ユーザーが文字を入力するたびに検索を実行し、候補を表示すること。
// パフォーマンスのため debounce（連続入力を間引く）と組み合わせて使う。

import apiClient from './client'
import type { MasterCertification } from '../types'

export const masterApi = {
  /**
   * 資格名でインクリメンタル検索する
   *
   * GET /api/master/certifications?q={keyword}
   * 2 文字以上の入力で検索を実行（サーバー側の制限）。
   * @param q - 検索キーワード
   * @returns `{ certifications: MasterCertification[] }`（最大 10 件）
   */
  search(q: string) {
    return apiClient.get<{ certifications: MasterCertification[] }>('/api/master/certifications', {
      // `params` オプションでクエリパラメータを指定（`?q=keyword` の形式に変換される）
      params: { q },
    })
  },
}
