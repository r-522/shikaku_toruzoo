<!-- ============================================================
components/common/ConfirmModal.vue — 確認モーダルコンポーネント
============================================================
このファイルは「本当に削除しますか？」等の確認ダイアログを表示するモーダル。
削除操作の前に必ずこのモーダルを使って誤操作を防ぐ。

【props と emits の役割】
- props（親→子）: show（表示/非表示）、メッセージ等の設定値
- emits（子→親）: confirm（確認）、cancel（キャンセル）のイベント通知

【`$emit('confirm')` について】
テンプレート内で `$emit` を直接呼ぶことで、
script setup で定義したイベントを発火できる。
親コンポーネントでは `@confirm="削除処理"` として受け取る。

【オーバーレイ（背景の黒い半透明）】
`style="background-color: rgba(0,0,0,0.5)"` で
モーダル後ろの背景を暗くする。
============================================================ -->
<template>
  <!-- `v-if="show"` でモーダルの表示/非表示を制御 -->
  <!-- d-block: display: block（Bootstrap のモーダルは通常 display: none） -->
  <div v-if="show" class="modal d-block" tabindex="-1" style="background-color: rgba(0,0,0,0.5)">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content">
        <!-- ヘッダー: タイトルと閉じるボタン -->
        <div class="modal-header">
          <h5 class="modal-title">{{ title }}</h5>
          <!-- X ボタンでキャンセルイベントを発火 -->
          <button type="button" class="btn-close" @click="$emit('cancel')"></button>
        </div>
        <!-- ボディ: 確認メッセージ -->
        <div class="modal-body">
          <p>{{ message }}</p>
        </div>
        <!-- フッター: 操作ボタン -->
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" @click="$emit('cancel')">キャンセル</button>
          <!-- confirm イベントを発火（親がこれを受け取って削除処理を実行） -->
          <button type="button" class="btn btn-danger" @click="$emit('confirm')">{{ confirmText }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// `withDefaults(defineProps<{...}>(), {...})` の構造:
// - `defineProps<{型}>()` で props の型を定義（TypeScript ジェネリクス構文）
// - `withDefaults(props, {デフォルト値})` でデフォルト値を設定
withDefaults(defineProps<{
  show: boolean           // モーダルを表示するか（必須）
  title?: string          // タイトル（任意: `?` は TypeScript でオプションの意）
  message: string         // 確認メッセージ（必須）
  confirmText?: string    // 確認ボタンのテキスト（任意）
}>(), {
  title: '確認',          // デフォルトタイトル
  confirmText: '削除する', // デフォルト確認ボタンテキスト
})

// `defineEmits<{イベント名: [引数の型]}>()` で emit できるイベントを宣言
// 配列が空 `[]` = 引数なしのイベント
defineEmits<{
  confirm: [] // 確認ボタンが押された
  cancel: []  // キャンセルボタン/X ボタンが押された
}>()
</script>
