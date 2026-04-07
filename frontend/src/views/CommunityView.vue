<!-- ============================================================
views/CommunityView.vue — コミュニティ画面
============================================================
このファイルはコミュニティ（他ユーザーの公開情報一覧）画面を提供する。

【ダッシュボードとの違い】
DashboardView.vue にもコミュニティセクションがあるが、
このビューはコミュニティに特化した独立したページ。
ページネーション・お気に入り優先表示・空状態表示を含む。

【v-if / v-else-if / v-else の連鎖】
Vue の条件分岐は
- `v-if`: 最初の条件
- `v-else-if`: 次の条件（v-if に続けて使う）
- `v-else`: 上記のどれでも None の時

`<template>` はレンダリングされない仮想の要素で、
複数の要素をグループ化するために使う。
============================================================ -->
<template>
  <div class="container py-4">
    <h3 class="mb-4">コミュニティ</h3>

    <!-- ローディング中: スピナーを表示 -->
    <div v-if="communityStore.loading" class="text-center py-5">
      <div class="spinner-border text-primary"></div>
    </div>

    <!-- ローディング完了後 -->
    <template v-else>
      <!-- ユーザーが 0 人の場合（自分以外にユーザーがいない） -->
      <EmptyState v-if="communityStore.users.length === 0" message="まだユーザーがいません。" />

      <!-- ユーザーが 1 人以上いる場合 -->
      <template v-else>
        <!-- お気に入りユーザーを上部に表示 -->
        <div v-if="favorites.length > 0" class="mb-3">
          <UserCard
            v-for="u in favorites"
            :key="u.id"
            :user="u"
            @toggle-favorite="handleToggleFavorite"
          />
        </div>

        <!-- お気に入りとその他の区切り（両方ある場合のみ） -->
        <div v-if="favorites.length > 0 && others.length > 0" class="text-center my-3" style="color: #BDBDBD">
          <span>&#x2500;&#x2500;&#x2500; みんなの状況 &#x2500;&#x2500;&#x2500;</span>
        </div>

        <!-- お気に入り以外のユーザー -->
        <div class="d-flex flex-column gap-3 mb-3">
          <UserCard
            v-for="u in others"
            :key="u.id"
            :user="u"
            @toggle-favorite="handleToggleFavorite"
          />
        </div>

        <!-- ページネーション -->
        <!-- `@page-change="communityStore.fetchUsers"` で
             ページ変更時にストアのフェッチ関数を直接呼び出す -->
        <Pagination
          :total="communityStore.total"
          :per-page="communityStore.perPage"
          :current-page="communityStore.currentPage"
          @page-change="communityStore.fetchUsers"
        />
      </template>
    </template>
  </div>
</template>

<script setup lang="ts">
// computed: 算出プロパティ（store の値から派生する値）
// onMounted: コンポーネント表示時に 1 回実行する処理
import { computed, onMounted } from 'vue'
import { useCommunityStore } from '../stores/community'
import { useFavoriteStore } from '../stores/favorite'
import { useToast } from '../composables/useToast'
import UserCard from '../components/community/UserCard.vue'
import Pagination from '../components/common/Pagination.vue'
import EmptyState from '../components/common/EmptyState.vue'
import type { CommunityUser } from '../types'

const communityStore = useCommunityStore()
const favoriteStore = useFavoriteStore()
const toast = useToast()

// ページ表示時にコミュニティユーザー一覧を取得
onMounted(() => communityStore.fetchUsers())

// お気に入りユーザーとそれ以外を computed で分離（自動更新される）
const favorites = computed(() => communityStore.users.filter((u) => u.is_favorite))
const others = computed(() => communityStore.users.filter((u) => !u.is_favorite))

/**
 * お気に入りトグルハンドラ
 *
 * UserCard の @toggle-favorite イベントを受け取り、
 * favorite ストアを通じて API を呼び出す。
 * 完了後にコミュニティ一覧を再取得して is_favorite フラグを更新する。
 *
 * @param user - お気に入り操作対象のユーザー
 */
async function handleToggleFavorite(user: CommunityUser) {
  try {
    await favoriteStore.toggle(user.id, user.is_favorite)
  } catch {
    toast.show('エラーが発生しました', 'error')
  }
}
</script>
