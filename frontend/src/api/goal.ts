// ============================================================
// api/goal.ts — 目標 API 呼び出し関数
// ============================================================
// このファイルは学習目標（TBL_GOAL）に関するバックエンド API を呼び出す。
//
// 【Partial<GoalForm> とは】
// TypeScript の `Partial<T>` は「T の全フィールドを Optional にした型」。
// 部分更新（一部のフィールドだけ送る）に使う。
// GoalForm の全フィールドが必須 → Partial<GoalForm> では全フィールドが省略可能。

import apiClient from './client'
import type { Goal, GoalForm } from '../types'

export const goalApi = {
  /**
   * 目標一覧取得
   *
   * GET /api/goals
   * @returns `{ goals: Goal[] }`
   */
  list() {
    return apiClient.get<{ goals: Goal[] }>('/api/goals')
  },

  /**
   * 目標を新規登録する
   *
   * POST /api/goals
   * @param form - `Partial<GoalForm>` で部分的に指定可能
   *   （status, memo, study_hours は省略できる）
   * @returns 登録された `Goal` データ
   */
  create(form: Partial<GoalForm>) {
    return apiClient.post<Goal>('/api/goals', form)
  },

  /**
   * 目標を更新する（部分更新）
   *
   * PUT /api/goals/{id}
   * 送ったフィールドだけが更新される（送らなかったフィールドは変わらない）。
   * @param id - 更新対象の目標 ID
   * @param form - 更新するフィールド（Partial なので一部だけでもOK）
   * @returns 更新後の `Goal` データ
   */
  update(id: string, form: Partial<GoalForm>) {
    return apiClient.put<Goal>(`/api/goals/${id}`, form)
  },

  /**
   * 目標を削除する
   *
   * DELETE /api/goals/{id}
   * @param id - 削除対象の目標 ID
   */
  remove(id: string) {
    return apiClient.delete(`/api/goals/${id}`)
  },
}
