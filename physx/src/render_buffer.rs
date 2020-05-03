use physx_sys::*;
use crate::traits::Releasable;
use glam::Vec3;

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct DebugPoint {
    pub pos: Vec3,
    pub color: u32,
}
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct DebugLine {
    pub pos0: Vec3,
    pub color0: u32,
    pub pos1: Vec3,
    pub color1: u32,
}

#[test]
fn check_point_line_sizes() {
    assert_eq!(std::mem::size_of::<PxDebugPoint>(), std::mem::size_of::<DebugPoint>());
    assert_eq!(std::mem::size_of::<PxDebugLine>(), std::mem::size_of::<DebugLine>());
}

// Mutable RenderBuffer wrapper for PxRenderBuffer
pub struct RenderBufferMut {
    ptr: *mut PxRenderBuffer,
}

impl RenderBufferMut {
    pub fn from_ptr(raw_ptr: *mut PxRenderBuffer) -> RenderBuffer {
        RenderBuffer{ ptr: raw_ptr } 
    }
    
    // pub fn from_ptrs(raw_ptr: *mut PxRenderBuffer, allocator_callback: *mut PxAllocatorCallback) -> RenderBuffer {
    //     RenderBuffer{ptr: raw_ptr, allocator_callback }
    // }

    pub fn clear(&mut self) {
        unsafe {
            PxRenderBuffer_clear_mut(self.ptr);
        }
    }

    pub fn append(&mut self, other: &RenderBuffer) {
        unsafe {
            PxRenderBuffer_append_mut(self.ptr, other.ptr);
        }
    }

    pub fn get_number_points(&self) -> u32 {
        unsafe {
            PxRenderBuffer_getNbPoints(self.ptr)
        }
    }
    pub fn get_points(&self) -> Vec<DebugPoint> {
        let mut points: Vec<DebugPoint> = Vec::new();
        points.resize_with(self.get_number_points() as usize, Default::default);

        let points_ptr = unsafe { PxRenderBuffer_getPoints(self.ptr) };
        unsafe{
            let debug_points_ptr = std::mem::transmute(points_ptr);
            let vec_points_ptr = points.as_mut_ptr();
            std::ptr::copy_nonoverlapping::<DebugPoint>(debug_points_ptr, vec_points_ptr, points.len());
        }

        points
    }

    pub fn get_number_lines(&self) -> u32 {
        unsafe { PxRenderBuffer_getNbLines(self.ptr) }
    }
    pub fn get_lines(&self) -> Vec<DebugLine> {
        let mut lines: Vec<DebugLine> = Vec::new();
        lines.resize_with(self.get_number_lines() as usize, Default::default);

        let lines_ptr = unsafe {PxRenderBuffer_getLines(self.ptr) };
        unsafe{
            let debug_lines_ptr  = std::mem::transmute(lines_ptr);
            let vec_lines_ptr = lines.as_mut_ptr();
            std::ptr::copy_nonoverlapping::<DebugLine>(debug_lines_ptr, vec_lines_ptr, lines.len());
        }

        lines
    }
}

impl Releasable for RenderBufferMut {
    fn release(&mut self) {
        unsafe{
            PxRenderBuffer_delete(self.ptr);
            self.ptr = std::mem::transmute( std::ptr::null::<PxRenderBuffer>() );
        }
    }
}

impl Drop for RenderBufferMut {
    fn drop(&mut self) {
        unsafe {
            PxRenderBuffer_delete(self.ptr);
            self.ptr = std::mem::transmute( std::ptr::null::<PxRenderBuffer>() );
        }
    }
}

// Immutable RenderBuffer wrapper for PxRenderBuffer
pub struct RenderBuffer {
    ptr: *const PxRenderBuffer,
}

impl RenderBuffer {
    pub fn from_ptr(raw_ptr: *const PxRenderBuffer) -> RenderBuffer {
        RenderBuffer{ ptr: raw_ptr }
    }
    
    // pub fn from_ptrs(raw_ptr: *mut PxRenderBuffer, allocator_callback: *mut PxAllocatorCallback) -> RenderBuffer {
    //     RenderBuffer{ptr: raw_ptr, allocator_callback }
    // }

    pub fn get_number_points(&self) -> u32 {
        unsafe { PxRenderBuffer_getNbPoints(self.ptr) }
    }
    pub fn get_points(&self) -> Vec<DebugPoint> {
        let mut points: Vec<DebugPoint> = Vec::new();
        points.resize_with(self.get_number_points() as usize, Default::default);

        let points_ptr = unsafe { PxRenderBuffer_getPoints(self.ptr) };
        unsafe{
            let debug_points_ptr = std::mem::transmute(points_ptr);
            let vec_points_ptr = points.as_mut_ptr();
            std::ptr::copy_nonoverlapping::<DebugPoint>(debug_points_ptr, vec_points_ptr, points.len());
        }

        points
    }

    pub fn get_number_lines(&self) -> u32 {
        unsafe { PxRenderBuffer_getNbLines(self.ptr) }
    }
    //TODO: Make non-allocating overload
    pub fn get_lines(&self) -> Vec<DebugLine> {
        let mut lines: Vec<DebugLine> = Vec::new();
        lines.resize_with(self.get_number_lines() as usize, Default::default);

        let lines_ptr = unsafe { PxRenderBuffer_getLines(self.ptr) };
        unsafe{
            let debug_lines_ptr  = std::mem::transmute(lines_ptr);
            let vec_lines_ptr = lines.as_mut_ptr();
            std::ptr::copy_nonoverlapping::<DebugLine>(debug_lines_ptr, vec_lines_ptr, lines.len());
        }

        lines
    }
}
