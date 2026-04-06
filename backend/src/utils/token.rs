// ============================================================
// utils/token.rs — セッショントークン生成
// ============================================================
// セッショントークンとは「ユーザーがログイン済みであること」を証明するランダム文字列。
// Cookie に保存し、リクエストのたびにサーバーへ送信される。
// 予測不可能な乱数で生成することで、他人がトークンを「当てる」ことができないようにする。

// Base64 エンコード用ライブラリ（バイト列 → 文字列に変換する際に使用）
use base64::Engine;
// 暗号学的乱数生成器（OS の安全な乱数源を使用）
use rand::RngCore;

/// 256ビット（32バイト）の暗号学的乱数を生成し、Base64文字列として返す
///
/// # なぜ「暗号学的乱数」を使うのか？
/// 通常の `rand()` は予測可能な場合がある。
/// 暗号学的乱数は予測不可能なため、セッション固定攻撃などを防止できる。
///
/// # 戻り値
/// 44文字の Base64 文字列（256bit = 32bytes → Base64エンコードで44文字）
///
/// # 使用例
/// ```
/// let token = generate_session_token();
/// // 例: "dGVzdHRva2VudGVzdHRva2VudGVzdHRva2VudGVzdA=="
/// ```
pub fn generate_session_token() -> String {
    // 32バイト（= 256ビット）のゼロ初期化配列を作成
    // `[0u8; 32]` は「u8型の0を32個並べた配列」という意味
    let mut bytes = [0u8; 32];

    // `rand::thread_rng()` でスレッドローカルな乱数生成器を取得
    // `.fill_bytes(&mut bytes)` で配列を乱数で埋める
    // ⚠️ `rand` の乱数は OS の乱数源（`/dev/urandom` など）を使うため暗号学的に安全
    rand::thread_rng().fill_bytes(&mut bytes);

    // バイト列を Base64 文字列にエンコードして返す
    // Base64 は英数字と一部記号だけで構成されるため、Cookie 値として安全に使える
    // `STANDARD` は標準的な Base64 エンコード方式（`+`, `/`, `=` を含む）
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

// ============================================================
// テストコード
// ============================================================
// `#[cfg(test)]` は `cargo test` 実行時のみコンパイルされる。
// 本番ビルドには含まれないため、バイナリサイズへの影響なし。
#[cfg(test)]
mod tests {
    // `super::*` は親モジュール（このファイル）の全シンボルをインポート
    use super::*;

    /// トークンの長さが44文字であることを検証
    /// 32バイト → Base64エンコード → 44文字 (32 * 4/3 を切り上げてパディング)
    #[test]
    fn test_token_length() {
        let token = generate_session_token();
        assert_eq!(token.len(), 44);
    }

    /// 2回生成したトークンが異なることを検証（乱数の一意性）
    #[test]
    fn test_token_uniqueness() {
        let t1 = generate_session_token();
        let t2 = generate_session_token();
        // `assert_ne!` は「等しくないこと」を確認するマクロ
        assert_ne!(t1, t2);
    }
}
