<!-- ============================================================
App.vue — ルートコンポーネント
============================================================
このファイルはアプリケーション全体のレイアウトを定義する。
すべての画面はこのコンポーネントの中に表示される。

【レイアウト構造】
AppHeader（ナビゲーションバー）
  ↓
main（RouterView: 現在の URL に対応する View を表示）
  ↓
AppFooter（フッター）
  ↓
ToastNotification（画面右上のポップアップ通知・常に表示可能）

【d-flex flex-column min-vh-100 について】
Bootstrap のユーティリティクラスを使ったフレックスボックスレイアウト。
- d-flex: display: flex
- flex-column: 縦方向に並べる
- min-vh-100: 最小高さをビューポートの 100% に
フッターが画面下部に固定される（sticky footer）効果がある。
============================================================ -->
<template>
  <!-- 全体を縦並びフレックスで包む（ヘッダー・コンテンツ・フッターを縦に配置） -->
  <div class="d-flex flex-column min-vh-100">
    <!-- ナビゲーションバー（全ページ共通） -->
    <AppHeader />

    <!-- メインコンテンツ領域
         flex-grow-1: 余白を全て使って縦に伸びる（フッターを下に押し下げる） -->
    <main class="flex-grow-1">
      <!-- RouterView: Vue Router が現在の URL に対応するコンポーネントを表示する -->
      <router-view />
    </main>

    <!-- フッター（全ページ共通） -->
    <AppFooter />

    <!-- トースト通知コンポーネント（画面右上に重ね表示される。position: fixed） -->
    <ToastNotification />
  </div>
</template>

<script setup lang="ts">
// レイアウトコンポーネント
import AppHeader from './components/layout/AppHeader.vue'
import AppFooter from './components/layout/AppFooter.vue'
// グローバルトースト通知コンポーネント（useToast() と連動）
import ToastNotification from './components/common/ToastNotification.vue'
</script>
