#![deny(missing_docs, unsafe_code)]
//! This crate helps you compile the Harmony protocol into Rust code.
//!
//! Note that you will need to include `prost` in your crate dependencies
//! if you are compiling the protocols. `hrpc` is also needed if you are
//! compiling client and server code.

use std::path::{Path, PathBuf};

use hrpc_build::Builder as HrpcConfig;
use prost_build::Config as ProstConfig;

/// A boxed object that implements [`std::error::Error`].
pub type BoxError = Box<dyn std::error::Error>;
/// Shorthand for `Result<T, BoxError>`.
pub type Result<T> = std::result::Result<T, BoxError>;

/// Contains paths for protocol includes and protos to compile.
#[derive(Debug)]
pub struct Protocol {
    /// Protocol path.
    path: PathBuf,
    /// The includes.
    includes: Vec<PathBuf>,
    /// The protos to compile, relative to `includes`.
    protos: Vec<PathBuf>,
}

impl AsRef<Protocol> for Protocol {
    fn as_ref(&self) -> &Protocol {
        self
    }
}

impl Protocol {
    /// Create a new [`Protocol`].
    pub fn new(path: PathBuf, includes: Vec<PathBuf>, protos: Vec<PathBuf>) -> Self {
        Self {
            path,
            includes,
            protos,
        }
    }

    /// Create a new [`Protocol`], using the protocol path and services
    /// to build. `*_svcs` are expected to be a list of strings in the format
    /// `service_name.version` where `service_name` is the name of the service
    /// and `version` is the version.
    ///
    /// Note that the expected structure is that of the structure in the
    /// repository of Harmony protocol.
    pub fn from_path<S>(
        path: impl AsRef<Path>,
        stable_svcs: &[S],
        staging_svcs: &[S],
    ) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let path = path.as_ref();

        let stable = path.join("stable");
        let staging = path.join("staging");

        let mut protos = Vec::new();

        let mut process_svcs = |svcs: &[S], include: &Path| {
            for service in svcs {
                let mut split = service.as_ref().split('.');
                let svc_name = split.next().ok_or("expected service name")?;
                let svc_version = split.next().ok_or("expected version")?;

                let svc_dir = include.join(svc_name).join(svc_version);
                for res in std::fs::read_dir(svc_dir)? {
                    let entry_name = res?.file_name();
                    let proto_name = entry_name.to_string_lossy();
                    if proto_name.ends_with(".proto") {
                        protos.push(
                            Path::new(svc_name)
                                .join(svc_version)
                                .join(proto_name.as_ref()),
                        );
                    }
                }
            }

            Result::Ok(())
        };

        process_svcs(stable_svcs, &stable)?;
        process_svcs(staging_svcs, &staging)?;

        let includes = vec![stable, staging];

        Ok(Self::new(path.to_path_buf(), includes, protos))
    }

    /// Add an include path.
    pub fn add_include(mut self, include: impl Into<PathBuf>) -> Self {
        self.includes.push(include.into());
        self
    }

    /// Add a proto path.
    pub fn add_proto(mut self, proto: impl Into<PathBuf>) -> Self {
        self.protos.push(proto.into());
        self
    }

    /// Filter include paths.
    pub fn filter_includes(mut self, f: impl Fn(&Path) -> bool) -> Self {
        self.includes = self.includes.into_iter().filter(|s| f(s)).collect();
        self
    }

    /// Filter proto paths.
    pub fn filter_protos(mut self, f: impl Fn(&Path) -> bool) -> Self {
        self.protos = self.protos.into_iter().filter(|s| f(s)).collect();
        self
    }

    /// Get include paths.
    #[inline]
    pub fn includes(&self) -> &[PathBuf] {
        self.includes.as_slice()
    }

    /// Get proto paths.
    #[inline]
    pub fn protos(&self) -> &[PathBuf] {
        self.protos.as_slice()
    }

    /// Get protocol path.
    #[inline]
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

