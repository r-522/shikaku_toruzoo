// ============================================================
// stores/certification.ts — 所持資格状態管理ストア
// ============================================================
// このファイルは所持資格のデータを Pinia で管理する。
//
// 【ストアの役割】
// - certifications: 一覧データをメモリに保持（キャッシュ）
// - fetchAll: API から全件取得してキャッシュを更新
// - add: 新規追加後にキャッシュの先頭に挿入（再取得不要）
// - update: 更新後にキャッシュ内の該当要素を差し替え
// - remove: 削除後にキャッシュから該当要素を除去
//
// この「オプティミスティック更新」パターンにより、
// 毎回 API を呼ばなくても画面が最新状態を反映できる。

import { defineStore } from 'pinia'
import { ref } from 'vue'
import { certificationApi } from '../api/certification'
import type { Certification, CertificationForm } from '../types'

export const useCertificationStore = defineStore('certification', () => {
  // ---- 状態 ----
  // `ref<Certification[]>([])` = Certification 型の配列（初期値: 空）
  const certifications = ref<Certification[]>([])
  const loading = ref(false)

  /**
   * 所持資格を全件取得してキャッシュを更新する
   *
   * try/finally で loading フラグを確実に解除する。
   */
  async function fetchAll() {
    loading.value = true
    try {
      const { data } = await certificationApi.list()
      // `data.certifications` でレスポン��のラッパーオブジェクトを外す
      certifications.value = data.certifications
    } finally {
      loading.value = false
    }
  }

  /**
   * 所持資格を追加してキャッシュの先頭に挿入する
   *
   * `unshift` は配列の先頭に要素を追加する（末尾追加は `push`）。
   * サーバーから登録日時降順で返るため、新規追加は先頭に入れる。
   * @param form - 資格名・マスタID・取得日
   */
  async function add(form: CertificationForm) {
    const { data } = await certificationApi.create(form)
    certifications.value.unshift(data) // 先頭に追加（新しいものが上に来る）
  }

  /**
   * 所持資格を更新してキャッシュを差し替える
   *
   * @param id - 更新対象のID
   * @param form - 更新後のデータ
   */
  async function update(id: string, form: CertificationForm) {
    const { data } = await certificationApi.update(id, form)
    // `findIndex` で配列内の対象要素の位置を探す
    const idx = certifications.value.findIndex((c) => c.id === id)
    // -1 は「見つからなかった」を意味する
    if (idx !== -1) certifications.value[idx] = data // インデックス直接代入で差し替え
  }

  /**
   * 所持資格を削除してキャッシュから除去する
   *
   * @param id - 削除対象のID
   */
  async function remove(id: string) {
    await certificationApi.remove(id)
    // `filter` で id が一致しない要素だけ残す（= 対象要素を除去）
    certifications.value = certifications.value.filter((c) => c.id !== id)
  }

  return { certifications, loading, fetchAll, add, update, remove }
})
