// ============================================================
// env.d.ts — TypeScript 環境定義ファイル
// ============================================================
// このファイルは TypeScript コンパイラに対して
// 「Vite 環境での型情報」と「.vue ファイルの型」を教える。
//
// 【なぜ必要か】
// TypeScript は標準では .vue ファイルや
// Vite の `import.meta.env.VITE_*` を知らない。
// このファイルがないと型エラーが発生する。

// Vite クライアント側の型定義を読み込む
// これにより import.meta.env.VITE_API_BASE_URL 等の型が使えるようになる
/// <reference types="vite/client" />

// .vue ファイルを TypeScript の import 文で読み込めるようにする型宣言
// 例: import App from './App.vue' が型エラーにならない
declare module '*.vue' {
  // DefineComponent は Vue コンポーネントの型
  import type { DefineComponent } from 'vue'
  // .vue ファイルはデフォルトエクスポートとして DefineComponent を持つ
  const component: DefineComponent<{}, {}, any>
  export default component
}
