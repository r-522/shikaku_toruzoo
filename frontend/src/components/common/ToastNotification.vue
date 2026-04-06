<!-- ============================================================
components/common/ToastNotification.vue — トースト通知コンポーネント
============================================================
このファイルは画面右上に表示されるポップアップ通知を定義する。
useToast コンポーザブルと連動して、show() が呼ばれると自動表示される。

【position-fixed について】
`position: fixed` + `top: 0` + `right: 0（end-0）` で
スクロールしても常に右上に固定される。
z-index: 9999 でモーダルより前面に表示される。

【:class バインディング】
`:class="{ 'クラス名': 条件 }"` でクラスを動的に付け外しする。
通知種別（success/error/info）に応じて背景色が変わる。
============================================================ -->
<template>
  <!-- `v-if` で visible.value が true の時のみ表示 -->
  <!-- `position-fixed top-0 end-0 p-3` でスクリーン右上に固定 -->
  <div
    v-if="toast.visible.value"
    class="position-fixed top-0 end-0 p-3"
    style="z-index: 9999"
  >
    <!-- Bootstrap の toast コンポーネント。種別に応じてクラスを切り替え -->
    <div
      class="toast show"
      :class="{
        'bg-success text-white': toast.type.value === 'success', // 緑（成功）
        'bg-danger text-white': toast.type.value === 'error',    // 赤（エラー）
        'bg-info text-white': toast.type.value === 'info',       // 水色（情報）
      }"
    >
      <!-- トースト本体: メッセージと閉じるボタン -->
      <div class="toast-body d-flex justify-content-between align-items-center">
        <span>{{ toast.message.value }}</span>
        <!-- `@click="toast.hide()"` で手動で閉じられる -->
        <button type="button" class="btn-close btn-close-white ms-2" @click="toast.hide()"></button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// useToast コンポーザブルからトースト状態を取得
// この状態はモジュールスコープで管理されるため、
// show() を呼んだどのコンポーネントの通知もここに表示される
import { useToast } from '../../composables/useToast'

const toast = useToast()
</script>
