use iana_media_types::{
    Application, Audio, Font, Image, MediaType, Message, Model, Multipart, Text, Video,
};

use crate::shared_provider::v3::shared_provider_metadata::SharedProviderMetadataV3;

#[derive(Debug, Clone, PartialEq)]
enum IANA {
    Appliction(Application),
    Audio(Audio),
    Font(Font),
    Image(Image),
    MediaType(MediaType),
    Message(Message),
    Model(Model),
    Multipart(Multipart),
    Text(Text),
    Video(Video),
}

#[derive(Debug, Clone)]
/// A source that has been used as input to generate the response.
pub enum LanguageModelV3Source {
    URl {
        /// The ID of the source.
        id: String,
        ///  The URL of the source.
        url: String,
        /// The title of the source.
        title: Option<String>,
        /// Additional provider metadata for the source.
        provider_metadata: Option<SharedProviderMetadataV3>,
    },
    Document {
        /// The ID of the source.
        id: String,
        /// IANA media type of the document (e.g., 'application/pdf').
        media_type: IANA,
        /// The title of the document.
        title: String,
        /// Optional filename of the document.
        file_name: Option<String>,
        /// Additional provider metadata for the source.
        provider_metadata: Option<SharedProviderMetadataV3>,
    },
}

#[cfg(test)]
mod tests_language_model_v3_source {
    use super::*;

    #[test]
    fn test_initialization_url() {
        let id_val = "123";
        let url_val = "http://example.com";
        let title_val = "New URL";
        let provider_metadata = None;

        let new_url = LanguageModelV3Source::URl {
            id: String::from(id_val),
            url: String::from(url_val),
            title: Some(String::from(title_val)),
            provider_metadata: provider_metadata,
        };

        match new_url {
            LanguageModelV3Source::URl {
                id,
                url,
                title,
                provider_metadata,
            } => {
                assert_eq!(id, id_val);
                assert_eq!(url, url_val);
                assert_eq!(title.unwrap_or_default(), title_val);
                assert!(provider_metadata.is_none());
            }
            _ => panic!("Expected URl variant"),
        }
    }

    #[test]
    fn test_initialization_document() {
        let id_var = "123";
        let media_type_var = IANA::Appliction(Application::Pdf);
        let title_var = "New Doc";
        let file_name_var = "new_doc.pdf";
        let provider_metadata_var = None;

        let new_document = LanguageModelV3Source::Document {
            id: String::from(id_var),
            media_type: media_type_var.clone(),
            title: String::from(title_var),
            file_name: Some(String::from(file_name_var)),
            provider_metadata: provider_metadata_var,
        };

        match new_document {
            LanguageModelV3Source::Document {
                id,
                media_type,
                title,
                file_name,
                provider_metadata,
            } => {
                assert_eq!(id, id_var);
                assert_eq!(media_type, media_type_var);
                assert_eq!(title, title_var);
                assert_eq!(file_name.unwrap_or_default(), file_name_var);
                assert!(provider_metadata.is_none());
            }
            _ => panic!("Expected Document varient"),
        }
    }
}
