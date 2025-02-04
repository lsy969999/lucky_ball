#[cfg(target_os = "ios")]
use raw_window_handle::RawWindowHandle;

pub struct Kv;
pub trait KvTrait {
    fn set(key: &str, value: &str);
    fn get(key: &str) -> String;
    fn delete(key: &str);
    fn exists(key: &str) -> bool;
}

pub struct AdmobInterstitial;
pub trait AdmobInterstitialTrait {
    fn interstitial_show();
    fn interstitial_load();
    fn interstitial_is_ready();
    fn interstitial_clear();
}

pub struct AdmobBanner;
pub trait AdmobBannerTrait {
    fn banner_launch(#[cfg(target_os = "ios")] rwh: RawWindowHandle);
}

pub struct AppFfi;

pub trait AppFfiTrait {
    fn exit();
    fn init();
    fn get_current_epoch_time() -> i64;
    fn get_locale() -> String;
    fn get_time_offset() -> i32;
    fn splash_hide();
}
