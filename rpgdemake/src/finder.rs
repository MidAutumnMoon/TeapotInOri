use std::path::{
    Path,
    PathBuf,
};

use tracing::debug;

use crate::resource::Resource;

use walkdir::WalkDir;


#[ tracing::instrument ]
pub fn find_all( toplevel: &Path )
    -> anyhow::Result< Vec<PathBuf> >
{
    let files = WalkDir::new( toplevel )
        .into_iter()
            .collect::< Result<Vec<_>, _> >()?
        .into_iter()
            .map( |e| e.path().to_owned() )
            // throw away non-files
            .filter_map( |p| p.is_file().then( || p ) )
            .filter( |f| Resource::real_extension( &f ).is_some() )
        .collect()
    ;

    Ok( files )
}
