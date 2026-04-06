// ============================================================
// api/certification.ts — 所持資格 API 呼び出し関数
// ============================================================
// このファイルは所持資格（TBL_HOLDING）に関するバックエンド API を呼び出す。
//
// 【エンドポイント対応】
// certificationApi.list()         → GET  /api/certifications
// certificationApi.create(form)   → POST /api/certifications
// certificationApi.update(id, form) → PUT /api/certifications/{id}
// certificationApi.remove(id)     → DELETE /api/certifications/{id}

import apiClient from './client'
import type { Certification, CertificationForm } from '../types'

export const certificationApi = {
  /**
   * 所持資格一覧取得
   *
   * GET /api/certifications
   * @returns `{ certifications: Certification[] }`（配列はオブジェクトで包まれて返る）
   */
  list() {
    return apiClient.get<{ certifications: Certification[] }>('/api/certifications')
  },

  /**
   * 所持資格を新規登録する
   *
   * POST /api/certifications
   * @param form - 資格名・マスタID・取得日
   * @returns 登録された `Certification` データ（自動生成の id を含む）
   */
  create(form: CertificationForm) {
    return apiClient.post<Certification>('/api/certifications', form)
  },

  /**
   * 所持資格を更新する
   *
   * PUT /api/certifications/{id}
   * @param id - 更新対象の所持資格 ID
   * @param form - 更新後のデータ
   * @returns 更新後の `Certification` データ
   */
  update(id: string, form: CertificationForm) {
    // テンプレートリテラル（`...`）で URL に id を埋め込む
    return apiClient.put<Certification>(`/api/certifications/${id}`, form)
  },

  /**
   * 所持資格を削除する
   *
   * DELETE /api/certifications/{id}
   * @param id - 削除対象の所持資格 ID
   * @returns レスポンスなし（204 No Content）
   */
  remove(id: string) {
    return apiClient.delete(`/api/certifications/${id}`)
  },
}
