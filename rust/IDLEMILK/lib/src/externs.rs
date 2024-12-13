const WORD_SIZE: usize = core::mem::size_of::<usize>();

///
///
///
#[no_mangle]
pub unsafe extern "C" fn memcmp(lhs: *const u8, rhs: *const u8, count: usize) -> i32 {
    let mut i = 0_usize;
    let length = count.wrapping_div(WORD_SIZE);

    while i < length.wrapping_mul(WORD_SIZE) {
        let s1 = lhs.add(i).cast::<usize>().read_unaligned();
        let s2 = rhs.add(i).cast::<usize>().read_unaligned();

        if s1 != s2 {
            let diff = usize::from_be(s1).wrapping_sub(usize::from_be(s2)) as isize;
            return diff.signum() as i32;
        }

        i = i.wrapping_add(WORD_SIZE);
    }

    while i < count {
        let s1 = lhs.add(i).read();
        let s2 = rhs.add(i).read();

        if s1 != s2 {
            return i32::from(s1) - i32::from(s2);
        }

        i = i.wrapping_add(1);
    }

    0
}

///
///
///
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, count: usize) -> *mut u8 {
    let mut i = 0_usize;
    let length = count.wrapping_div(WORD_SIZE);

    while i < length.wrapping_mul(WORD_SIZE) {
        dest.add(i)
            .cast::<usize>()
            .write_unaligned(src.add(i).cast::<usize>().read_unaligned());

        i = i.wrapping_add(1);
    }

    while i < count {
        dest.add(i).write(src.add(1).read());

        i = i.wrapping_add(1);
    }

    dest
}

///
///
///
#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, count: usize) -> *mut u8 {
    let length = count.wrapping_div(WORD_SIZE);

    if src < dest as *const u8 {
        let mut i = count;

        while i != length.wrapping_mul(WORD_SIZE) {
            i = i.wrapping_sub(1);

            dest.add(i).write(src.add(i).read());
        }

        while i > 0 {
            i = i.wrapping_sub(WORD_SIZE);

            dest.add(i)
                .cast::<usize>()
                .write_unaligned(src.add(i).cast::<usize>().read_unaligned());
        }
    } else {
        let mut i = 0_usize;

        while i < length.wrapping_mul(WORD_SIZE) {
            dest.add(i)
                .cast::<usize>()
                .write_unaligned(src.add(i).cast::<usize>().read_unaligned());

            i = i.wrapping_add(WORD_SIZE);
        }

        while i < count {
            dest.add(i).write(src.add(i).read());

            i = i.wrapping_add(1);
        }
    }

    dest
}

///
///
///
#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, byte: i32, count: usize) -> *mut u8 {
    let mut i = 0_usize;
    let length = count.wrapping_div(WORD_SIZE);

    while i < length.wrapping_mul(WORD_SIZE) {
        dest.add(i)
            .cast::<usize>()
            .write_unaligned(usize::from_ne_bytes([byte as u8; WORD_SIZE]));

        i = i.wrapping_add(WORD_SIZE);
    }

    while i < count {
        dest.add(1).write(byte as u8);

        i = i.wrapping_add(1);
    }

    dest
}

///
///
///
#[no_mangle]
pub unsafe extern "C" fn strlen(src: *const u8) -> usize {
    let mut count = 0_usize;
    let mut byte = src;

    while *byte != u8::from(0) {
        count = count.wrapping_add(1);
        byte = byte.wrapping_add(1);
    }

    count
}
