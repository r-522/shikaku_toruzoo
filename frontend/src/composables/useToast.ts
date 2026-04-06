// ============================================================
// composables/useToast.ts — トースト通知コンポーザブル
// ============================================================
// このファイルはポップアップ通知（トースト）の状態と表示制御を管理する。
//
// 【トースト通知とは】
// 画面の端に一時的に表示される通知メッセージ。
// 「登録しました」「エラーが発生しました」等の短いフィードバックに使う。
//
// 【モジュールスコープの状態（グローバル状態）】
// `ref()` の変数をモジュールの最上位（関数の外）に置くことで、
// このファイルがインポートされるすべての場所で同じインスタンスを共有する。
// （Pinia ストアと異なり、シンプルな共有状態に適している）
//
// 【setTimeout の管理】
// `clearTimeout(timer)` で前回のタイマーをキャンセルしてから
// 新しいタイマーを設定することで、連続呼び出し時も正しく動作する。

import { ref } from 'vue'

// ---- モジュールスコープのリアクティブ状態（全コンポーネントで共有） ----
const message = ref('')                                   // 表示するメッセージ
const type = ref<'success' | 'error' | 'info'>('info')   // 通知の種類（色に影響）
const visible = ref(false)                                // 表示/非表示フラグ

// タイマーの参照を保持（clearTimeout に使う）
// `ReturnType<typeof setTimeout>` は setTimeout の戻り値の型（ブラウザとNode.jsで異なるため）
let timer: ReturnType<typeof setTimeout> | null = null

/**
 * トースト通知を管理するコンポーザブル
 *
 * コンポーネントで:
 * ```vue
 * <script setup>
 * const { show } = useToast()
 * show('保存しました', 'success')
 * </script>
 * ```
 *
 * @returns トースト状態・表示関数・非表示関数
 */
export function useToast() {
  /**
   * トースト通知を表示する
   *
   * @param msg - 表示するメッセージ
   * @param t - 通知の種類（'success' | 'error' | 'info'。デフォルト: 'info'）
   * @param duration - 表示時間（ミリ秒。デフォルト: 3000ms = 3 秒）
   */
  function show(msg: string, t: 'success' | 'error' | 'info' = 'info', duration = 3000) {
    // 状態を更新（Vue のリアクティブシステムが自動的に DOM を更新する）
    message.value = msg
    type.value = t
    visible.value = true

    // 前回のタイマーが残っていればキャンセル（短時間に連続呼び出しされた場合の対策）
    if (timer) clearTimeout(timer)

    // 指定時間後に自動的に非表示にする
    // `setTimeout(コールバック, ミリ秒)` は指定時間後にコールバックを実行する
    timer = setTimeout(() => {
      visible.value = false
    }, duration)
  }

  /**
   * トースト通知を手動で非表示にする
   */
  function hide() {
    visible.value = false
  }

  return { message, type, visible, show, hide }
}
