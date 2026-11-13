use adapter::t2t::copenai::request::state::{CompatibleOpenAIDecoderState as State, ObjectPhase};

#[test]
fn advances_partial_key_and_model_value_one_byte_at_a_time() {
    let mut state = State::new();
    for byte in b"{\"mode" {
        state = state.next(*byte).unwrap();
    }
    assert!(matches!(&state, State::Key { text, .. } if text == "mode"));
    assert_eq!(state.position(), &State::Request);

    for byte in b"l\":\"provider/model" {
        state = state.next(*byte).unwrap();
    }
    assert!(matches!(state, State::String { .. }));
    assert_eq!(state.position(), &State::Model);

    state = state.next(b'"').unwrap();
    assert!(matches!(
        state,
        State::Object {
            phase: ObjectPhase::CommaOrEnd,
            ..
        }
    ));
    assert_eq!(state.position(), &State::Request);
    state = state.next(b'}').unwrap();
    state.finish().unwrap();
}

#[test]
fn retains_nested_positions_and_string_progress_across_chunks() {
    let body = r#"{
        "model":"provider/model",
        "messages":[{"role":"user","content":"Paris é \" \\ \uD83D\uDE00"}],
        "tools":[{"type":"function","function":{"name":"weather","parameters":{
            "properties":{"city":{"type":"string"}},"required":["city"],
            "additionalProperties":false,"example":[null,-1.2e+3,{"x":true}]
        }}}],
        "tool_choice":{"type":"function","function":{"name":"weather"}},
        "response_format":{"type":"json_schema","json_schema":{
            "name":"answer","strict":true,"schema":{"type":"object"}
        }},
        "provider":{"require_parameters":true},"top_p":0.9,"stream":false
    }"#;
    for size in [1, 2, 7, 64] {
        let mut state = State::new();
        let mut saw_tool = false;
        let mut saw_schema = false;
        for chunk in body.as_bytes().chunks(size) {
            for byte in chunk {
                state = state.next(*byte).unwrap();
                saw_tool |= state.position() == &State::Tool;
                saw_schema |= state.position() == &State::ResponseFormatJsonSchemaSchema;
            }
        }
        state.finish().unwrap();
        assert!(saw_tool && saw_schema);
    }
}
