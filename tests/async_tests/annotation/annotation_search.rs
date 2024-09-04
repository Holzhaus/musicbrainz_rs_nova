use musicbrainz_rs_nova::entity::annotation::*;
use musicbrainz_rs_nova::Search;

#[tokio::test]
#[serial_test::serial]
async fn should_search_annotation() {
    let query = AnnotationSearchQuery::query_builder()
        .text("Warner Classics International")
        .build();

    let result = Annotation::search(query).execute().await.unwrap();

    assert!(result
        .entities
        .iter()
        .any(|annotation| annotation.name == "Warner Classics"));
}
