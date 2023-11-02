pub mod navbar_feature;
pub mod post_listing_feature;
pub mod tags_feature;
use self::navbar_feature::NavbarConfig;
use self::post_listing_feature::PostlistingConfig;
pub use navbar_feature::NavbarFeature;
pub use post_listing_feature::PostListingFeature;
use serde::Deserialize;
pub use tags_feature::TagsFeature;

#[derive(Debug, Deserialize, Clone)]
pub enum FeatureConfig {
    Navbar(NavbarConfig),
    Tags,
    Postlisting(PostlistingConfig),
}
