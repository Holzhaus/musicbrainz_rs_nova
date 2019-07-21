extern crate musicbrainz_rs;
use musicbrainz_rs::model::label;
use musicbrainz_rs::model::label::Label;
use musicbrainz_rs::QueryAble;

#[test]
fn should_get_label_releases() {
    let ninja_tune = Label::fetch()
        .id("dc940013-b8a8-4362-a465-291026c04b42")
        .include(label::Include::Releases)
        .execute();

    let releases = ninja_tune.unwrap().releases;

    assert!(releases.unwrap().iter().any(|release| release.title == "The Final Corporate Colonization of the Unconscious"));
}

#[test]
fn should_get_label_aliases() {
    let motown = Label::fetch()
        .id("8e479e57-ef44-490c-b75d-cd28df89bf1b")
        .include(label::Include::Aliases)
        .execute();

    let aliases = motown.unwrap().aliases;

    assert!(aliases.unwrap().iter().any(|alias| alias.name == "Motown"));
}
