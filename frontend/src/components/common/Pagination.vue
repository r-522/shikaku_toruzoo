<!-- ============================================================
components/common/Pagination.vue — ページネーションコンポーネント
============================================================
このファイルはページネーション（ページ切り替え）UIを提供する。
総件数・1ページあたりの件数・現在のページを受け取り、
ページ番号ボタンと前後矢印ボタンを表示する。

【totalPages の計算】
`Math.ceil(total / perPage)` で総ページ数を計算する。
例: 合計 45 件、1 ページ 20 件 → Math.ceil(45/20) = Math.ceil(2.25) = 3 ページ

【v-if="totalPages > 1"】
1 ページのみの場合はページネーションを非表示にする（UX 改善）

【@click.prevent について】
`href="#"` は本来 URL の # へジャンプするが、
`@click.prevent` でデフォルトの動作をキャンセルして Vue のハンドラだけ実行する。

【pageChange イベント】
ページボタンをクリックすると `pageChange` イベントを発火し、
親コンポーネントが新しいページ番号を受け取って API を再呼び出しする。
============================================================ -->
<template>
  <!-- totalPages が 2 以上の場合のみページネーションを表示 -->
  <nav v-if="totalPages > 1">
    <ul class="pagination justify-content-center flex-wrap">
      <!-- 前のページへの矢印ボタン（最初のページでは無効化） -->
      <li class="page-item" :class="{ disabled: currentPage <= 1 }">
        <!-- `$emit('pageChange', currentPage - 1)` で前のページ番号を親に通知 -->
        <a class="page-link" href="#" @click.prevent="$emit('pageChange', currentPage - 1)">
          &laquo; <!-- 左向き二重矢印（«） -->
        </a>
      </li>

      <!-- ページ番号ボタン（1 から totalPages まで）
           `v-for="page in totalPages"` は 1, 2, 3, ... と整数を生成する -->
      <li
        v-for="page in totalPages"
        :key="page"
        class="page-item"
        :class="{ active: page === currentPage }" <!-- 現在のページは強調表示 -->
      >
        <a class="page-link" href="#" @click.prevent="$emit('pageChange', page)">
          {{ page }}
        </a>
      </li>

      <!-- 次のページへの矢印ボタン（最後のページでは無効化） -->
      <li class="page-item" :class="{ disabled: currentPage >= totalPages }">
        <a class="page-link" href="#" @click.prevent="$emit('pageChange', currentPage + 1)">
          &raquo; <!-- 右向き二重矢印（»） -->
        </a>
      </li>
    </ul>
  </nav>
</template>

<script setup lang="ts">
import { computed } from 'vue'

// props の取得（`props` 変数として参照するために代入）
const props = defineProps<{
  total: number       // 全件数
  perPage: number     // 1 ページあたりの件数
  currentPage: number // 現在のページ番号
}>()

defineEmits<{
  pageChange: [page: number] // クリックしたページ番号を渡すイベント
}>()

// 総ページ数を計算（computed = props が変わると自動再計算）
const totalPages = computed(() => Math.ceil(props.total / props.perPage))
</script>
