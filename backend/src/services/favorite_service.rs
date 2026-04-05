use uuid::Uuid;

use crate::db::SupabaseClient;
use crate::errors::AppError;

pub async fn add(
    db: &SupabaseClient,
    user_id: Uuid,
    target_user_id: Uuid,
) -> Result<(), AppError> {
    if user_id == target_user_id {
        return Err(AppError::ValidationError(
            "自分自身をお気に入りに登録できません".to_string(),
        ));
    }

    db.insert(
        "TBL_FAVORITE",
        &serde_json::json!({
            "favui": user_id.to_string(),
            "favti": target_user_id.to_string(),
        }),
    )
    .await?;

    Ok(())
}

pub async fn remove(
    db: &SupabaseClient,
    user_id: Uuid,
    target_user_id: Uuid,
) -> Result<(), AppError> {
    db.delete(
        "TBL_FAVORITE",
        &format!("favui=eq.{}&favti=eq.{}", user_id, target_user_id),
    )
    .await?;

    Ok(())
}

pub async fn list(
    db: &SupabaseClient,
    user_id: Uuid,
) -> Result<Vec<serde_json::Value>, AppError> {
    let result = db
        .select(
            "TBL_FAVORITE",
            &format!(
                "select=favid,favti,favca,TBL_USER!TBL_FAVORITE_favti_fkey(useid,usenm)&favui=eq.{}&order=favca.desc",
                user_id
            ),
        )
        .await?;

    let favorites: Vec<serde_json::Value> = serde_json::from_value(result)
        .map_err(|e| AppError::Internal(format!("Parse error: {}", e)))?;

    Ok(favorites)
}
