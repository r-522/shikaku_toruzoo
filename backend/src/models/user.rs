// ============================================================
// models/user.rs — ユーザーモデル（TBL_USER）
// ============================================================
// ユーザー登録・ログイン・ユーザー情報取得に使用するデータ構造を定義する。
//
// 【重要】DB のパスワード・メールアドレスはハッシュ化済みの値が入っている。
// User 構造体の useml はハッシュ値、usepw もハッシュ値である。
// 平文の値が DB に保存されることは絶対にない。

use serde::{Deserialize, Serialize};
use uuid::Uuid;
// validator クレートを使ってフィールドの入力バリデーションを定義する
use validator::Validate;

/// TBL_USER テーブルのレコードに対応する構造体（DB取得用）
///
/// DB カラム名の対応:
/// - useid → ユーザーID（主キー）
/// - usenm → ユーザー名（表示名）
/// - useml → メールアドレス（HMAC-SHA256ハッシュ値が入る）
/// - usepw → パスワード（Argon2idハッシュ値が入る）
/// - useca → 作成日時
/// - useua → 更新日時
#[derive(Debug, Deserialize)]
pub struct User {
    pub useid: Uuid,
    pub usenm: String,
    /// ⚠️ DB に保存されているのは HMAC-SHA256 ハッシュ値（平文ではない）
    pub useml: String,
    /// ⚠️ DB に保存されているのは Argon2id ハッシュ値（平文ではない）
    pub usepw: String,
    pub useca: String,
    pub useua: String,
}

// ============================================================
// リクエストボディの構造体
// ============================================================
// `Validate` を derive することで、`.validate()` メソッドが自動生成される。
// `#[validate(...)]` アトリビュートで各フィールドの制約を宣言的に記述できる。

/// サインアップ（新規登録）リクエストのボディ
///
/// フロントエンドから POST /api/auth/signup に送信される JSON に対応する。
#[derive(Debug, Deserialize, Validate)]
pub struct SignupRequest {
    /// ユーザー名: 3〜30文字
    #[validate(length(min = 3, max = 30, message = "ユーザー名は3〜30文字で入力してください"))]
    pub username: String,

    /// メールアドレス: メール形式チェック
    /// ⚠️ DB保存前に HMAC-SHA256 でハッシュ化される
    #[validate(email(message = "有効なメールアドレスを入力してください"))]
    pub email: String,

    /// パスワード: 8文字以上
    /// ⚠️ DB保存前に Argon2id でハッシュ化される
    #[validate(length(min = 8, message = "パスワードは8文字以上で入力してください"))]
    pub password: String,
}

/// サインイン（ログイン）リクエストのボディ
///
/// フロントエンドから POST /api/auth/signin に送信される JSON に対応する。
#[derive(Debug, Deserialize, Validate)]
pub struct SigninRequest {
    /// メールアドレス（照合時にハッシュ化してDB検索に使用）
    #[validate(email(message = "有効なメールアドレスを入力してください"))]
    pub email: String,

    /// パスワード（照合時にハッシュと比較する）
    #[validate(length(min = 1, message = "パスワードを入力してください"))]
    pub password: String,
}

// ============================================================
// レスポンスの構造体
// ============================================================
// `Serialize` を derive することで `.json()` メソッドで自動的に JSON に変換できる。
// ⚠️ レスポンスにはパスワードやメールアドレスを絶対に含めない。

/// GET /api/auth/me のレスポンスボディ
///
/// 認証済みユーザーの情報を返す。機密情報（パスワード・メール）は含まない。
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// ユーザーの一意識別子
    pub id: Uuid,
    /// 表示名
    pub username: String,
    /// アカウント作成日時（ISO8601形式）
    pub created_at: String,
}
