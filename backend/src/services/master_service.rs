// ============================================================
// services/master_service.rs — 資格マスタビジネスロジック
// ============================================================
// このファイルは「資格マスタ（TBL_MASTER）」の検索・作成を担うサービス層。
//
// 【TBL_MASTER とは】
// 資格の「正式名称・カテゴリ・正規化名」を管理するマスタテーブル。
// 複数ユーザーが同じ資格を登録する場合、同じ TBL_MASTER レコードを参照する。
//
// 【正規化名（masnr）の目的】
// 「AWS Solutions Architect」と「aws solutions architect」と
// 「AWS  Solutions  Architect」は同じ資格を指す可能性が高い。
// normalize_name 関数で小文字化・トリム・空白除去を行い、
// 正規化名で重複チェックをすることで「実質的に同じ名前」の重複を防ぐ。
//
// 【find_or_create パターン】
// ユーザーが手入力した資格名を DB に保存する際、
// 既存マスタを再利用（find）するか、なければ作成（create）するパターン。
// このパターンにより、複数ユーザーが同じ資格を記録しても
// マスタレコードが増えすぎない。

use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;
use crate::models::master::MasterSearchResult;

// ============================================================
// normalize_name — 資格名の正規化
// ============================================================
/// 資格名を正規化する（検索・重複チェック用）
///
/// 正規化ルール:
/// 1. 前後の空白を除去（trim）
/// 2. すべての文字を小文字に変換（to_lowercase）
/// 3. 空白文字（スペース・タブ等）をすべて除去
///
/// # 引数
/// - `name`: 元の資格名
///
/// # 戻り値
/// 正規化後の文字列
///
/// # 使用例
/// ```
/// normalize_name("  AWS Solutions Architect  ")
/// // → "awssolutionsarchitect"
/// normalize_name("aws solutions architect")
/// // → "awssolutionsarchitect"（同じ結果）
/// ```
pub fn normalize_name(name: &str) -> String {
    name.trim()              // 前後の空白を除去する
        .to_lowercase()      // 大文字→小文字（"AWS" → "aws"）
        .chars()             // 文字列を1文字ずつのイテレータに変換
        // `filter` でホワイトスペース（空白・タブ・改行）を除去
        // `!c.is_whitespace()` = 「空白でない文字だけ残す」
        .filter(|c| !c.is_whitespace())
        .collect()           // イテレータを String に収集
}

// ============================================================
// search — 資格名のインクリメンタル検索
// ============================================================
/// 資格名でインクリメンタル検索する（オートコンプリート用）
///
/// 入力が 2 文字以上の場合に限り検索を実行し、最大 10 件を返す。
/// PostgREST の `ilike` 演算子（大文字小文字無視のワイルドカードマッチ）を使用。
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `query`: 検索キーワード（2 文字未満は空配列を返す）
///
/// # 戻り値
/// マッチした資格マスタのリスト（最大 10 件）
///
/// # 注意
/// `ilike=*keyword*` は「keyword を含む」という部分一致検索
/// （SQL の `ILIKE '%keyword%'` に相当）
pub async fn search(db: &SupabaseClient, query: &str) -> Result<Vec<MasterSearchResult>, AppError> {
    // 入力が 2 文字未満なら結果を返さない（DB への無駄なリクエストを避ける）
    if query.len() < 2 {
        return Ok(vec![]); // 空の Vec を返す
    }

    // クエリ文字列を URL エンコードする（日本語・特殊文字対応）
    let encoded_query = urlencoding::encode(query);
    let result = db
        .select(
            "TBL_MASTER",
            &format!(
                // `ilike.*{}*` は「前後にワイルドカードのある大文字小文字無視マッチ」
                // `limit=10` で最大 10 件に制限
                "select=masid,masnm,masct&masnm=ilike.*{}*&limit=10",
                encoded_query
            ),
        )
        .await?;

    let masters: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // JSON 配列を MasterSearchResult のベクタに変換
    Ok(masters
        .iter()
        .filter_map(|m| {
            Some(MasterSearchResult {
                id: Uuid::parse_str(m["masid"].as_str()?).ok()?,
                name: m["masnm"].as_str()?.to_string(),
                category: m["masct"].as_str()?.to_string(),
            })
        })
        .collect())
}

// ============================================================
// find_or_create — 既存マスタを探すか新規作成する
// ============================================================
/// 正規化名でマスタを検索し、なければ新規作成して UUID を返す
///
/// このパターンにより「同じ資格名を複数ユーザーが登録しても
/// マスタレコードは 1 件だけ」という状態を維持する。
///
/// 処理の流れ:
/// 1. normalize_name で正規化
/// 2. 正規化名（masnr）で検索
/// 3. 見つかればその ID を返す（find）
/// 4. 見つからなければ新規 INSERT して ID を返す（create）
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `name`: 元の資格名（前後空白あり・大文字小文字混在も可）
/// - `category`: カテゴリ（手入力の場合は "その他"）
///
/// # 戻り値
/// マスタレコードの UUID
pub async fn find_or_create(
    db: &SupabaseClient,
    name: &str,
    category: &str,
) -> Result<Uuid, AppError> {
    // 名前を正規化（重複チェックの基準値として使う）
    let normalized = normalize_name(name);

    // ---- Find: 正規化名で検索 ----
    // `masnr=eq.{normalized}` で完全一致検索
    let result = db
        .select(
            "TBL_MASTER",
            &format!("select=masid&masnr=eq.{}", urlencoding::encode(&normalized)),
        )
        .await?;
    let existing: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    // 既存レコードが見つかった場合はそのIDを返して終了（OR の Find 側）
    if let Some(master) = existing.first() {
        return Uuid::parse_str(master["masid"].as_str().unwrap_or_default())
            .map_err(|e| AppError::Internal(format!("UUID parse error: {}", e)));
    }

    // ---- Create: 新規レコードを作成 ----
    // `name.trim()` でトリムした表示名を保存（元の大文字・スペースは保持）
    let result = db
        .insert(
            "TBL_MASTER",
            &serde_json::json!({
                "masnm": name.trim(),    // 表示用の名前（元の大文字等を維持）
                "masct": category,       // カテゴリ
                "masnr": normalized,     // 正規化名（重複チェック用）
            }),
        )
        .await?;

    let created: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    let master = created
        .first()
        .ok_or_else(|| AppError::Internal("Master creation returned no data".to_string()))?;

    // 新規作成されたレコードの UUID を返す
    Uuid::parse_str(master["masid"].as_str().unwrap_or_default())
        .map_err(|e| AppError::Internal(format!("UUID parse error: {}", e)))
}
