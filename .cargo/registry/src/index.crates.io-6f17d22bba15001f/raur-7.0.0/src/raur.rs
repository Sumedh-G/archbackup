use serde_derive::{Deserialize, Serialize};
use std::fmt;

/// The default URL used for the AUR.
pub static AUR_RPC_URL: &str = "https://aur.archlinux.org/rpc/";

/// The package info that a query will return.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Package {
    /// The ID of the package. Mostly used internally,
    /// to not have to reference a package by name.
    #[serde(rename = "ID")]
    pub id: u32,
    /// The name of the package.
    pub name: String,
    /// The ID associated with the git location of the package.
    #[serde(rename = "PackageBaseID")]
    pub package_base_id: u32,
    /// This is the git URL, usually matches the name of the package.
    pub package_base: String,
    /// The package version.
    pub version: String,
    /// The package description.
    pub description: Option<String>,
    /// The URL belonging to the upstream software.
    #[serde(default, rename = "URL")]
    pub url: Option<String>,
    /// The number of votes for the package.
    pub num_votes: u32,
    /// How often the package is downloaded. Decays over time.
    pub popularity: f64,
    /// This is the date that it was marked out-of-date.
    pub out_of_date: Option<i64>,
    /// The name of the package maintainer, if there is one.
    pub maintainer: Option<String>,
    /// The name of the package submitter.
    pub submitter: Option<String>,
    /// The time that the package was first submitted.
    pub first_submitted: i64,
    /// When the package was last updated.
    pub last_modified: i64,
    /// Path to download this package as a tarball.
    /// This must be appended to the domain name, as it does not include it.
    #[serde(default, rename = "URLPath")]
    pub url_path: String,
    /// The names of the groups this package belongs to.
    #[serde(default)]
    pub groups: Vec<String>,
    /// The dependencies of the package itself.
    #[serde(default)]
    pub depends: Vec<String>,
    /// The dependencies that are only relevant
    /// while the package is being built.
    #[serde(default)]
    pub make_depends: Vec<String>,
    /// Optional dependencies needed to enable
    /// certain features.
    #[serde(default)]
    pub opt_depends: Vec<String>,
    /// Dependencies needed for the 'check' stage.
    #[serde(default)]
    pub check_depends: Vec<String>,
    /// The list of packages that this package conflicts with.
    #[serde(default)]
    pub conflicts: Vec<String>,
    /// The list of packages that this package is capable of replacing.
    #[serde(default)]
    pub replaces: Vec<String>,
    /// The namespace this package provides. For example, a *-git
    /// versions of packages provide the same package as non-git versions.
    #[serde(default)]
    pub provides: Vec<String>,
    /// The licenses the package is signed by.
    #[serde(default)]
    pub license: Vec<String>,
    /// Keywords relevant to the package for searching on the AUR.
    #[serde(default)]
    pub keywords: Vec<String>,
    /// The list of Co-Maintainers for a package.
    #[serde(default, rename = "CoMaintainers")]
    pub co_maintainers: Vec<String>,
}

/// What field to search by.
///
/// Name and NameDesc will match if your query is a substring of what you are searching
/// by. The others will only match on exact matches.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Default)]
pub enum SearchBy {
    /// search by package name only
    Name,
    /// search by package name and description (default)
    #[default]
    NameDesc,
    /// search by package maintainer
    Maintainer,
    /// search by package submitter
    Submitter,
    /// search for packages that depend on the query
    Depends,
    /// search for packages that makedepend on the query
    MakeDepends,
    /// search for packages that optdepend on the query
    OptDepends,
    /// search for packages that checkdepend on the query
    CheckDepends,
    /// search for packages providing a given package
    Provides,
    /// search for packages conflicting with a given package
    Conflicts,
    /// search for packages replacing a given package
    Replaces,
    /// search for packages included in a group
    Groups,
    /// search for packages containing a keyword
    Keywords,
    /// search for packages co-maintained by a given maintainer
    CoMaintainers,
}

impl fmt::Display for SearchBy {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            SearchBy::Name => "name",
            SearchBy::NameDesc => "name-desc",
            SearchBy::Maintainer => "maintainer",
            SearchBy::Submitter => "submitter",
            SearchBy::Depends => "depends",
            SearchBy::MakeDepends => "makedepends",
            SearchBy::OptDepends => "optdepends",
            SearchBy::CheckDepends => "checkdepends",
            SearchBy::Provides => "provides",
            SearchBy::Conflicts => "conflicts",
            SearchBy::Replaces => "replaces",
            SearchBy::Groups => "groups",
            SearchBy::Keywords => "keywords",
            SearchBy::CoMaintainers => "comaintainers",
        })
    }
}
