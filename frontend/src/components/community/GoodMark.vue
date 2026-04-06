<!-- ============================================================
components/community/GoodMark.vue — グッドマークコンポーネント
============================================================
このファイルは「1つ以上の合格がある」ユーザーに表示するグッドマークバッジ。

【&#10003; について】
HTML の文字参照で「✓」（チェックマーク）を表示する。
Unicode の U+2713（CHECK MARK）に相当する。

【v-if="hasGoodMark"】
hasGoodMark が false の場合は DOM から要素を削除する（非表示ではなく未生成）。
============================================================ -->
<template>
  <!-- グッドマークがある場合のみ表示（v-if で条件付きレンダリング） -->
  <span
    v-if="hasGoodMark"
    class="badge rounded-pill"
    style="background-color: #2E7D32; font-size: 0.75rem"
  >
    &#10003; <!-- ✓ チェックマーク -->
  </span>
</template>

<script setup lang="ts">
defineProps<{
  hasGoodMark: boolean  // グッドマークを表示するか（true = 1つ以上合格済み）
}>()
</script>
