// ============================================================
// types/index.ts — TypeScript 型定義ファイル
// ============================================================
// このファイルはアプリ全体で使用する TypeScript の型・インターフェースを定義する。
//
// 【型定義の目的】
// TypeScript では変数や引数に「型」を付けることで、
// 間違った値の代入やプロパティの参照ミスをコンパイル時に検出できる。
//
// 【interface と type の違い】
// - interface: オブジェクトの形状（プロパティとその型）を定義する
// - type: エイリアス。複数の値の中の一つ（ユニオン型等）に使う
//
// このファイルの型は API レスポンス・フォームデータ両方で使う。

// ============================================================
// ユーザー関連
// ============================================================

/** ログイン中ユーザーの基本情報 */
export interface User {
  id: string          // ユーザーID（UUID 形式の文字列）
  username: string    // ユーザー名（表示名）
  created_at: string  // アカウント作成日時（ISO 8601 形式）
}

// ============================================================
// 所持資格関連
// ============================================================

/** 所持資格（TBL_HOLDING）のデータ型 */
export interface Certification {
  id: string                    // 所持資格 ID（UUID）
  certification_name: string    // 資格名（TBL_MASTER から取得）
  master_id: string             // 資格マスタ ID
  acquired_date: string | null  // 取得日（未設定の場合は null）
  created_at: string            // 登録日時
}

// ============================================================
// 目標関連
// ============================================================

/**
 * 目標のステータス型（ユニオン型）
 *
 * 4 つの値のいずれかしか取れない文字列型。
 * - exam_date: 受験日が設定された（目標設定中）
 * - passed: 合格
 * - failed: 不合格（再挑戦可能）
 * - abandoned: 断念
 */
export type GoalStatus = 'exam_date' | 'passed' | 'failed' | 'abandoned'

/** 目標（TBL_GOAL）のデータ型 */
export interface Goal {
  id: string                  // 目標 ID（UUID）
  certification_name: string  // 資格名（TBL_MASTER から取得）
  master_id: string           // 資格マスタ ID
  target_date: string         // 目標日（YYYY-MM-DD 形式）
  status: GoalStatus          // ステータス（上記 GoalStatus のいずれか）
  memo: string | null         // メモ（null 許容）
  study_hours: number         // 勉強時間（0.0〜の小数）
  created_at: string          // 登録日時
}

// ============================================================
// 資格マスタ関連
// ============================================================

/** 資格マスタ検索結果（オートコンプリート用） */
export interface MasterCertification {
  id: string        // マスタ ID（UUID）
  name: string      // 資格名（表示用）
  category: string  // カテゴリ（例: "IT", "語学", "その他"）
}

// ============================================================
// コミュニティ関連
// ============================================================

/** コミュニティ画面で表示する「所持資格」の簡易版 */
export interface CommunityCert {
  certification_name: string    // 資格名
  acquired_date: string | null  // 取得日
}

/** コミュニティ画面で表示する「目標」の簡易版 */
export interface CommunityGoal {
  certification_name: string  // 資格名
  status: GoalStatus          // ステータス
  study_hours: number         // 勉強時間
  target_date: string         // 目標日
}

/** コミュニティ一覧画面の 1 ユーザー分のデータ */
export interface CommunityUser {
  id: string                      // ユーザー ID
  username: string                // ユーザー名
  certification_count: number     // 所持資格数
  goal_count: number              // 目標数
  achieved_count: number          // 合格数（status === 'passed' の件数）
  total_study_hours: number       // 総勉強時間
  has_good_mark: boolean          // グッドマーク（1つ以上合格していれば true）
  is_favorite: boolean            // 自分がお気に入り登録しているか
  certifications: CommunityCert[] // 所持資格リスト
  goals: CommunityGoal[]          // 目標リスト
}

/** ユーザー詳細画面のデータ（資格・目標の詳細情報を含む） */
export interface CommunityUserDetail {
  id: string                  // ユーザー ID
  username: string            // ユーザー名
  has_good_mark: boolean      // グッドマーク
  certifications: Certification[] // 資格の詳細リスト
  goals: Goal[]               // 目標の詳細リスト
}

// ============================================================
// ページネーション
// ============================================================

/**
 * ページネーション付きレスポンスのジェネリック型
 *
 * `<T>` はジェネリック（型パラメータ）。
 * 使用時に具体的な型を指定する（例: PaginatedResponse<CommunityUser>）。
 */
export interface PaginatedResponse<T> {
  users: T[]        // ユーザーのリスト（型は T で柔軟に指定）
  total: number     // 全件数（ページネーション前）
  page: number      // 現在のページ番号
  per_page: number  // 1 ページあたりの件数
}

// ============================================================
// フォームデータ型
// ============================================================

/** サインアップ（新規登録）フォームのデータ型 */
export interface SignUpForm {
  username: string  // ユーザー名
  email: string     // メールアドレス
  password: string  // パスワード
}

/** サインイン（ログイン）フォームのデータ型 */
export interface SignInForm {
  email: string     // メールアドレス
  password: string  // パスワード
}

/** 所持資格登録・更新フォームのデータ型 */
export interface CertificationForm {
  certification_name: string   // 資格名（手入力またはオートコンプリートで選択）
  master_id: string | null     // マスタ ID（補完から選択した場合。手入力は null）
  acquired_date: string        // 取得日（YYYY-MM-DD 形式）
}

/** 目標登録・更新フォームのデータ型 */
export interface GoalForm {
  certification_name: string   // 資格名
  master_id: string | null     // マスタ ID（null なら手入力）
  target_date: string          // 目標日（YYYY-MM-DD 形式）
  status: GoalStatus           // ステータス
  memo: string                 // メモ（空文字列 = なし）
  study_hours: number          // 勉強時間（数値）
}
