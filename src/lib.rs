#![feature(allocator)]
#![allocator]

#![no_std]

#![crate_name="alloc_chibios"]
#![crate_type="rlib"]

extern crate chibios;

#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    unsafe { chibios::c::chHeapAlloc(0 as *mut u8, size) }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    unsafe { chibios::c::chHeapFree(ptr) }
}

#[no_mangle]
// pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,
pub extern fn __rust_reallocate(_ptr: *mut u8, _old_size: usize, _size: usize,
                                _align: usize) -> *mut u8 {
	panic!();
    // unsafe {
    //     chibios::realloc(ptr as *mut libc::c_void, size as libc::size_t) as *mut u8
    // }

    /* Borrowed from ChibiOS forum:
     * http://www.chibios.com/forum/viewtopic.php?t=1314
	void *realloc (void *addr, size_t size)
	{
	   union heap_header *hp;
	   size_t prev_size, new_size;

	   void *ptr;

	   if(addr == NULL) {
	      return chHeapAlloc(NULL, size);
	   }

	   /* previous allocated segment is preceded by an heap_header */
	   hp = addr - sizeof(union heap_header);
	   prev_size = hp->h.size; /* size is always multiple of 8 */

	   /* check new size memory alignment */
	   if(size % 8 == 0) {
	      new_size = size;
	   }
	   else {
	      new_size = ((int) (size / 8)) * 8 + 8;
	   }

	   if(prev_size >= new_size) {
	      return addr;
	   }

	   ptr = chHeapAlloc(NULL, size);
	   if(ptr == NULL) {
	      return NULL;
	   }

	   memcpy(ptr, addr, prev_size);

	   chHeapFree(addr);

	   return ptr;
	} /* realloc */
	*/
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size // this api is not supported by ChibiOS
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
