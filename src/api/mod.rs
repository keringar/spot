mod api_models;
mod cached_client;
mod client;
mod connect_player;

pub mod cache;

pub use cached_client::{CachedSpotifyClient, SpotifyApiClient, SpotifyResult};
pub use client::SpotifyApiError;
pub use connect_player::SpotifyConnectPlayer;

pub async fn clear_user_cache() -> Option<()> {
    cache::CacheManager::new(&[])?
        .clear_cache_pattern("spot/net", &*cached_client::USER_CACHE)
        .await
        .ok()
}
