// ============================================================
// stores/favorite.ts — お気に入り状態管理ストア
// ============================================================
// このファイルはお気に入りの追加・解除を Pinia で管理する。
//
// 【toggle パターン】
// 現在のお気に入り状態（isFavorite）を受け取り、
// true なら解除、false なら追加するトグル関数。
// コンポーネント側で「今の状態」を渡すだけでよいシンプルな設計。
//
// 【コミュニティストアとの連携】
// お気に入り操作後にコミュニティ一覧を再取得することで、
// 画面の is_favorite フラグが最新状態になる。

import { defineStore } from 'pinia'
import { favoriteApi } from '../api/favorite'
// 別のストアを参照する（ストア間の連携）
import { useCommunityStore } from './community'

export const useFavoriteStore = defineStore('favorite', () => {
  /**
   * お気に入りを追加/解除する（トグル）
   *
   * @param userId - 対象ユーザーのID
   * @param isFavorite - 現在の状態（true=お気に入り中 → 解除、false=未登録 → 追加）
   */
  async function toggle(userId: string, isFavorite: boolean) {
    if (isFavorite) {
      // 現在お気に入り中 → 解除（DELETE リクエスト）
      await favoriteApi.remove(userId)
    } else {
      // 現在未登録 → 追加（POST リクエスト）
      await favoriteApi.add(userId)
    }

    // コミュニティストアの一覧を再取得して is_favorite フラグを更新する
    // `useCommunityStore()` はコンポーネント外でも使える（アクション内でOK）
    const communityStore = useCommunityStore()
    // 現在のページを維持したまま再取得
    await communityStore.fetchUsers(communityStore.currentPage)
  }

  return { toggle }
})
