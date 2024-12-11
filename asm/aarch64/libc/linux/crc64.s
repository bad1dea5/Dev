.global crc64
.global crc64_init

.bss
    .lcomm crc64_lookup_table, 256 * 8

.text
    //
    //  CRC64_ECMA_182
    //
    //  arguments:
    //          x0:     char*           source
    //          x1:     unsigned long   length
    //
    //  return:
    //          x0:     unsigned long   result
    //
    crc64:
        sub     sp, sp, 0x30

        str     x0, [sp]                            // source
        str     x1, [sp, 0x8]                       // length
        str     xzr, [sp, 0x10]                     // result
        str     xzr, [sp, 0x18]                     // i

        b       crc64_loop_check

    crc64_loop:
        ldr     x0, [sp, 0x10]
        lsr     x1, x0, 0x38                        // result >> 56
        ldr     x0, [sp]
        add     x2, x0, 0x1                         // source++
        str     x2, [sp]
        ldrb    w0, [x0]                            // byte
        and     x0, x0, 0xff
        eor     x0, x1, x0                          // result ^ byte
        and     x0, x0, 0xff
        str     x0, [sp, 0x20]                      // byte

        adrp    x0, crc64_lookup_table
        add     x0, x0, :lo12:crc64_lookup_table
        ldr     x1, [sp, 0x20]
        ldr     x1, [x0, x1, lsl 3]                 // crc64_lookup_table[byte]
        ldr     x0, [sp, 0x10]
        lsl     x0, x0, 0x8                         // result << 8
        eor     x0, x1, x0                          // x1 ^ x0
        str     x0, [sp, 0x10]

        ldr     x0, [sp, 0x18]
        add     x0, x0, 0x1                         // i++
        str     x0, [sp, 0x18]

    crc64_loop_check:
        ldr     x1, [sp, 0x18]                      // i
        ldr     x0, [sp, 0x8]                       // length
        cmp     x1, x0                              // i == length
        bcc     crc64_loop

        ldr     x0, [sp, 0x10]

        add     sp, sp, 0x30
        ret
    
    //
    //  initialize lookup table
    //
    crc64_init:
        sub     sp, sp, 0x20

        str     wzr, [sp]                           // i = 0
        b       crc64_init_check

    crc64_init_loop:
        str     xzr, [sp, 0x8]                      // crc = 0

        ldrsw   x0, [sp]
        lsl     x0, x0, 0x38                        // i << 56
        str     x0, [sp, 0x10]                      // mask

        str     wzr, [sp, 0x4]                      // j = 0
        b       crc64_inner_loop_check

    crc64_inner_loop:
        ldr     x1, [sp, 0x8]                       // crc
        ldr     x0, [sp, 0x10]                      // mask
        eor     x0, x1, x0                          // crc ^ mask
        cmp     x0, 0x0
        bge     crc64_inner_shift_crc

        ldr     x0, [sp, 0x8]
        lsl     x1, x0, 0x1                         // crc << 1
        mov     x0, 0x3693
        movk    x0, 0xa9ea, lsl 16
        movk    x0, 0xe1eb, lsl 32
        movk    x0, 0x42f0, lsl 48
        eor     x0, x1, x0                          // crc ^ 0x42F0E1EBA9EA3693
        str     x0, [sp, 0x8]
        b       crc64_inner_shift_mask

    crc64_inner_shift_crc:
        ldr     x0, [sp, 0x8]
        lsl     x0, x0, 0x1                         // crc <<= 1
        str     x0, [sp, 0x8]

    crc64_inner_shift_mask:
        ldr     x0, [sp, 0x10]
        lsl     x0, x0, 0x1                         // mask <<= 1
        str     x0, [sp, 0x10]

        ldr     w0, [sp, 0x4]
        add     w0, w0, 0x1                         // j++
        str     w0, [sp, 0x4]

    crc64_inner_loop_check:
        ldr     w0, [sp, 0x4]
        cmp     w0, 0x7                             // j == 7
        ble     crc64_inner_loop

        adrp    x0, crc64_lookup_table
        add     x0, x0, :lo12:crc64_lookup_table
        ldrsw   x1, [sp]                            // i
        ldr     x2, [sp, 0x8]                       // crc
        str     x2, [x0, x1, lsl 3]                 // crc64_lookup_table[i] = crc

        ldr     w0, [sp]
        add     w0, w0, 0x1                         // i++
        str     w0, [sp]

    crc64_init_check:
        ldr     w0, [sp]
        cmp     w0, 0xff                            // i == 255
        ble     crc64_init_loop

        add     sp, sp, 0x20
        ret
