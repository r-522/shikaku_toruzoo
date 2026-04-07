<!-- ============================================================
components/goal/GoalCard.vue — 目標カードコンポーネント
============================================================
このファイルは 1 件の学習目標を表示するカードコンポーネント。
資格名・ステータスバッジ・目標日・残り日数・勉強時間・メモを表示する。

【残り日数の計算】
`daysRemaining` computed で今日から目標日までの日数を計算する。
- 正の値: 残り N 日
- 0: 本日
- 負の値: N 日超過（超過した場合はオレンジ色で強調）

【Math.ceil について】
`Math.ceil(x)` は小数点以下を切り上げる関数。
日数計算でミリ秒を使うため、端数が出た場合に切り上げる。
例: 1.2 日 → 2 日（余裕を持って表示）

【text-truncate について】
Bootstrap のクラス。テキストが長い場合に「...」で省略表示する。
`max-width` と組み合わせて使う。
============================================================ -->
<template>
  <div class="card shadow-sm mb-2">
    <div class="card-body py-3">
      <!-- カードの上部: 資格名・バッジ・操作ボタン -->
      <div class="d-flex justify-content-between align-items-start">
        <!-- 左側: 情報 -->
        <div class="flex-grow-1">
          <!-- 資格名とステータスバッジ -->
          <div class="d-flex align-items-center gap-2 mb-1">
            <span class="fw-bold">{{ goal.certification_name }}</span>
            <!-- StatusBadge コンポーネントにステータスを渡して色付きバッジを表示 -->
            <StatusBadge :status="goal.status" />
          </div>

          <!-- 目標日・残り日数・勉強時間（flex-wrap でモバイル折り返し対応） -->
          <div class="d-flex gap-2 flex-wrap">
            <small class="text-muted">目標日: {{ goal.target_date }}</small>
            <!-- daysColor で残り日数の色を動的に設定（超過時はオレンジ） -->
            <small :style="{ color: daysColor }">{{ daysText }}</small>
            <!-- 勉強時間が 0 の場合は表示しない（v-if で制御） -->
            <small v-if="goal.study_hours > 0" class="text-muted">{{ goal.study_hours }}h 勉強</small>
          </div>

          <!-- メモ（存在する場合のみ表示） -->
          <!-- text-truncate: 長い場合は省略（...）で表示 -->
          <div v-if="goal.memo" class="text-muted small mt-1 text-truncate" style="max-width: 100%">
            {{ goal.memo }}
          </div>
        </div>

        <!-- 右側: 操作ボタン -->
        <div class="ms-2">
          <button class="btn btn-outline-secondary btn-sm me-1" @click="$emit('edit', goal)">編集</button>
          <button class="btn btn-outline-danger btn-sm" @click="$emit('delete', goal)">削除</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import StatusBadge from './StatusBadge.vue'
import type { Goal } from '../../types'

const props = defineProps<{
  goal: Goal  // 表示する目標データ
}>()

defineEmits<{
  edit: [goal: Goal]    // 編集ボタンが押された
  delete: [goal: Goal]  // 削除ボタンが押された
}>()

// ---- 残り日数の計算 ----

/**
 * 今日から目標日までの残り日数を計算する
 *
 * `new Date(props.goal.target_date)` で目標日の Date オブジェクトを作成
 * `.getTime()` でミリ秒に変換して差を計算
 * `1000 * 60 * 60 * 24` = 1 日のミリ秒数
 * `Math.ceil()` で切り上げ（少数が出た場合に余裕を持って表示）
 */
const daysRemaining = computed(() => {
  const target = new Date(props.goal.target_date)
  const today = new Date()
  // 今日の時刻部分をゼロにする（「本日 0 時」からの日数で計算）
  today.setHours(0, 0, 0, 0)
  return Math.ceil((target.getTime() - today.getTime()) / (1000 * 60 * 60 * 24))
})

/**
 * 残り日数を表示文字列に変換する
 *
 * - 超過: "N 日超過"
 * - 本日: "本日"
 * - 残り: "残り N 日"
 */
const daysText = computed(() => {
  const d = daysRemaining.value
  if (d < 0) return `${Math.abs(d)}日超過`  // Math.abs: 絶対値（負の数を正にする）
  if (d === 0) return '本日'
  return `残り${d}日`
})

// 超過した場合はオレンジ色、通常はグレー
const daysColor = computed(() => daysRemaining.value < 0 ? '#E65100' : '#757575')
</script>
