/*! The python `frame` type.

*/

use super::super::frame::Frame;
use super::super::pyobject::{
    PyContext, PyFuncArgs, PyObjectKind, PyObjectRef, PyResult, TypeProtocol,
};
use super::super::vm::VirtualMachine;
use super::objtype;

pub fn init(context: &PyContext) {
    let ref frame_type = context.frame_type;
    context.set_attr(&frame_type, "__new__", context.new_rustfunc(frame_new));
    context.set_attr(&frame_type, "__repr__", context.new_rustfunc(frame_repr));
    context.set_attr(&frame_type, "f_locals", context.new_property(frame_flocals));
    context.set_attr(&frame_type, "f_code", context.new_property(frame_fcode));
}

fn frame_new(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(_cls, None)]);
    Err(vm.new_type_error(format!("Cannot directly create frame object")))
}

fn frame_repr(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(_frame, Some(vm.ctx.frame_type()))]);
    let repr = format!("<frame object at .. >");
    Ok(vm.new_str(repr))
}

fn frame_flocals(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(frame, Some(vm.ctx.frame_type()))]);
    let frame = get_value(frame);
    let py_scope = frame.locals.clone();
    let py_scope = py_scope.borrow();

    if let PyObjectKind::Scope { scope } = &py_scope.kind {
        Ok(scope.locals.clone())
    } else {
        panic!("The scope isn't a scope!");
    }
}

fn frame_fcode(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(frame, Some(vm.ctx.frame_type()))]);
    Ok(vm.ctx.new_code_object(get_value(frame).code))
}

pub fn get_value(obj: &PyObjectRef) -> Frame {
    if let PyObjectKind::Frame { frame } = &obj.borrow().kind {
        frame.clone()
    } else {
        panic!("Inner error getting int {:?}", obj);
    }
}
