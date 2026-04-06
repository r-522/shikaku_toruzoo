// ============================================================
// stores/community.ts — コミュニティ状態管理ストア
// ============================================================
// このファイルはコミュニティのユーザー一覧・詳細データを Pinia で管理する。
//
// 【ページネーション状態の管理】
// total（全件数）・currentPage・perPage を状態として持つことで、
// コンポーネントがこれらを参照してページ切り替えを行える。

import { defineStore } from 'pinia'
import { ref } from 'vue'
import { communityApi } from '../api/community'
import type { CommunityUser, CommunityUserDetail } from '../types'

export const useCommunityStore = defineStore('community', () => {
  // ---- 状態 ----
  const users = ref<CommunityUser[]>([])
  const total = ref(0)           // 全件数（ページネーション用）
  const currentPage = ref(1)     // 現在のページ番号
  const perPage = ref(20)        // 1 ページあたりの件数
  const loading = ref(false)
  // ユーザー詳細（null = 未取得）
  const userDetail = ref<CommunityUserDetail | null>(null)

  /**
   * コミュニティのユーザー一覧を取得する
   *
   * @param page - ページ番号（デフォルト: 1）
   */
  async function fetchUsers(page = 1) {
    loading.value = true
    try {
      const { data } = await communityApi.listUsers(page, perPage.value)
      users.value = data.users
      total.value = data.total
      currentPage.value = data.page
    } finally {
      loading.value = false
    }
  }

  /**
   * ユーザー詳細を取得する（コミュニティ詳細ページ用）
   *
   * @param id - 取得対象のユーザー ID
   */
  async function fetchUserDetail(id: string) {
    loading.value = true
    try {
      const { data } = await communityApi.getUser(id)
      userDetail.value = data
    } finally {
      loading.value = false
    }
  }

  return { users, total, currentPage, perPage, loading, userDetail, fetchUsers, fetchUserDetail }
})
