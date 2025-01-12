use crate::{ArcPackage, Cache, Package, SearchBy};
use async_trait::async_trait;

/// The main trait for RPC functionality.
///
/// This trait is implemented by [`Handle`](crate::Handle), which is what you should use at run-time.
/// You can also use the mock implementation of this trait in e.g. tests.
///
/// The trait itself is implemented using [`async-trait`].
#[async_trait]
pub trait Raur {
    /// The error type.
    type Err;

    /// Performs an AUR info request.
    ///
    /// You probably want to use [`info`](Self::info) instead.
    ///
    /// This function sends an info request to the AUR RPC. This kind of request takes a list
    /// of package names and returns a list of AUR [`Package`](struct.Package.html)s who's name exactly matches
    /// the input.
    ///
    /// **Note:** If a package being queried does not exist then it will be silently ignored
    /// and not appear in return value.
    ///
    /// **Note:** The return value has no guaranteed order.
    async fn raw_info<S: AsRef<str> + Send + Sync>(
        &self,
        pkg_names: &[S],
    ) -> Result<Vec<Package>, Self::Err>;

    /// Performs an AUR info request, splitting requests as needed.
    ///
    /// This function sends an info request to the AUR RPC. This kind of request takes a list
    /// of package names and returns a list of AUR [`Package`](struct.Package.html)s who's name exactly matches
    /// the input.
    ///
    /// **Note:** If a package being queried does not exist then it will be silently ignored
    /// and not appear in return value.
    ///
    /// **Note:** The return value has no guaranteed order.
    async fn info<S: AsRef<str> + Send + Sync>(
        &self,
        pkg_names: &[S],
    ) -> Result<Vec<Package>, Self::Err> {
        let mut packages = Vec::with_capacity(pkg_names.len());

        for chunk in pkg_names.chunks(500) {
            packages.extend(self.raw_info(chunk).await?);
        }

        Ok(packages)
    }

    /// Perform an info request, storing the results into cache. Requests are not made
    /// for packages already in cache. If all packages are already in cache then no network request
    /// will be made.
    ///
    /// The packages requested will be returned back (even if they were already in cache). Packages
    /// that could not be found will be missing from the return.
    async fn cache_info<S: AsRef<str> + Send + Sync>(
        &self,
        cache: &mut Cache,
        pkgs: &[S],
    ) -> Result<Vec<ArcPackage>, Self::Err> {
        let mut ret = Vec::with_capacity(pkgs.len());
        let mut resolve = Vec::with_capacity(pkgs.len());

        for pkg in pkgs {
            if let Some(pkg) = cache.get(pkg.as_ref()) {
                ret.push(pkg.clone());
            } else {
                resolve.push(pkg.as_ref());
            }
        }

        cache.reserve(resolve.len());

        for chunk in resolve.chunks(100) {
            let res = self.info(chunk).await?;
            for pkg in res.into_iter() {
                let pkg = ArcPackage::from(pkg);
                cache.insert(pkg.clone());
                ret.push(pkg);
            }
        }

        Ok(ret)
    }

    /// Performs an AUR search request.
    ///
    /// This function sends a search request to the AUR RPC. This kind of request takes a
    /// single query and returns a list of matching packages.
    ///
    /// **Note:** Unlike info, search results will never include:
    ///
    /// - Dependency types
    /// - Licences
    /// - Groups
    ///
    /// See [`SearchBy`](enum.SearchBy.html) for how packages are matched.
    async fn search_by<S: AsRef<str> + Send + Sync>(
        &self,
        query: S,
        strategy: SearchBy,
    ) -> Result<Vec<Package>, Self::Err>;

    /// Performs an AUR search request by NameDesc.
    ///
    /// This is the same as [`fn.search_by`](fn.search_by.html) except it always searches by
    /// NameDesc (the default for the AUR).
    async fn search<S: AsRef<str> + Send + Sync>(
        &self,
        query: S,
    ) -> Result<Vec<Package>, Self::Err> {
        self.search_by(query, SearchBy::default()).await
    }

    /// Returns a list of all orphan packages in the AUR.
    async fn orphans(&self) -> Result<Vec<Package>, Self::Err> {
        self.search_by("", SearchBy::Maintainer).await
    }
}
