// ============================================================
// utils/hash.rs — パスワード・メールアドレスのハッシュ化
// ============================================================
// 「ハッシュ化」とは、元のデータから一方向に変換した固定長の文字列を生成すること。
// 元のデータ（パスワード等）はDBに保存せず、ハッシュのみを保存する。
// これにより、DBが漏洩してもパスワードが直接わからない。

// Argon2 ライブラリのインポート
// Argon2id は OWASP（セキュリティ標準団体）が推奨する最もセキュアなパスワードハッシュアルゴリズム
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
// HMAC（Hash-based Message Authentication Code）ライブラリ
// メールアドレスのハッシュ化に使用
use hmac::{Hmac, Mac};
// SHA-256 ハッシュアルゴリズム
use sha2::Sha256;

// 自分のモジュールのエラー型をインポート
use crate::errors::AppError;

// `type` で型エイリアスを定義。`HmacSha256` は「SHA-256を使ったHMAC」の短縮名
// これにより長い型名を毎回書かなくて済む
type HmacSha256 = Hmac<Sha256>;

// ============================================================
// Argon2id インスタンスの生成（内部ヘルパー）
// ============================================================
/// Argon2id ハッシュ化器のインスタンスを生成する（内部用）
///
/// パラメータの意味:
/// - メモリ: 65536 KB（64 MB）— ブルートフォースをメモリコストで困難にする
/// - 反復回数: 3 回 — CPU コストを増やす
/// - 並列度: 4 スレッド
/// - ハッシュ長: 32 バイト
fn argon2_instance() -> Argon2<'static> {
    // `Params::new(メモリKB, 反復回数, 並列度, 出力バイト数)` でパラメータ設定
    let params = Params::new(65536, 3, 4, Some(32)).expect("Invalid Argon2 params");
    // `Algorithm::Argon2id` は Argon2 の最も安全なバリアント（id = サイドチャネル耐性あり）
    // `Version::V0x13` は Argon2 のバージョン（現在の標準: 0x13 = 19）
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

// ============================================================
// パスワードハッシュ化
// ============================================================
/// パスワードを Argon2id でハッシュ化する
///
/// # 引数
/// - `password`: 平文パスワード（ユーザーが入力したもの）
///
/// # 戻り値
/// PHC 文字列形式のハッシュ（例: `$argon2id$v=19$m=65536,t=3,p=4$...`）
/// この文字列にはソルト・パラメータ・ハッシュ値がすべて含まれる。
///
/// # 初学者向け補足
/// 「ソルト」とは、同じパスワードが同じハッシュにならないよう付加する乱数のこと。
/// ソルトがないと「レインボーテーブル（ハッシュの辞書）」による攻撃が可能になる。
pub fn hash_password(password: &str) -> Result<String, AppError> {
    // `SaltString::generate` で安全な乱数ソルトを生成
    // `OsRng` は OS の乱数源を使うため暗号学的に安全
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = argon2_instance();

    // `.hash_password(パスワードのバイト列, ソルト)` でハッシュ化
    // 失敗した場合は `AppError::Internal` に変換（`.map_err` で変換）
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Password hash error: {}", e)))?;

    // `.to_string()` で PHC 文字列形式に変換して返す
    Ok(hash.to_string())
}

// ============================================================
// パスワード検証
// ============================================================
/// 平文パスワードとハッシュが一致するか検証する
///
/// # 引数
/// - `hash`: DB に保存されているハッシュ文字列（PHC 形式）
/// - `password`: ユーザーが入力した平文パスワード
///
/// # 戻り値
/// 一致する場合 `true`、しない場合 `false`
///
/// # 注意（タイミング攻撃対策）
/// 内部で一定時間比較を行っている。`hash == hash_password(password)` のような
/// 単純な文字列比較は使ってはいけない（処理時間の差から情報が漏れる）。
pub fn verify_password(hash: &str, password: &str) -> Result<bool, AppError> {
    // PHC 文字列を `PasswordHash` 型にパース（ソルト・パラメータ等を取り出す）
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("Password hash parse error: {}", e)))?;

    let argon2 = argon2_instance();

    // `.verify_password` は一致すれば `Ok(())`、不一致なら `Err(...)` を返す
    // `.is_ok()` で bool に変換（Ok → true、Err → false）
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

