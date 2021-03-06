use crate::models::media::*;
use mizer_media::api::TagCreateModel;
use mizer_media::documents::*;
use protobuf::SingularPtrField;
use std::path::Path;

impl From<CreateMediaTag> for TagCreateModel {
    fn from(model: CreateMediaTag) -> Self {
        TagCreateModel { name: model.name }
    }
}

impl From<TagDocument> for MediaTag {
    fn from(tag: TagDocument) -> Self {
        MediaTag {
            name: tag.name,
            id: tag.id.to_string(),
            ..Default::default()
        }
    }
}

impl From<MediaDocument> for MediaFile {
    fn from(media: MediaDocument) -> Self {
        let thumbnail_path = Path::new(&media.name).with_extension("png");
        let thumbnail_path = thumbnail_path.as_os_str().to_str().unwrap();
        let content_url = format!("http://localhost:50050/media/{}", media.name);
        MediaFile {
            id: media.id.to_string(),
            name: media.name,
            tags: media
                .tags
                .into_iter()
                .map(MediaTag::from)
                .collect::<Vec<_>>()
                .into(),
            contentUrl: content_url,
            thumbnailUrl: format!("http://localhost:50050/thumbnails/{}", thumbnail_path),
            ..Default::default()
        }
    }
}

impl From<AttachedTag> for MediaTag {
    fn from(tag: AttachedTag) -> Self {
        MediaTag {
            id: tag.id.to_string(),
            name: tag.name,
            ..Default::default()
        }
    }
}

impl From<MediaTag> for AttachedTag {
    fn from(tag: MediaTag) -> Self {
        AttachedTag {
            id: tag.id.parse().unwrap(),
            name: tag.name,
        }
    }
}

impl From<TagDocument> for MediaTagWithFiles {
    fn from(tag: TagDocument) -> Self {
        MediaTagWithFiles {
            tag: SingularPtrField::some(MediaTag {
                name: tag.name,
                id: tag.id.to_string(),
                ..Default::default()
            }),
            files: tag
                .media
                .into_iter()
                .map(MediaFile::from)
                .collect::<Vec<_>>()
                .into(),
            ..Default::default()
        }
    }
}

impl From<AttachedMediaDocument> for MediaFile {
    fn from(media: AttachedMediaDocument) -> Self {
        let document = match media {
            AttachedMediaDocument::Video(media) => media,
            AttachedMediaDocument::Image(media) => media,
            AttachedMediaDocument::Audio(media) => media,
            AttachedMediaDocument::Vector(media) => media,
        };

        MediaFile {
            id: document.id.to_string(),
            name: document.name,
            contentUrl: format!("http://localhost:50050/media/{}.mp4", document.id),
            thumbnailUrl: format!("http://localhost:50050/thumbnails/{}.png", document.id),
            ..Default::default()
        }
    }
}
