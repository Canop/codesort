use codesort::*;

/// Check that the annotation content isn't used for sorting itams
#[test]
fn test_exclude_annotation_from_sort_key() {
    static INPUT: &str = r#"
    #[derive(thiserror::Error, Debug)]
    pub enum AeError {
        #[error("{0}")]
        Arn(#[from] ArnError),

        #[error("Wrong endpoint for bucket {bucket_name} (probably wrong region)")]
        BucketWrongEndpoint { bucket_name: String },

        #[error("Bucket {bucket_name} not found (may be in another account)")]
        BucketNotFound { bucket_name: String },

        #[error(
            "Project {0} doesn't exist or isn't valid, create it with the 'init' command"
        )]
        ProjectNotFound(PathBuf),
    }
    "#;
    static OUTPUT: &str = r#"
    #[derive(thiserror::Error, Debug)]
    pub enum AeError {
        #[error("{0}")]
        Arn(#[from] ArnError),

        #[error("Bucket {bucket_name} not found (may be in another account)")]
        BucketNotFound { bucket_name: String },

        #[error("Wrong endpoint for bucket {bucket_name} (probably wrong region)")]
        BucketWrongEndpoint { bucket_name: String },

        #[error(
            "Project {0} doesn't exist or isn't valid, create it with the 'init' command"
        )]
        ProjectNotFound(PathBuf),
    }
    "#;
    let list = LocList::read_str(INPUT, Language::Rust).unwrap();
    let focused = list.focus_around_line_index(4).unwrap();
    let sorted_list = focused.sort();
    sorted_list.print_debug("Sorted list");
    assert_eq!(sorted_list.to_string(), OUTPUT);
}