// ============================================================
// メールアドレスハッシュ化
// ============================================================
/// メールアドレスを HMAC-SHA256 でハッシュ化する
///
/// # なぜパスワードと別のアルゴリズムを使うのか？
/// メールアドレスはログイン時に「検索」が必要（WHERE useml = ?）。
/// Argon2id はランダムソルトを使うため同じ入力でも毎回異なるハッシュになり、検索できない。
/// HMAC-SHA256 は同じ入力・同じキーなら常に同じハッシュになるため、検索可能。
///
/// # 引数
/// - `email`: 平文メールアドレス
/// - `secret`: サーバーが管理するシークレットキー（`EMAIL_HMAC_SECRET` 環境変数）
///
/// # 戻り値
/// 64文字の16進数（Hex）文字列
///
/// # 正規化
/// 大文字小文字・前後空白を統一することで、`Test@Example.COM` と `test@example.com` が
/// 同じハッシュを持つようにする（同じメールアドレスとみなす）。
pub fn hash_email(email: &str, secret: &str) -> String {
    // 正規化: 前後の空白を除去し、すべて小文字に変換
    let normalized = email.trim().to_lowercase();

    // HMAC-SHA256 のインスタンスを作成（シークレットキーをHMACキーとして使用）
    // `.new_from_slice` はキーのバイト列からインスタンスを作成する
    // HMAC はどんな長さのキーでも使えるため `.expect` はほぼ失敗しない
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");

    // メールアドレスのバイト列を HMAC 計算に追加する
    mac.update(normalized.as_bytes());

    // HMAC 計算を完了し、結果のバイト列を取得
    let result = mac.finalize();

    // バイト列を16進数（Hex）文字列にエンコードして返す
    // `into_bytes()` で `GenericArray` → バイト列に変換
    hex::encode(result.into_bytes())
}

// ============================================================
// テストコード
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    /// パスワードのハッシュ化と検証が正しく動作するか確認
    #[test]
    fn test_hash_password_and_verify() {
        let password = "Test1234!";
        let hash = hash_password(password).unwrap();
        // PHC 文字列の先頭が "$argon2id$" であることを確認
        assert!(hash.starts_with("$argon2id$"));
        // 同じパスワードで検証 → true になるべき
        assert!(verify_password(&hash, password).unwrap());
        // 異なるパスワードで検証 → false になるべき
        assert!(!verify_password(&hash, "WrongPass1!").unwrap());
    }

    /// 同じパスワードを2回ハッシュしても異なるハッシュになることを確認（ソルトの一意性）
    #[test]
    fn test_hash_uniqueness() {
        let h1 = hash_password("Test1234!").unwrap();
        let h2 = hash_password("Test1234!").unwrap();
        // ソルトがランダムなため、同じ入力でも異なるハッシュになる
        assert_ne!(h1, h2);
    }

    /// 同じメールアドレス・同じシークレットなら常に同じハッシュになることを確認
    #[test]
    fn test_hash_email_deterministic() {
        let secret = "test-secret-key-32bytes-minimum!!";
        let h1 = hash_email("test@example.com", secret);
        let h2 = hash_email("test@example.com", secret);
        // 決定論的：同じ入力 → 同じ出力
        assert_eq!(h1, h2);
        // SHA-256 は 32バイト = 64 Hex 文字
        assert_eq!(h1.len(), 64);
    }

    /// メールアドレスの大文字小文字・前後空白を正規化して同一ハッシュになることを確認
    #[test]
    fn test_hash_email_normalization() {
        let secret = "test-secret-key-32bytes-minimum!!";
        let h1 = hash_email("Test@Example.COM", secret);
        let h2 = hash_email("  test@example.com  ", secret);
        // 正規化後は同じ文字列になるため、同じハッシュになるべき
        assert_eq!(h1, h2);
    }

    /// シークレットが異なれば、同じメールアドレスでも異なるハッシュになることを確認
    #[test]
    fn test_hash_email_different_secret() {
        let h1 = hash_email("test@example.com", "secret1");
        let h2 = hash_email("test@example.com", "secret2");
        // キーが変わればHMACの結果も変わる
        assert_ne!(h1, h2);
    }
}
