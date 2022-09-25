//! <b style="font-variant:small-caps">badges.csv</b>

use crate::crates::CrateId;
use crate::error::{err, Result};
use crate::load::FromRecord;
use csv::StringRecord;
use serde_derive::Deserialize;
use std::collections::BTreeMap as Map;

/// One row of **badges.csv**.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Row {
    pub crate_id: CrateId,
    pub badge_type: BadgeType,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum BadgeType {
    #[non_exhaustive]
    Appveyor {
        repository: String,
        project_name: Option<String>,
        branch: Option<String>,
        service: Option<String>,
        id: Option<String>,
    },

    #[non_exhaustive]
    AzureDevops {
        project: String,
        pipeline: String,
        build: Option<String>,
    },

    #[non_exhaustive]
    BitbucketPipelines { repository: String, branch: String },

    #[non_exhaustive]
    CircleCi {
        repository: String,
        branch: Option<String>,
    },

    #[non_exhaustive]
    CirrusCi {
        repository: String,
        branch: Option<String>,
    },

    #[non_exhaustive]
    Codecov {
        repository: String,
        branch: Option<String>,
        service: Option<String>,
    },

    #[non_exhaustive]
    Coveralls {
        repository: String,
        branch: Option<String>,
        service: Option<String>,
    },

    #[non_exhaustive]
    Gitlab {
        repository: String,
        branch: Option<String>,
        tag: Option<String>,
    },

    #[non_exhaustive]
    IsItMaintainedIssueResolution {
        repository: String,
        service: Option<String>,
    },

    #[non_exhaustive]
    IsItMaintainedOpenIssues {
        repository: String,
        service: Option<String>,
    },

    #[non_exhaustive]
    Maintenance { status: MaintenanceStatus },

    #[non_exhaustive]
    TravisCi {
        repository: String,
        branch: Option<String>,
        service: Option<String>,
        master: Option<String>,
        tld: Option<String>,
    },

    Other {
        badge_type: String,
        attributes: Map<String, String>,
    },
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum MaintenanceStatus {
    ActivelyDeveloped,
    AsIs,
    Deprecated,
    Experimental,
    LookingForMaintainer,
    None,
    PassivelyMaintained,
}

impl FromRecord for Row {
    fn from_record(record: &StringRecord, headers: &StringRecord) -> Result<Self> {
        de(record, headers)
    }
}

fn de(record: &StringRecord, headers: &StringRecord) -> Result<Row> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Record<'a> {
        attributes: &'a str,
        badge_type: &'a str,
        crate_id: CrateId,
    }

    let record: Record = record.deserialize(Some(headers)).map_err(err)?;

    let badge_type = match record.badge_type {
        "appveyor" => {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Attributes {
                repository: String,
                #[serde(alias = "project-name")]
                project_name: Option<String>,
                branch: Option<String>,
                service: Option<String>,
                id: Option<String>,
            }
            serde_json::from_str(record.attributes)
                .ok()
                .map(|attributes: Attributes| BadgeType::Appveyor {
                    repository: attributes.repository,
                    project_name: attributes.project_name,
                    branch: attributes.branch,
                    service: attributes.service,
                    id: attributes.id,
                })
        }

        "azure-devops" => {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Attributes {
                project: String,
                pipeline: String,
                build: Option<String>,
            }
            serde_json::from_str(record.attributes)
                .ok()
                .map(|attributes: Attributes| BadgeType::AzureDevops {
                    project: attributes.project,
                    pipeline: attributes.pipeline,
                    build: attributes.build,
                })
        }

        "bitbucket-pipelines" => {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Attributes {
                repository: String,
                branch: String,
            }
            serde_json::from_str(record.attributes)
                .ok()
                .map(|attributes: Attributes| BadgeType::BitbucketPipelines {
                    repository: attributes.repository,
                    branch: attributes.branch,
                })
        }

        "circle-ci" => {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Attributes {
                repository: String,
                branch: Option<String>,
            }
            serde_json::from_str(record.attributes)
                .ok()
                .map(|attributes: Attributes| BadgeType::CircleCi {
                    repository: attributes.repository,
                    branch: attributes.branch,
                })
        }

        "cirrus-ci" => {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Attributes {
                repository: String,
                branch: Option<String>,
            }
            serde_json::from_str(record.attributes)
                .ok()
                .map(|attributes: Attributes| BadgeType::CirrusCi {
                    repository: attributes.repository,
                    branch: attributes.branch,
                })
        }

        "codecov" => {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Attributes {
                repository: String,
                branch: Option<String>,
                service: Option<String>,
            }
            serde_json::from_str(record.attributes)
                .ok()
                .map(|attributes: Attributes| BadgeType::Codecov {
                    repository: attributes.repository,
                    branch: attributes.branch,
                    service: attributes.service,
                })
        }

        "coveralls" => {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Attributes {
                repository: String,
                branch: Option<String>,
                service: Option<String>,
            }
            serde_json::from_str(record.attributes)
                .ok()
                .map(|attributes: Attributes| BadgeType::Coveralls {
                    repository: attributes.repository,
                    branch: attributes.branch,
                    service: attributes.service,
                })
        }

        "gitlab" => {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Attributes {
                repository: String,
                branch: Option<String>,
                tag: Option<String>,
            }
            serde_json::from_str(record.attributes)
                .ok()
                .map(|attributes: Attributes| BadgeType::Gitlab {
                    repository: attributes.repository,
                    branch: attributes.branch,
                    tag: attributes.tag,
                })
        }

        "is-it-maintained-issue-resolution" => {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Attributes {
                repository: String,
                service: Option<String>,
            }
            serde_json::from_str(record.attributes)
                .ok()
                .map(
                    |attributes: Attributes| BadgeType::IsItMaintainedIssueResolution {
                        repository: attributes.repository,
                        service: attributes.service,
                    },
                )
        }

        "is-it-maintained-open-issues" => {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Attributes {
                repository: String,
                service: Option<String>,
            }
            serde_json::from_str(record.attributes)
                .ok()
                .map(
                    |attributes: Attributes| BadgeType::IsItMaintainedOpenIssues {
                        repository: attributes.repository,
                        service: attributes.service,
                    },
                )
        }

        "maintenance" => {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Attributes {
                status: MaintenanceStatus,
            }
            serde_json::from_str(record.attributes)
                .ok()
                .map(|attributes: Attributes| BadgeType::Maintenance {
                    status: attributes.status,
                })
        }

        "travis-ci" => {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Attributes {
                repository: String,
                branch: Option<String>,
                service: Option<String>,
                master: Option<String>,
                tld: Option<String>,
            }
            serde_json::from_str(record.attributes)
                .ok()
                .map(|attributes: Attributes| BadgeType::TravisCi {
                    repository: attributes.repository,
                    branch: attributes.branch,
                    service: attributes.service,
                    master: attributes.master,
                    tld: attributes.tld,
                })
        }

        _other => None,
    };

    let badge_type = if let Some(badge_type) = badge_type {
        badge_type
    } else {
        BadgeType::Other {
            badge_type: record.badge_type.to_owned(),
            attributes: serde_json::from_str(record.attributes).map_err(err)?,
        }
    };

    Ok(Row {
        badge_type,
        crate_id: record.crate_id,
    })
}