/// Type containing configuration for building a [`Protocol`].
#[derive(Debug)]
pub struct Builder {
    prost_config: ProstConfig,
    hrpc_config: HrpcConfig,
    write_all_permissions: bool,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            prost_config: ProstConfig::new(),
            hrpc_config: hrpc_build::configure(),
            write_all_permissions: false,
        }
    }

    /// Set prost config.
    pub fn prost_config(mut self, new: ProstConfig) -> Self {
        self.prost_config = new;
        self
    }

    /// Set hRPC config.
    pub fn hrpc_config(mut self, new: HrpcConfig) -> Self {
        self.hrpc_config = new;
        self
    }

    /// Set whether to write all permissions.
    pub fn write_permissions(mut self, write: bool) -> Self {
        self.write_all_permissions = write;
        self
    }

    /// Modify prost config.
    pub fn modify_prost_config(mut self, f: impl FnOnce(ProstConfig) -> ProstConfig) -> Self {
        self.prost_config = f(self.prost_config);
        self
    }

    /// Modify hRPC config.
    pub fn modify_hrpc_config(mut self, f: impl FnOnce(HrpcConfig) -> HrpcConfig) -> Self {
        self.hrpc_config = f(self.hrpc_config);
        self
    }

    /// Generate code and write it to `out_dir`.
    pub fn generate(
        mut self,
        protocol: impl AsRef<Protocol>,
        out_dir: impl AsRef<Path>,
    ) -> Result<()> {
        let protocol = protocol.as_ref();
        let out_dir = out_dir.as_ref();

        self.hrpc_config = self.hrpc_config.out_dir(out_dir);

        let compile_result = self.hrpc_config.compile_with_config(
            self.prost_config,
            &protocol.protos,
            &protocol.includes,
        );
        if let Err(err) = compile_result {
            eprintln!("protocol path: {:?}", protocol.path());
            eprintln!("includes paths: {:?}", protocol.includes());
            eprintln!("protos paths: {:?}", protocol.protos());
            let _ = list_dir_all(protocol.path().to_path_buf());
            return Err(Box::new(err));
        }

        #[cfg(feature = "all_permissions")]
        if self.write_all_permissions {
            write_all_permissions(protocol.path(), out_dir)?;
        }

        Ok(())
    }
}

fn list_dir_all(path: PathBuf) -> std::io::Result<()> {
    use std::fs;

    let dir = fs::read_dir(path)?;
    for entry in dir {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            list_dir_all(entry.path())?;
        } else {
            eprintln!("{:?}", entry.path());
        }
    }
    Ok(())
}

/// Writes all permissions collected from the given protocol path to `out_dir`.
/// The file will be named `permissions.rs`.
#[cfg(feature = "all_permissions")]
pub fn write_all_permissions(protocol_path: &Path, out_dir: &Path) -> Result<()> {
    use regex::Regex;
    use walkdir::WalkDir;

    const NEWLINE: &str = if cfg!(target_os = "windows") {
        "\r\n"
    } else {
        "\n"
    };

    let r = Regex::new(&format!(r#"option \(harmonytypes.v1.metadata\).requires_permission_node[ {}]+=[ {}]+"(?P<perm>.+)";"#, NEWLINE, NEWLINE)).unwrap();

    let mut perms = String::new();

    let files = WalkDir::new(protocol_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in files {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(".proto") {
            let text = std::fs::read_to_string(entry.path())?;
            for m in r.captures_iter(&text).flat_map(|c| c.name("perm")) {
                let perm = m.as_str();
                let const_name = perm
                    .replace(|c| ['.', '-'].contains(&c), "_")
                    .to_uppercase();
                let perm_const = format!(
                    "/// `{}` permission\npub const {}: &str = \"{}\";\n",
                    perm, const_name, perm
                );
                if !perms.contains(&perm_const) {
                    perms.push_str(&perm_const);
                }
            }
        }
    }

    std::fs::write(out_dir.join("permissions.rs"), perms)?;

    Ok(())
}
