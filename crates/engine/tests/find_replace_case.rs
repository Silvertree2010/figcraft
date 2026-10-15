#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use figcraft_engine::{
    Session,
    doc::{NodeId, NodeKind},
};
use serde_json::json;

fn text_of(s: &Session, id: NodeId) -> String {
    let node = s.doc().unwrap().node(id).unwrap();
    let NodeKind::Text(text) = &node.kind else { panic!("expected text") };
    text.characters.clone()
}

#[test]
fn replacement_uses_the_same_matches_as_find() {
    let mut s = Session::new();
    let created = s.execute("text.create", &json!({"text":"A","x":0,"y":0})).unwrap();
    let id = NodeId(created["id"].as_u64().unwrap());
    s.execute("select.none", &json!({})).unwrap();
    let result = s.execute("edit.findReplace", &json!({"find":"a","replace":"B"})).unwrap();
    assert_eq!(result["matches"], json!([id]));
    assert_eq!(s.selection(), vec![id]);
    assert_eq!(text_of(&s, id), "B");
}

#[test]
fn mixed_case_text_replaces_only_the_matched_part() {
    let mut s = Session::new();
    let created = s.execute("text.create", &json!({"text":"Hello World hello","x":0,"y":0})).unwrap();
    let id = NodeId(created["id"].as_u64().unwrap());
    s.execute("edit.findReplace", &json!({"find":"hello","replace":"Bye"})).unwrap();
    assert_eq!(text_of(&s, id), "Bye World Bye");
}

#[test]
fn exact_case_and_absent_controls_keep_working() {
    let mut s = Session::new();
    let created = s.execute("text.create", &json!({"text":"Hello World","x":0,"y":0})).unwrap();
    let id = NodeId(created["id"].as_u64().unwrap());
    let none = s.execute("edit.findReplace", &json!({"find":"absent","replace":"X"})).unwrap();
    assert_eq!(none["matches"], json!([]));
    assert_eq!(text_of(&s, id), "Hello World");
    s.execute("edit.findReplace", &json!({"find":"Hello","replace":"Bye"})).unwrap();
    assert_eq!(text_of(&s, id), "Bye World");
}

#[test]
fn layer_name_replacement_ignores_case_like_find() {
    let mut s = Session::new();
    let created = s.execute("shape.rectangle", &json!({"x":0,"y":0,"width":10,"height":10})).unwrap();
    let id = NodeId(created["id"].as_u64().unwrap());
    s.execute("object.rename", &json!({"id": id, "name": "Icon Button"})).unwrap();
    let result = s.execute("edit.findReplace", &json!({"find":"icon button","replace":"Badge","layerNames":true})).unwrap();
    assert_eq!(result["matches"], json!([id]));
    assert_eq!(s.doc().unwrap().node(id).unwrap().name, "Badge");
}
