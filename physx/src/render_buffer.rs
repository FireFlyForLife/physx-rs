use physx_sys::*;
use crate::traits::Releasable;

#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct DebugPoint {
    pub pos: [f32; 3],
    pub color: u32,
}
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct DebugLine {
    pub pos0: [f32; 3],
    pub color0: u32,
    pub pos1: [f32; 3],
    pub color1: u32,
}
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct DebugTriangle {
    pub pos0: [f32; 3],
    pub color0: u32,
    pub pos1: [f32; 3],
    pub color1: u32,
    pub pos2: [f32; 3],
    pub color2: u32,
}

#[test]
fn check_point_sizes() {
    assert_eq!(std::mem::size_of::<PxDebugPoint>(), std::mem::size_of::<DebugPoint>());
}
#[test]
fn check_lines_sizes() {
    assert_eq!(std::mem::size_of::<PxDebugLine>(), std::mem::size_of::<DebugLine>());
}
#[test]
fn check_triangle_sizes() {
    assert_eq!(std::mem::size_of::<PxDebugTriangle>(), std::mem::size_of::<DebugTriangle>());
}

// Mutable RenderBuffer wrapper for PxRenderBuffer
pub struct RenderBufferMut {
    ptr: *mut PxRenderBuffer,
}

impl RenderBufferMut {
    pub fn from_ptr(raw_ptr: *mut PxRenderBuffer) -> RenderBufferMut {
        RenderBufferMut{ ptr: raw_ptr } 
    }
    
    // pub fn from_ptrs(raw_ptr: *mut PxRenderBuffer, allocator_callback: *mut PxAllocatorCallback) -> RenderBuffer {
    //     RenderBuffer{ptr: raw_ptr, allocator_callback }
    // }

    pub fn clear(&mut self) {
        unsafe {
            PxRenderBuffer_clear_mut(self.ptr);
        }
    }

    pub fn append(&mut self, other: &RenderBufferMut) {
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

    pub fn get_number_triangles(&self) -> u32 {
        unsafe { PxRenderBuffer_getNbTriangles(self.ptr) }
    }
    pub fn get_triangles(&self) -> Vec<DebugTriangle> {
        let mut triangles: Vec<DebugTriangle> = Vec::new();
        triangles.resize_with(self.get_number_triangles() as usize, Default::default);

        let triangles_ptr = unsafe {PxRenderBuffer_getTriangles(self.ptr) };
        unsafe {
            let debug_triangles_ptr  = std::mem::transmute(triangles_ptr);
            let vec_triangles_ptr = triangles.as_mut_ptr();
            std::ptr::copy_nonoverlapping::<DebugTriangle>(debug_triangles_ptr, vec_triangles_ptr, triangles.len());
        }

        triangles
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
pub struct RenderBuffer<'a> {
    ptr: *const PxRenderBuffer,
    _lifetime_capture: std::marker::PhantomData<&'a PxRenderBuffer>,
}

impl<'a> RenderBuffer<'a> {
    pub fn from_ptr(raw_ptr: *const PxRenderBuffer) -> RenderBuffer<'a> {
        RenderBuffer{ ptr: raw_ptr, _lifetime_capture: std::marker::PhantomData }
    }
    
    // pub fn from_ptrs(raw_ptr: *mut PxRenderBuffer, allocator_callback: *mut PxAllocatorCallback) -> RenderBuffer {
    //     RenderBuffer{ptr: raw_ptr, allocator_callback }
    // }

    pub fn get_number_points(&self) -> u32 {
        unsafe { PxRenderBuffer_getNbPoints(self.ptr) }
    }
    pub fn get_points(&self) -> &'a [DebugPoint] {
        unsafe { 
            let px_points_ptr = PxRenderBuffer_getPoints(self.ptr);
            let debug_points_ptr: *const DebugPoint = std::mem::transmute(px_points_ptr);
            std::slice::from_raw_parts::<'a, DebugPoint>(debug_points_ptr, self.get_number_points() as usize)
        }
    }

    pub fn get_number_lines(&self) -> u32 {
        unsafe { PxRenderBuffer_getNbLines(self.ptr) }
    }
    pub fn get_lines(&self) -> &'a [DebugLine] {
        unsafe { 
            let px_lines_ptr = PxRenderBuffer_getLines(self.ptr);
            let debug_lines_ptr: *const DebugLine = std::mem::transmute(px_lines_ptr);
            std::slice::from_raw_parts::<'a, DebugLine>(debug_lines_ptr, self.get_number_lines() as usize)
        }
    }

    pub fn get_number_triangles(&self) -> u32 {
        unsafe { PxRenderBuffer_getNbTriangles(self.ptr) }
    }
    pub fn get_triangles(&self) -> &'a [DebugTriangle] {
        unsafe { 
            let px_triangles_ptr = PxRenderBuffer_getTriangles(self.ptr);
            let debug_triangles_ptr: *const DebugTriangle = std::mem::transmute(px_triangles_ptr);
            std::slice::from_raw_parts::<'a, DebugTriangle>(debug_triangles_ptr, self.get_number_triangles() as usize)
        }
    }
}
