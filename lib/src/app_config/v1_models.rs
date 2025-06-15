use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfigV1_0_0 {
    pub deplio: DeplioConfigV1_0_0,
    pub server: ServerSectionV1_0_0,
    pub app: AppSectionV1_0_0,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeplioConfigV1_0_0 {
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerSectionV1_0_0 {
    pub deplio_server: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSectionV1_0_0 {
    pub name: String,
    pub charts: Vec<ChartV1_0_0>,
    pub sdlc: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChartV1_0_0 {
    pub name: String,
    pub url: String,
    pub namespace: String,
}
