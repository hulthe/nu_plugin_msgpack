mod from;
mod into;

use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, PluginSignature, Span, Value};
use rmpv::decode::read_value_ref;

fn main() {
    serve_plugin(&mut FromMsgpack, MsgPackSerializer {});
}

pub struct FromMsgpack;

const FROM_MSGPACK: &str = "from msgpack";
const TO_MSGPACK: &str = "to msgpack";

impl Plugin for FromMsgpack {
    fn signature(&self) -> Vec<nu_protocol::PluginSignature> {
        vec![
            PluginSignature::build(FROM_MSGPACK)
                .usage("Convert from msgpack to structured data.")
                .category(Category::Formats),
            PluginSignature::build(TO_MSGPACK)
                .usage("Converts data into msgpack.")
                .category(Category::Formats),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        _config: &Option<Value>,
        _call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            FROM_MSGPACK => {
                let mut bin = input.as_binary()?;

                let v = match read_value_ref(&mut bin) {
                    Err(e) => {
                        return Err(LabeledError {
                            label: "Invalid msgpack".into(),
                            msg: e.to_string(),
                            span: None,
                        })
                    }
                    Ok(v) => v,
                };

                from::rmpv_to_nu(v)
            }
            TO_MSGPACK => {
                let msgpack_value = into::nu_to_rmpv(input.clone())?;
                let mut encoded = vec![];
                rmpv::encode::write_value(&mut encoded, &msgpack_value)
                    .expect("encoding to vec can't fail, right?");
                Ok(Value::binary(encoded, Span::unknown()))
            }
            _ => Err(LabeledError {
                label: "Unknown command".into(),
                msg: format!("{name:?} is not a command supported by nu_plugin_msgpack"),
                span: None,
            }),
        }
    }
}
