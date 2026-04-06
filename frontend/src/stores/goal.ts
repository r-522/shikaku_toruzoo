// ============================================================
// stores/goal.ts — 目標状態管理ストア
// ============================================================
// このファイルは学習目標のデータを Pinia で管理する。
//
// 【computed による自動フィルタリング】
// ステータス別のリスト（examDateGoals, passedGoals 等）を computed で定義する。
// goals が変わると自動的にフィルタリングし直されるため、
// ダッシュボードのステータス別タブ表示に活用できる。

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { goalApi } from '../api/goal'
import type { Goal, GoalForm } from '../types'

export const useGoalStore = defineStore('goal', () => {
  // ---- 状態 ----
  const goals = ref<Goal[]>([])
  const loading = ref(false)

  // ---- ステータス別フィルタ（computed）----
  // `filter` はコールバックが true を返す要素だけの新しい配列を作る
  const examDateGoals = computed(() => goals.value.filter((g) => g.status === 'exam_date'))
  const passedGoals = computed(() => goals.value.filter((g) => g.status === 'passed'))
  const failedGoals = computed(() => goals.value.filter((g) => g.status === 'failed'))
  const abandonedGoals = computed(() => goals.value.filter((g) => g.status === 'abandoned'))

  /**
   * 目標を全件取得してキャッシュを更新する
   */
  async function fetchAll() {
    loading.value = true
    try {
      const { data } = await goalApi.list()
      goals.value = data.goals
    } finally {
      loading.value = false
    }
  }

  /**
   * 目標を追加してキャッシュの先頭に挿入する
   *
   * @param form - 目標データ（Partial = 一部フィールドのみ指定可）
   */
  async function add(form: Partial<GoalForm>) {
    const { data } = await goalApi.create(form)
    goals.value.unshift(data) // 先頭に追加
  }

  /**
   * 目標を更新してキャッシュを差し替える
   *
   * @param id - 更新対象のID
   * @param form - 更新後のデータ（Partial なので変更するフィールドだけでOK）
   */
  async function update(id: string, form: Partial<GoalForm>) {
    const { data } = await goalApi.update(id, form)
    const idx = goals.value.findIndex((g) => g.id === id)
    if (idx !== -1) goals.value[idx] = data
  }

  /**
   * 目標を削除してキャッシュから除去する
   *
   * @param id - 削除対象のID
   */
  async function remove(id: string) {
    await goalApi.remove(id)
    goals.value = goals.value.filter((g) => g.id !== id)
  }

  return {
    goals, loading,
    // ステータス別フィルタもエクスポート（ダッシュボードのタブ表示で使用）
    examDateGoals, passedGoals, failedGoals, abandonedGoals,
    fetchAll, add, update, remove,
  }
})
