pub mod navbar_feature;
pub mod post_listing_feature;
use self::{navbar_feature::NavbarConfig, post_listing_feature::PostlistingConfig};
pub use navbar_feature::NavbarFeature;
pub use post_listing_feature::PostListingFeature;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum FeatureConfig {
    Navbar(NavbarConfig),
    Tags,
    Postlisting(PostlistingConfig),
}
