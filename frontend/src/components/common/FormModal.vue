<!-- ============================================================
components/common/FormModal.vue — フォームモーダルコンポーネント
============================================================
このファイルは「フォームを表示するモーダル」の汎用コンポーネント。
資格登録・目標登録等、フォームを含むモーダルで共通して使用する。

【スロット（<slot>）について】
`<slot>` は親コンポーネントから任意の HTML/コンポーネントを注入できる「穴」。
このコンポーネントを使う親が <FormModal> の中に form フィールドを書くと、
それが <slot> の位置に表示される。

例（親コンポーネント）:
<FormModal title="資格を追加" :show="showModal" @submit="save">
  <input v-model="form.name" />  ← これが <slot> の位置に入る
</FormModal>

【@submit.prevent について】
`<form @submit.prevent>` は
フォームの送信（ページリロード）をデフォルト動作をキャンセルして
Vue のイベントハンドラに処理を委ねる。
============================================================ -->
<template>
  <div v-if="show" class="modal d-block" tabindex="-1" style="background-color: rgba(0,0,0,0.5)">
    <div class="modal-dialog modal-dialog-centered modal-dialog-scrollable">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title">{{ title }}</h5>
          <!-- X ボタンでモーダルを閉じる（close イベントを発火） -->
          <button type="button" class="btn-close" @click="$emit('close')"></button>
        </div>
        <!-- `@submit.prevent` でフォームのデフォルト送信を無効化し、
             submit イベントを親に伝える -->
        <form @submit.prevent="$emit('submit')">
          <div class="modal-body">
            <!-- スロット: 親コンポーネントが渡すフォームフィールドがここに入る -->
            <slot></slot>
          </div>
          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" @click="$emit('close')">キャンセル</button>
            <!-- `:disabled="loading"` は loading が true の間ボタンを無効化
                 （二重送信防止） -->
            <button type="submit" class="btn btn-primary" :disabled="loading">
              {{ submitText }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  show: boolean        // モーダルの表示/非表示
  title: string        // モーダルのタイトル（必須）
  submitText?: string  // 送信ボタンのテキスト（任意）
  loading?: boolean    // 送信中フラグ（true でボタン無効化）
}>(), {
  submitText: '保存',  // デフォルト: 保存
  loading: false,      // デフォルト: 非ローディング
})

defineEmits<{
  close: []  // X ボタン/キャンセルボタンで発火
  submit: [] // 送信ボタンで発火（.prevent で form のデフォルト送信は抑制済み）
}>()
</script>
