<!-- ============================================================
components/goal/GoalForm.vue — 目標フォームコンポーネント
============================================================
このファイルは学習目標の登録・編集フォームを提供するコンポーネント。
FormModal.vue の <slot> に挿入されて使われる。

【勉強時間の 0.5 時間刻み操作】
数値入力フィールドに加えて +/- ボタンを設けて
0.5 時間刻みで増減できるようにする。
浮動小数点数の精度問題（0.1 + 0.2 = 0.30000000000000004）を避けるため
`Math.round(値 * 10) / 10` で丸めを行う。

【v-model.number について】
`.number` 修飾子は入力値を自動的に数値型に変換する。
（デフォルトでは input の value は文字列になる）

【watch の役割】
編集時に props.initial が変わったとき（別の目標を編集する際）に
フォームデータを新しい値で上書きする。
============================================================ -->
<template>
  <div>
    <!-- 資格名（オートコンプリート付き） -->
    <div class="mb-3">
      <label class="form-label">資格名</label>
      <CertAutocomplete
        v-model="form.certification_name"
        @select="onSelect"
      />
    </div>

    <!-- 目標日 -->
    <div class="mb-3">
      <label class="form-label">目標日</label>
      <!-- required: HTML5 のバリデーション属性（空欄でフォーム送信を防ぐ） -->
      <input v-model="form.target_date" type="date" class="form-control" required />
    </div>

    <!-- ステータス選択 -->
    <div class="mb-3">
      <label class="form-label">ステータス</label>
      <!-- `v-model` で form.status と select の値を双方向バインディング -->
      <select v-model="form.status" class="form-select">
        <option value="exam_date">受験日</option>
        <option value="passed">合格</option>
        <option value="failed">不合格</option>
        <option value="abandoned">断念</option>
      </select>
    </div>

    <!-- 勉強時間（+/- ボタン付き） -->
    <div class="mb-3">
      <label class="form-label">勉強時間</label>
      <!-- input-group: Bootstrap でボタンと入力フィールドをグループ化 -->
      <div class="input-group">
        <!-- -0.5 ボタン: 最小値 0.5 を下回らないように decrementHours が制御 -->
        <button class="btn btn-outline-secondary" type="button" @click="decrementHours">-</button>
        <!-- v-model.number: 入力値を文字列ではなく数値として扱う -->
        <input
          v-model.number="form.study_hours"
          type="number"
          class="form-control text-center"
          min="0"
          step="0.5"
          style="max-width: 100px"
        />
        <!-- +0.5 ボタン -->
        <button class="btn btn-outline-secondary" type="button" @click="incrementHours">+</button>
        <span class="input-group-text">時間</span>
      </div>
    </div>

    <!-- メモ -->
    <div class="mb-3">
      <label class="form-label">メモ</label>
      <!-- maxlength: サーバー側の制限（1000 文字）と合わせる -->
      <textarea v-model="form.memo" class="form-control" rows="3" maxlength="1000"></textarea>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue'
import CertAutocomplete from '../master/CertAutocomplete.vue'
import type { GoalForm } from '../../types'

const props = defineProps<{
  initial?: Partial<GoalForm>  // 初期値（Partial = すべてのフィールドが任意）
}>()

// フォームの状態（react で変更を追跡）
const form = reactive<GoalForm>({
  certification_name: props.initial?.certification_name || '',
  master_id: props.initial?.master_id || null,
  target_date: props.initial?.target_date || '',
  status: props.initial?.status || 'exam_date',
  memo: props.initial?.memo || '',
  study_hours: props.initial?.study_hours || 0,
})

// props.initial が更新されたとき（別の目標を編集する際）にフォームを同期
watch(() => props.initial, (val) => {
  if (val) {
    form.certification_name = val.certification_name || ''
    form.master_id = val.master_id || null
    form.target_date = val.target_date || ''
    form.status = val.status || 'exam_date'
    form.memo = val.memo || ''
    form.study_hours = val.study_hours || 0
  }
})

/**
 * 勉強時間を 0.5 時間増やす
 *
 * 浮動小数点誤差を防ぐため `Math.round(値 * 10) / 10` で丸める。
 * 例: 0.1 + 0.5 = 0.6000...01 → Math.round(6.0000...01) / 10 = 0.6
 */
function incrementHours() {
  form.study_hours = Math.round((form.study_hours + 0.5) * 10) / 10
}

/**
 * 勉強時間を 0.5 時間減らす（0 未満にはならない）
 */
function decrementHours() {
  // 0.5 未満は減算しない（マイナスの勉強時間を防ぐ）
  if (form.study_hours >= 0.5) {
    form.study_hours = Math.round((form.study_hours - 0.5) * 10) / 10
  }
}

/**
 * オートコンプリートで資格を選択した時の処理
 */
function onSelect(payload: { name: string; master_id: string | null }) {
  form.certification_name = payload.name
  form.master_id = payload.master_id
}

// 親コンポーネントから form データを参照できるように公開
defineExpose({ form })
</script>
