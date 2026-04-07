<!-- ============================================================
components/community/UserCard.vue — コミュニティユーザーカードコンポーネント
============================================================
このファイルはコミュニティ一覧画面で 1 ユーザー分の情報を表示するカードコンポーネント。

【表示内容】
- ヘッダー: お気に入り星、ユーザー名（詳細へのリンク）、グッドマーク、総勉強時間
- 所持資格: 資格名と取得日のリスト
- 目標: ステータスバッジと資格名のリスト

【コンポーネントの連携】
FavoriteStar → @toggle → $emit('toggleFavorite', user) → 親（CommunityView.vue）
→ favoriteStore.toggle() → API 呼び出し → コミュニティ一覧を再取得

【v-for の key について】
`v-for` でリスト描画する際に `:key` は必須。
Vue が各要素を一意に識別して効率的な DOM 更新を行うために使う。
'c' + idx や 'g' + idx は資格と目標で key が衝突しないようにプレフィックスを付けている。
============================================================ -->
<template>
  <div class="card shadow-sm mb-2">
    <div class="card-body py-3">
      <!-- ヘッダー: ユーザー情報 -->
      <div class="d-flex justify-content-between align-items-center mb-2">
        <div class="d-flex align-items-center gap-2">
          <!-- お気に入り星ボタン（クリックで親に toggleFavorite イベントを送る） -->
          <FavoriteStar :is-favorite="user.is_favorite" @toggle="$emit('toggleFavorite', user)" />
          <!-- ユーザー名: 詳細ページへのリンク（router-link でページ遷移） -->
          <router-link :to="`/community/${user.id}`" class="fw-bold text-decoration-none text-dark">
            {{ user.username }}
          </router-link>
          <!-- グッドマーク: 1 つ以上合格がある場合に ✓ を表示 -->
          <GoodMark :has-good-mark="user.has_good_mark" />
        </div>
        <!-- 総勉強時間 -->
        <div class="small fw-bold" style="color: #1A73E8">
          合計 {{ user.total_study_hours }}h
        </div>
      </div>

      <!-- 所持資格リスト（1 件以上ある場合のみ表示） -->
      <div v-if="user.certifications.length > 0" class="ps-4 mb-1">
        <div class="text-muted small fw-bold mb-1">所持資格</div>
        <!-- v-for: 配列の各要素を繰り返し表示する -->
        <div
          v-for="(cert, idx) in user.certifications"
          :key="'c' + idx"
          class="d-flex align-items-center gap-2 small"
          style="line-height: 1.7"
        >
          <!-- 「取得」バッジ（flex-shrink-0 で縮まない） -->
          <span class="badge flex-shrink-0" style="background-color: #2E7D32">取得</span>
          <!-- flex-grow-1 + text-truncate + min-width:0: 長い資格名を省略表示 -->
          <span class="flex-grow-1 text-truncate" style="min-width: 0">{{ cert.certification_name }}</span>
          <!-- 取得日（text-nowrap で改行させず右端に固定） -->
          <span v-if="cert.acquired_date" class="text-muted text-nowrap">({{ cert.acquired_date }})</span>
        </div>
      </div>

      <!-- 目標リスト（1 件以上ある場合のみ表示） -->
      <div v-if="user.goals.length > 0" class="ps-4">
        <div class="text-muted small fw-bold mb-1">目標</div>
        <div
          v-for="(goal, idx) in user.goals"
          :key="'g' + idx"
          class="d-flex align-items-center gap-2 small"
          style="line-height: 1.7"
        >
          <!-- ステータスバッジ（flex-shrink-0 で縮まない） -->
          <StatusBadge :status="goal.status" class="flex-shrink-0" />
          <!-- flex-grow-1 + text-truncate + min-width:0: 長い資格名を省略表示 -->
          <span class="flex-grow-1 text-truncate" style="min-width: 0">{{ goal.certification_name }}</span>
          <!-- 勉強時間（text-nowrap で改行させず右端に固定） -->
          <span v-if="goal.study_hours > 0" class="text-muted text-nowrap">({{ goal.study_hours }}h)</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import FavoriteStar from './FavoriteStar.vue'
import GoodMark from './GoodMark.vue'
import StatusBadge from '../goal/StatusBadge.vue'
import type { CommunityUser } from '../../types'

defineProps<{
  user: CommunityUser  // 表示するユーザーデータ
}>()

defineEmits<{
  toggleFavorite: [user: CommunityUser]  // お気に入りトグルボタンが押された
}>()
</script>
