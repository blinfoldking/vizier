use rustpython_vm::{PyObjectRef, VirtualMachine};
use serde_json::Value;

pub fn json_to_py(value: &serde_json::Value, vm: &VirtualMachine) -> PyObjectRef {
    match value {
        Value::Null => vm.ctx.none(),
        Value::Bool(b) => vm.ctx.new_bool(*b).into(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                vm.ctx.new_int(i).into()
            } else if let Some(f) = n.as_f64() {
                vm.ctx.new_float(f).into()
            } else {
                unimplemented!("Unsupported number variant")
            }
        }
        Value::String(s) => vm.ctx.new_str(s.as_str()).into(),
        Value::Array(arr) => {
            let items: Vec<PyObjectRef> = arr.iter().map(|v| json_to_py(v, vm)).collect();
            vm.ctx.new_list(items).into()
        }
        Value::Object(map) => {
            let dict = vm.ctx.new_dict();
            for (k, v) in map {
                let py_val = json_to_py(v, vm);
                dict.set_item(&k.clone(), py_val, vm)
                    .expect("Failed to set dict item");
            }
            dict.into()
        }
    }
}
