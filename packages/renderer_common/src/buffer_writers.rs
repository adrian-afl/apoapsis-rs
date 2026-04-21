#![allow(clippy::not_unsafe_ptr_arg_deref)]
// all here return count of elements written, to be added to offset

// TODO this can all take references instead of copies .......

use glam::{DMat4, DVec2, DVec3, DVec4};

pub fn write_mat4(ptr: *mut f32, base_offset: isize, data: DMat4) -> isize {
    let mut offset = base_offset;
    unsafe {
        for v in data.to_cols_array() {
            ptr.offset(offset).write(v as f32);
            offset += 1;
        }
    }
    // offset += write_vec4(ptr, offset, data.x_axis);
    // offset += write_vec4(ptr, offset, data.y_axis);
    // offset += write_vec4(ptr, offset, data.z_axis);
    // offset += write_vec4(ptr, offset, data.w_axis);
    16
}

pub fn write_dmat4(ptr: *mut f64, base_offset: isize, data: DMat4) -> isize {
    let mut offset = base_offset;
    unsafe {
        for v in data.to_cols_array() {
            ptr.offset(offset).write(v as f64);
            offset += 1;
        }
    }
    // offset += write_vec4(ptr, offset, data.x_axis);
    // offset += write_vec4(ptr, offset, data.y_axis);
    // offset += write_vec4(ptr, offset, data.z_axis);
    // offset += write_vec4(ptr, offset, data.w_axis);
    16
}

pub fn write_vec2(ptr: *mut f32, base_offset: isize, data: DVec2) -> isize {
    let mut offset = base_offset;
    unsafe {
        ptr.offset(offset).write(data.x as f32);
        offset += 1;
        ptr.offset(offset).write(data.y as f32);
    }
    2
}

pub fn write_vec3_zero(ptr: *mut f32, base_offset: isize, data: DVec3) -> isize {
    let mut offset = base_offset;
    unsafe {
        ptr.offset(offset).write(data.x as f32);
        offset += 1;
        ptr.offset(offset).write(data.y as f32);
        offset += 1;
        ptr.offset(offset).write(data.z as f32);
        offset += 1;
        ptr.offset(offset).write(0.0_f32);
    }
    4
}

pub fn write_vec3_with_float(
    ptr: *mut f32,
    base_offset: isize,
    data_a: DVec3,
    data_b: f64,
) -> isize {
    let mut offset = base_offset;
    unsafe {
        ptr.offset(offset).write(data_a.x as f32);
        offset += 1;
        ptr.offset(offset).write(data_a.y as f32);
        offset += 1;
        ptr.offset(offset).write(data_a.z as f32);
        offset += 1;
        ptr.offset(offset).write(data_b as f32);
    }
    4
}

pub fn write_vec4(ptr: *mut f32, base_offset: isize, data: DVec4) -> isize {
    let mut offset = base_offset;
    unsafe {
        ptr.offset(offset).write(data.x as f32);
        offset += 1;
        ptr.offset(offset).write(data.y as f32);
        offset += 1;
        ptr.offset(offset).write(data.z as f32);
        offset += 1;
        ptr.offset(offset).write(data.w as f32);
    }
    4
}

pub fn write_float(ptr: *mut f32, base_offset: isize, data: f64) -> isize {
    unsafe {
        ptr.offset(base_offset).write(data as f32);
    }
    1
}

pub fn write_int(ptr: *mut f32, base_offset: isize, data: i32) -> isize {
    let ptr_int = ptr as *mut i32;
    unsafe {
        ptr_int.offset(base_offset).write(data);
    }
    1
}

pub fn write_uint(ptr: *mut f32, base_offset: isize, data: u32) -> isize {
    let ptr_uint = ptr as *mut u32;
    unsafe {
        ptr_uint.offset(base_offset).write(data);
    }
    1
}

pub fn write_bool_as_uint(ptr: *mut f32, base_offset: isize, data: bool) -> isize {
    let ptr_uint = ptr as *mut u32;
    unsafe {
        ptr_uint.offset(base_offset).write(if data { 1 } else { 0 });
    }
    1
}
