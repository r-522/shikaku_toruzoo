<!-- ============================================================
components/community/FavoriteStar.vue — お気に入り星コンポーネント
============================================================
このファイルはお気に入り状態をトグルする星アイコンボタンを提供する。

【Unicode 文字の説明】
- '\u2605': ★（BLACK STAR）= お気に入り中（金色で表示）
- '\u2606': ☆（WHITE STAR）= 未登録（グレーで表示）

【クリック時の動作】
ボタンクリック → `toggle` イベントを親に発火
→ 親（UserCard.vue）が `toggleFavorite` イベントを上位に伝える
→ ビュー（CommunityView.vue）が favorite ストアの toggle() を呼ぶ
============================================================ -->
<template>
  <!-- お気に入り状態に応じて色と星の種類を切り替えるボタン -->
  <!-- btn-sm p-0 border-0: 最小サイズで枠線なし（アイコン風のボタン） -->
  <button
    class="btn btn-sm p-0 border-0"
    @click="$emit('toggle')"
    :style="{ color: isFavorite ? '#F9A825' : '#BDBDBD', fontSize: '1.4rem' }"
  >
    <!-- 三項演算子: isFavorite が true なら ★、false なら ☆ -->
    <!-- '\u2605' = ★（黒塗り星）、'\u2606' = ☆（白抜き星） -->
    {{ isFavorite ? '\u2605' : '\u2606' }}
  </button>
</template>

<script setup lang="ts">
defineProps<{
  isFavorite: boolean  // お気に入り登録済みか（true = 金色の★）
}>()

defineEmits<{
  toggle: []  // ボタンクリック時に発火（引数なし）
}>()
</script>
