// ============================================================
// services/favorite_service.rs — お気に入りビジネスロジック
// ============================================================
// このファイルは「お気に入り（TBL_FAVORITE）」の操作を担うサービス層。
//
// 【TBL_FAVORITE とは】
// ユーザーが他のユーザーを「お気に入り」として登録するテーブル。
// favui（お気に入り登録したユーザー）→ favti（お気に入りされたユーザー）
// という「誰が誰をフォローしているか」の関係を表す。
//
// 【DB の UNIQUE 制約】
// TBL_FAVORITE には (favui, favti) のペアに UNIQUE 制約がある。
// そのため同じユーザーを二重登録しようとすると 409 Conflict になる。
// このエラーは db.rs の insert メソッドで AppError::Conflict に変換される。
//
// 【自己お気に入り防止】
// add 関数で `user_id == target_user_id` のチェックを行い、
// 自分自身をお気に入りにできないようにする。

use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;

// ============================================================
// add — お気に入り追加
// ============================================================
/// 別のユーザーをお気に入りに追加する
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `user_id`: お気に入り登録を行うユーザーID（自分）
/// - `target_user_id`: お気に入りにしたいユーザーID（相手）
///
/// # 戻り値
/// 成功時は `Ok(())`
///
/// # エラー
/// - `AppError::ValidationError`: 自分自身をお気に入りにしようとした場合
/// - `AppError::Conflict`: 既にお気に入り登録済みの場合（DB 制約）
pub async fn add(
    db: &SupabaseClient,
    user_id: Uuid,
    target_user_id: Uuid,
) -> Result<(), AppError> {
    // 自己お気に入りチェック: UUID 型の == 比較で同一性を確認
    if user_id == target_user_id {
        return Err(AppError::ValidationError(
            "自分自身をお気に入りに登録できません".to_string(),
        ));
    }

    // TBL_FAVORITE にレコードを挿入
    // UNIQUE(favui, favti) 制約で重複は DB レベルで防止される
    db.insert(
        "TBL_FAVORITE",
        &serde_json::json!({
            "favui": user_id.to_string(),        // お気に入り登録したユーザー
            "favti": target_user_id.to_string(), // お気に入りされたユーザー
        }),
    )
    .await?;

    Ok(())
}

// ============================================================
// remove — お気に入り解除
// ============================================================
/// お気に入り登録を解除する
///
/// `favui=eq.{user_id}&favti=eq.{target_user_id}` の両条件で
/// 「自分が相手をお気に入りにしているレコード」を特定して削除する。
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `user_id`: お気に入り解除を行うユーザーID（自分）
/// - `target_user_id`: 解除したいユーザーID（相手）
///
/// # 戻り値
/// 成功時は `Ok(())`（対象レコードが存在しない場合もエラーにしない）
pub async fn remove(
    db: &SupabaseClient,
    user_id: Uuid,
    target_user_id: Uuid,
) -> Result<(), AppError> {
    // 両条件を AND で組み合わせて「自分が登録した相手のみ」を削除
    db.delete(
        "TBL_FAVORITE",
        &format!("favui=eq.{}&favti=eq.{}", user_id, target_user_id),
    )
    .await?;

    Ok(())
}

// ============================================================
// list — お気に入り一覧取得
// ============================================================
/// ログイン中ユーザーのお気に入りユーザー一覧を取得する
///
/// TBL_FAVORITE と TBL_USER を JOIN して
/// お気に入りにした相手のユーザー名も返す。
///
/// # 引数
/// - `db`: Supabase クライアント
/// - `user_id`: 一覧を取得するユーザーID（自分）
///
/// # 戻り値
/// お気に入りレコードのリスト（JSON Value のまま返す）
/// ※ ハンドラ側でさらに整形する
///
/// # PostgREST JOIN 構文の補足
/// `TBL_USER!TBL_FAVORITE_favti_fkey(useid,usenm)` の意味:
/// - `TBL_USER`: JOIN 先のテーブル
/// - `!TBL_FAVORITE_favti_fkey`: 使用する外部キー制約名を明示
///   （TBL_USER への外部キーが複数ある場合に必要）
/// - `(useid,usenm)`: 取得するカラム
pub async fn list(
    db: &SupabaseClient,
    user_id: Uuid,
) -> Result<Vec<serde_json::Value>, AppError> {
    let result = db
        .select(
            "TBL_FAVORITE",
            &format!(
                // `!TBL_FAVORITE_favti_fkey` で favti 側の外部キーを使って JOIN を指定
                "select=favid,favti,favca,TBL_USER!TBL_FAVORITE_favti_fkey(useid,usenm)&favui=eq.{}&order=favca.desc",
                user_id
            ),
        )
        .await?;

    let favorites: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    Ok(favorites)
}
