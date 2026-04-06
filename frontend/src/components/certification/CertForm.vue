<!-- ============================================================
components/certification/CertForm.vue — 所持資格フォームコンポーネント
============================================================
このファイルは所持資格の登録・編集フォームを提供するコンポーネント。
FormModal.vue の <slot> に挿入されて使われる。

【defineExpose({ form }) について】
通常 <script setup> 内の変数は外部から参照できない（カプセル化）。
`defineExpose` で明示的に公開することで、
親コンポーネントが `certFormRef.value.form` でデータを取得できる。

【reactive() と ref() の違い】
- `ref(値)`: 単一値（プリミティブ）に使う。`.value` でアクセス
- `reactive(オブジェクト)`: オブジェクト全体をリアクティブにする。`.value` 不要

【watch の使い方】
`watch(() => props.initial, コールバック)` で
props.initial が変わったとき（例: 編集対象が切り替わったとき）に
フォームを新しい値で上書きする。
============================================================ -->
<template>
  <div>
    <!-- 資格名（オートコンプリート付き） -->
    <div class="mb-3">
      <label class="form-label">資格名</label>
      <!-- CertAutocomplete は v-model で入力値を管理し、
           @select で選択した資格の { name, master_id } を受け取る -->
      <CertAutocomplete
        v-model="form.certification_name"
        @select="onSelect"
      />
    </div>

    <!-- 取得日（HTML5 の date 入力） -->
    <div class="mb-3">
      <label class="form-label">取得日</label>
      <!-- `v-model` で form.acquired_date と双方向バインディング -->
      <!-- `type="date"` でカレンダー UI が表示される（YYYY-MM-DD 形式） -->
      <input v-model="form.acquired_date" type="date" class="form-control" />
    </div>
  </div>
</template>

<script setup lang="ts">
// reactive: オブジェクトをリアクティブにする
// watch: 値の変化を監視する
import { reactive, watch } from 'vue'
import CertAutocomplete from '../master/CertAutocomplete.vue'
import type { CertificationForm } from '../../types'

const props = defineProps<{
  initial?: CertificationForm  // 初期値（編集時に渡される。新規の場合は undefined）
}>()

// フォームの状態（reactive でオブジェクト全体をリアクティブに）
// `?.` オプショナルチェーン: initial が undefined の場合は || の右側を使う
const form = reactive<CertificationForm>({
  certification_name: props.initial?.certification_name || '',
  master_id: props.initial?.master_id || null,
  acquired_date: props.initial?.acquired_date || '',
})

// 編集対象が変わったとき（props.initial が更新されたとき）にフォームを同期
watch(() => props.initial, (val) => {
  if (val) {
    form.certification_name = val.certification_name
    form.master_id = val.master_id
    form.acquired_date = val.acquired_date
  }
})

/**
 * CertAutocomplete で候補を選択した時の処理
 *
 * @param payload - { name: 選択した資格名, master_id: マスタID（手入力時は null）}
 */
function onSelect(payload: { name: string; master_id: string | null }) {
  form.certification_name = payload.name
  form.master_id = payload.master_id
}

// form を外部から参照可能にする（親が `certFormRef.value.form` でデータを取得）
defineExpose({ form })
</script>
