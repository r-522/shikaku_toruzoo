// ============================================================
// main.ts — フロントエンドのエントリポイント
// ============================================================
// このファイルは Vue アプリケーションの起動処理を行う。
// HTML の `<div id="app">` にアプリケーションをマウントする。
//
// 【処理の流れ】
// 1. Vue アプリインスタンスを作成
// 2. Pinia（状態管理）を登録
// 3. Vue Router（ルーティング）を登録
// 4. グローバルスタイルを読み込む
// 5. `#app` 要素にマウント（HTML の `<div id="app">` を Vue が管理する）

// Vue コアの createApp 関数（アプリインスタンスを作成する）
import { createApp } from 'vue'
// Pinia: Vue 3 向けの状態管理ライブラリ（Vuex の後継）
// アプリ全体で共有するデータ（ユーザー情報等）を管理する
import { createPinia } from 'pinia'
// ルートコンポーネント（アプリのベースとなる .vue ファイル）
import App from './App.vue'
// Vue Router の設定（router/index.ts で定義）
import router from './router'
// グローバル SCSS（全コンポーネントに適用されるスタイル）
import './assets/styles/main.scss'

// Vue アプリインスタンスを作成（`App.vue` がルートコンポーネント）
const app = createApp(App)

// Pinia プラグインをアプリに登録
// これにより各コンポーネントで `useXxxStore()` が使えるようになる
app.use(createPinia())

// Vue Router をアプリに登録
// これにより `<RouterView>` や `useRouter()` が使えるようになる
app.use(router)

// `#app` 要素（index.html の `<div id="app">`）に Vue アプリをマウント
// この瞬間から Vue が DOM を管理する
app.mount('#app')
