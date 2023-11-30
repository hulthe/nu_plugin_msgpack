use nu_plugin::LabeledError;
use nu_protocol::{Record, Value};

/// Convert [nu_protocol::Value] to a [rmpv::Value].
pub fn nu_to_rmpv(value: Value) -> Result<rmpv::Value, LabeledError> {
    Ok(match value {
        Value::Bool { val, .. } => val.into(),
        Value::Int { val, .. } => val.into(),
        Value::Float { val, .. } => val.into(),
        Value::String { val, .. } => val.into(),
        Value::Binary { val, .. } => val.into(),
        Value::Nothing { .. } => rmpv::Value::Nil,
        Value::List { vals, .. } => {
            let vals: Result<_, _> = vals.into_iter().map(nu_to_rmpv).collect();
            rmpv::Value::Array(vals?)
        }

        // Convert record to map.
        Value::Record { val, .. } => {
            let Record { cols, vals } = val;
            let pairs: Result<_, LabeledError> = cols
                .into_iter()
                .zip(vals)
                .map(|(k, v)| Ok((k.into(), nu_to_rmpv(v)?)))
                .collect();

            rmpv::Value::Map(pairs?)
        }

        // Convert filesize to number of bytes, like `to json` does.
        Value::Filesize { val, .. } => val.into(),

        // Convert duration to nanoseconds, like `to json` does.
        Value::Duration { val, .. } => val.into(),

        // Convert date to msgpack extension type -1
        // defined in https://github.com/msgpack/msgpack/blob/master/spec.md
        Value::Date { val, .. } => {
            let nanos: u32 = val.timestamp_subsec_nanos();
            let seconds: i64 = val.timestamp();

            let mut data: Vec<u8>;

            // use the smallest datetime representation possible
            // TODO: implement 8 byte representation
            if let (Ok(seconds), 0) = (u32::try_from(seconds), nanos) {
                data = seconds.to_be_bytes().to_vec();
            } else {
                data = Vec::with_capacity(12);
                data.extend_from_slice(&nanos.to_be_bytes());
                data.extend_from_slice(&seconds.to_be_bytes());
            }
            rmpv::Value::Ext(-1, data)
        }
        Value::Range { val, .. } => {
            let vals: Result<_, _> = val.into_range_iter(None)?.map(nu_to_rmpv).collect();
            rmpv::Value::Array(vals?)
        }

        Value::CustomValue { val, internal_span } => {
            let val = val.to_base_value(internal_span)?;
            nu_to_rmpv(val)?
        }

        Value::LazyRecord { val, .. } => nu_to_rmpv(val.collect()?)?,

        // Convert anything we can't represent in msgpck to nil
        // Pretty sure this is how `to json` does it.
        _ => rmpv::Value::Nil,
        //Value::Block { val, .. } => todo!(),
        //Value::Closure { val, .. } => todo!(),
        //Value::Error { error, .. } => todo!(),
        //Value::CellPath { val, .. } => todo!(),
        //Value::MatchPattern { val, .. } => todo!(),
    })
}
