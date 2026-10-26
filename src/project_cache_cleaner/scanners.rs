use crate::project_cache_cleaner::{i_scanner::IScanner, project_type::ProjectType};

pub struct FlutterScanner;

impl IScanner for FlutterScanner {
    fn project_type(&self) -> super::project_type::ProjectType {
        ProjectType::Flutter
    }

    fn cache_folder_names(&self) -> Vec<String> {
        vec![String::from(".dart_tool"), String::from("build")]
    }

    fn is_match(&self, filename: &String) -> bool {
        filename == "pubspec.yaml"
    }

    fn should_queue_sub_folder(&self, dirname: &str) -> bool {
        if dirname == "example" {
            return true;
        }
        false
    }
}
