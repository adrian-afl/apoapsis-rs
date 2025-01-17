// all here return count of elements written, to be added to offset

use glam::{DMat4, DVec3, DVec4};

pub fn write_mat4(ptr: *mut f32, base_offset: isize, data: DMat4) -> isize {
    let mut offset = base_offset;
    unsafe {
        for v in data.as_ref() {
            ptr.offset(offset).write(*v as f32);
            offset += 1;
        }
    }
    16
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
        ptr.offset(offset).write(0.0 as f32);
        offset += 1;
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
        offset += 1;
    }
    4
}

pub fn write_float(ptr: *mut f32, base_offset: isize, data: f64) -> isize {
    unsafe {
        ptr.offset(base_offset).write(data as f32);
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
