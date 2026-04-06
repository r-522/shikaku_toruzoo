<!-- ============================================================
components/common/EmptyState.vue — 空状態コンポーネント
============================================================
このファイルはリストが空の場合に表示するプレースホルダーを提供する。

「まだデータがありません」等のメッセージを中央揃えで表示する。
props の message に任意のテキストを渡すことで再利用できる。

【使用例】
<EmptyState message="まだ資格が登録されていません" />
<EmptyState message="目標がありません。新しい目標を追加してみましょう" />
============================================================ -->
<template>
  <!-- py-5: 上下パディング 5 単位（Bootstrap の spacing ユーティリティ） -->
  <!-- 9E9E9E: グレー色（コンテンツなしを示す落ち着いた色）-->
  <div class="text-center py-5" style="color: #9E9E9E">
    <!-- `props` から受け取ったメッセージをそのまま表示 -->
    <p class="fs-5">{{ message }}</p>
  </div>
</template>

<script setup lang="ts">
// `defineProps<{ message: string }>()` で message プロパティの型を定義
// この構文は TypeScript ジェネリクスを使ったコンパイル時の型チェック
defineProps<{
  message: string  // 空状態に表示するメッセージ（必須）
}>()
</script>
