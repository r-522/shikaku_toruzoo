// ============================================================
// composables/useDebounce.ts — デバウンスコンポーザブル
// ============================================================
// このファイルは「デバウンス」（連続し��呼び出しを間引く）機能を提供する。
//
// 【デバウンスとは】
// 短時間に連続して呼ばれる関数の実行を遅らせ、
// 「最後の呼び出しから一定時間後に 1 回だけ実行する」仕組み。
//
// 【使用場面: オートコンプリート】
// ユーザーが「AWS Solutions Architect」と入力するとき、
// デバウンスなし: 1文字入力ごとに API を呼ぶ（12 回）
// デバウンスあり（300ms）: 入力が止まってから 300ms 後に 1 回だけ API を呼ぶ
//
// 【TypeScript ジェネリクスの説明】
// `<T extends (...args: unknown[]) => void>` は
// 「関数型を受け取り、その引数と戻り値の型を保持する」型パラメータ。
// これにより debounced 関数の引数が元の fn と同じ型を持つことが保証される。

/**
 * 関数をデバウンスするコンポーザブル
 *
 * @param fn - デバウンス対象の関数
 * @param delay - 待機時間（ミリ秒。デフォルト: 300ms）
 * @returns `{ debounced: デバウンス済み関数, cancel: タイマーキャンセル関数 }`
 *
 * @example
 * ```ts
 * const { debounced: searchDebounced } = useDebounce(searchApi, 300)
 * // 入力イベントで searchDebounced を呼ぶ
 * // 最後の入力から 300ms 後に searchApi が 1 回だけ実行される
 * ```
 */
export function useDebounce<T extends (...args: unknown[]) => void>(fn: T, delay = 300) {
  // タイマーの参照（クロージャで保持する）
  let timer: ReturnType<typeof setTimeout> | null = null

  /**
   * デバウンス済みの関数
   *
   * `...args: Parameters<T>` は「元の fn と同じ引数の型」を表す
   * TypeScript のマップ型。これにより型安全に引数を転送できる。
   */
  function debounced(...args: Parameters<T>) {
    // 前回のタイマーをキャンセル（リセット）
    if (timer) clearTimeout(timer)
    // 新しいタイマーをセット: delay ms 後に fn を元の引数で呼び出す
    timer = setTimeout(() => fn(...args), delay)
  }

  /**
   * 未発火のタイマーをキャンセルする
   *
   * コンポーネントのアンマウント時等に呼び出してリソースを解放する。
   */
  function cancel() {
    if (timer) clearTimeout(timer)
  }

  return { debounced, cancel }
}
