// ============================================================
// api/community.ts — コミュニティ API 呼び出し関数
// ============================================================
// このファイルはコミュニティ（他ユーザーの公開情報）に関する API を呼び出す。
//
// 【ページネーションパラメータ】
// listUsers(page, perPage) でページ番号と1ページあたり件数を指定する。
// サーバーから `{ users: [...], total: n, page: n, per_page: n }` が返る。

import apiClient from './client'
import type { CommunityUser, CommunityUserDetail, PaginatedResponse } from '../types'

export const communityApi = {
  /**
   * コミュニティのユーザー一覧を取得する
   *
   * GET /api/community/users?page={page}&per_page={perPage}
   * @param page - ページ番号（デフォルト: 1）
   * @param perPage - 1 ページあたりの件数（デフォルト: 20）
   * @returns ページネーション付きのユーザーリスト
   */
  listUsers(page = 1, perPage = 20) {
    return apiClient.get<PaginatedResponse<CommunityUser>>('/api/community/users', {
      // `per_page` はスネークケース（バックエンドの命名規則に合わせる）
      params: { page, per_page: perPage },
    })
  },

  /**
   * ユーザー詳細情報を取得する
   *
   * GET /api/community/users/{id}
   * @param id - 取得対象のユーザー ID
   * @returns ユーザー詳細（資格・目標の詳細情報を含む）
   */
  getUser(id: string) {
    return apiClient.get<CommunityUserDetail>(`/api/community/users/${id}`)
  },
}
