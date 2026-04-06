// ============================================================
// middleware/rate_limit.rs — レート制限ミドルウェア
// ============================================================
// レート制限とは「一定時間内に受け付けるリクエスト数の上限」を設ける仕組み。
// ブルートフォース攻撃（パスワードを何度も試す攻撃）を防ぐために使用する。
//
// 例: 認証エンドポイントは 5回/分 まで
//
// 【アルゴリズム（スライディングウィンドウ）】
// 各 IP アドレスごとにリクエストのタイムスタンプを記録し、
// 直近の window_secs 秒以内のリクエスト数を数える。
// 上限を超えていたら `false`（拒否）を返す。

// Actix-web のサービス層型（ミドルウェア実装に必要）
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use std::{
    // HashMap: IPアドレス → タイムスタンプリストのマッピング
    collections::HashMap,
    future::{ready, Ready},
    // Arc: 複数スレッドで共有できるポインタ
    // Mutex: 複数スレッドから安全にアクセスするためのロック
    sync::{Arc, Mutex},
    // Instant: 高精度な現在時刻の取得（単調増加クロック）
    time::Instant,
};

/// レート制限器の設定と状態を保持する構造体
///
/// # フィールド説明
/// - `max_requests`: ウィンドウ内の最大リクエスト数
/// - `window_secs`: ウィンドウの長さ（秒）
/// - `store`: IPアドレス → リクエスト時刻のリスト
///   - `Arc<Mutex<...>>` により複数スレッドから安全にアクセスできる
#[derive(Clone)]
pub struct RateLimiter {
    /// 許可する最大リクエスト数
    max_requests: usize,
    /// 時間ウィンドウ（秒）
    window_secs: u64,
    /// リクエスト履歴（IPアドレス → タイムスタンプのリスト）
    /// `Arc` = Atomically Reference Counted（スレッド間での安全な共有）
    /// `Mutex` = 排他ロック（同時に1スレッドだけアクセス可能）
    store: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
}

impl RateLimiter {
    /// 新しいレート制限器を作成する
    ///
    /// # 引数
    /// - `max_requests`: 許可する最大リクエスト数
    /// - `window_secs`: 時間ウィンドウの長さ（秒）
    ///
    /// # 使用例
    /// ```
    /// // 認証エンドポイント用: 5回/分
    /// let auth_limiter = RateLimiter::new(5, 60);
    /// // 一般エンドポイント用: 60回/分
    /// let general_limiter = RateLimiter::new(60, 60);
    /// ```
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            max_requests,
            window_secs,
            // `Arc::new(Mutex::new(...))` でスレッドセーフな空の HashMap を作成
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 指定したキー（IPアドレス等）のリクエストが許可されるか確認する
    ///
    /// 許可されるなら `true` を返し、リクエスト時刻を記録する。
    /// 上限に達している場合は `false` を返す。
    ///
    /// # 引数
    /// - `key`: レート制限の識別子（通常はIPアドレス）
    ///
    /// # 戻り値
    /// - `true`: リクエストを許可する
    /// - `false`: リクエストを拒否する（上限超過）
    pub fn check(&self, key: &str) -> bool {
        // Mutex をロックして HashMap への排他アクセスを取得
        // `unwrap()` はロックの失敗（パニック中のスレッドがロックを持っていた場合）
        let mut store = self.store.lock().unwrap();

        // 現在時刻を記録
        let now = Instant::now();
        // ウィンドウの長さを Duration 型に変換
        let window = std::time::Duration::from_secs(self.window_secs);

        // このキーのタイムスタンプリストを取得（なければ空のVecを作成して挿入）
        // `entry(...).or_insert_with(...)` は "キーがなければデフォルト値を挿入"
        let timestamps = store.entry(key.to_string()).or_insert_with(Vec::new);

        // ウィンドウ外（古すぎる）タイムスタンプを除去する
        // `retain` はクロージャが `true` を返す要素だけを残す
        // `now.duration_since(*t)` は「現在からtまでの経過時間」
        // 経過時間がウィンドウより小さい（= まだウィンドウ内）ものだけ残す
        timestamps.retain(|t| now.duration_since(*t) < window);

        // 残ったタイムスタンプ数（= ウィンドウ内のリクエスト数）が上限に達しているか確認
        if timestamps.len() >= self.max_requests {
            // 上限超過：拒否
            false
        } else {
            // 許可：現在時刻をリストに追加して記録
            timestamps.push(now);
            true
        }
    }
}
