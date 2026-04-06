// ============================================================
// api/favorite.ts — お気に入り API 呼び出し関数
// ============================================================
// このファイルはお気に入り（TBL_FAVORITE）に関する API を呼び出す。
//
// 【エンドポイント対応】
// favoriteApi.add(userId)    → POST   /api/favorites/{userId}
// favoriteApi.remove(userId) → DELETE /api/favorites/{userId}
// favoriteApi.list()         → GET    /api/favorites

import apiClient from './client'

export const favoriteApi = {
  /**
   * ユーザーをお気に入��に追加する
   *
   * POST /api/favorites/{userId}
   * @param userId - お気に入りにしたい相手のユーザー ID
   * @returns `{ message: "お気に入りに追加しました" }`
   */
  add(userId: string) {
    return apiClient.post(`/api/favorites/${userId}`)
  },

  /**
   * お気に入���を解除する
   *
   * DELETE /api/favorites/{userId}
   * @param userId - 解除したい相手のユーザー ID
   * @returns レスポンスなし（204 No Content）
   */
  remove(userId: string) {
    return apiClient.delete(`/api/favorites/${userId}`)
  },

  /**
   * お気に入り一覧を取得する
   *
   * GET /api/favorites
   * @returns `{ favorites: [...] }` （各要素に相手のユーザー情報を含む）
   */
  list() {
    return apiClient.get('/api/favorites')
  },
}
