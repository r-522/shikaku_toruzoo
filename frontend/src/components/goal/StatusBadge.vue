<!-- ============================================================
components/goal/StatusBadge.vue — ステータスバッジコンポーネント
============================================================
このファイルは目標のステータスを色付きバッジで表示するコンポーネント。
ステータス文字列（exam_date, passed 等）を日本語ラベルと色に変換して表示する。

【statusMap について】
`Record<GoalStatus, { label: string; color: string }>` は
TypeScript のマップ型。GoalStatus の全値に対して
label と color を定義している（型安全な辞書）。

【オプショナルチェーン (?.) とヌル合体演算子 (??) 】
`statusMap[props.status]?.label ?? props.status` の意味:
1. `statusMap[props.status]` が存在する場合: `.label` を返す
2. 存在しない（undefined）場合: `??` の右側 `props.status` を返す（フォールバック）
============================================================ -->
<template>
  <!-- `:style="{ backgroundColor: color }"` で動的にスタイルを設定 -->
  <!-- `badge` は Bootstrap のバッジスタイル（丸みのある小さなタグ） -->
  <span class="badge" :style="{ backgroundColor: color }">{{ label }}</span>
</template>

<script setup lang="ts">
import { computed } from 'vue'
// GoalStatus はユニオン型（'exam_date' | 'passed' | 'failed' | 'abandoned'）
import type { GoalStatus } from '../../types'

const props = defineProps<{
  status: GoalStatus  // 表示するステータス値
}>()

// ステータス値から表示ラベルと色へのマッピング
// `Record<キー型, 値型>` は TypeScript のオブジェクト型
const statusMap: Record<GoalStatus, { label: string; color: string }> = {
  exam_date: { label: '受験日', color: '#1565C0' }, // 青（目標設定中）
  passed:    { label: '合格',   color: '#2E7D32' }, // 緑（成功）
  failed:    { label: '不合格', color: '#E65100' }, // オレンジ（失敗）
  abandoned: { label: '断念',   color: '#616161' }, // グレー（断念）
}

// computed: props.status が変わると自動的に再計算される
// `?.` = オプショナルチェーン（存在確認してからアクセス）
// `??` = ヌル合体演算子（null/undefined の場合のデフォルト値）
const label = computed(() => statusMap[props.status]?.label ?? props.status)
const color = computed(() => statusMap[props.status]?.color ?? '#757575')
</script>
