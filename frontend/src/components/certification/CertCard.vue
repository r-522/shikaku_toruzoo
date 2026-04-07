<!-- ============================================================
components/certification/CertCard.vue — 所持資格カードコンポーネント
============================================================
このファイルは 1 件の所持資格を表示するカードコンポーネント。
資格名・取得日・編集ボタン・削除ボタンを表示する。

【emits の設計】
このコンポーネントはデータを「表示するだけ」で、
実際の編集・削除処理は親コンポーネント（DashboardView.vue）が行う。
emit('edit', cert) で「編集してほしい資格データ」を親に渡す。

【|| による代替表示】
`cert.acquired_date || '日付未設定'` は
acquired_date が null/空文字列の場合に '日付未設定' を表示する。
============================================================ -->
<template>
  <!-- card shadow-sm mb-2: Bootstrap のカードスタイル。薄い影付き、下マージンあり -->
  <div class="card shadow-sm mb-2">
    <div class="card-body d-flex justify-content-between align-items-center py-3">
      <!-- 左側: 資格情報（min-width:0 で長い資格名がボタンを押し出すのを防ぐ） -->
      <div style="min-width: 0">
        <!-- 資格名（text-truncate で長い場合は省略） -->
        <div class="fw-bold text-truncate">{{ cert.certification_name }}</div>
        <!-- 取得日（未設定の場合は代替テキスト） -->
        <small class="text-muted">{{ cert.acquired_date || '日付未設定' }}</small>
      </div>

      <!-- 右側: 操作ボタン（flex-shrink-0 で縮まないよう固定） -->
      <div class="flex-shrink-0">
        <!-- 編集ボタン: クリックで親に cert データを渡す -->
        <button class="btn btn-outline-secondary btn-sm me-1" @click="$emit('edit', cert)">編集</button>
        <!-- 削除ボタン: クリックで親に cert データを渡す（確認ダイアログは親が担当） -->
        <button class="btn btn-outline-danger btn-sm" @click="$emit('delete', cert)">削除</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Certification } from '../../types'

defineProps<{
  cert: Certification  // 表示する資格データ
}>()

defineEmits<{
  edit: [cert: Certification]    // 編集ボタンが押された（渡すデータ: 対象の資格）
  delete: [cert: Certification]  // 削除ボタンが押された
}>()
</script>
