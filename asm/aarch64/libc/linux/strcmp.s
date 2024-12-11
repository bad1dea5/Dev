.global strcmp

.text
	//
	//	strcmp
	//
	//	arguments
	//
	//		x0	const char*		lhs
	//		x1	const char*		rhs
	//
	//	return value
	//
	//		x0:		int			return value
	//
	//		-1 if lhs appears before rhs in lexicographical order.
	//		0  if lhs and rhs compare equal.
	//		1  if lhs appears after rhs in lexicographical order. 
	//
	strcmp:
		sub		sp, sp, 0x10

		str		x0, [sp]
		str		x1, [sp, 0x8]

		ldr		x0, [sp]
		cmp		x0, 0x0
		bne		is_str1_null

		ldr		x0, [sp, 0x8]
		cmp		x0, 0x0
		bne		is_str1_null

		mov		w0, 0x0
		b		exit

	is_str1_null:
		ldr		x0, [sp]
		cmp		x0, 0x0
		bne		is_str2_null

		mov		w0, -1
		b		exit

	is_str2_null:
		ldr		x0, [sp, 0x8]
		cmp		x0, 0x0
		bne		compare_characters

		mov		w0, 0x1
		b		exit

	increment_pointers:
		ldr		x0, [sp]
		add		x0, x0, 0x1
		str		x0, [sp]

		ldr		x0, [sp, 0x8]
		add		x0, x0, 0x1
		str		x0, [sp, 0x8]

	compare_characters:
		ldr		x0, [sp]
		ldrb	w0, [x0]
		cmp		w0, 0x0
		beq		compare_string

		ldr     x0, [sp, 0x8]
        ldrb    w0, [x0]
        cmp     w0, 0x0
        beq     compare_string

		ldr     x0, [sp]
        ldrb    w1, [x0]
		ldr     x0, [sp, 0x8]
        ldrb    w0, [x0]
        cmp     w1, w0
        beq     increment_pointers

	compare_string:
		ldr     x0, [sp]
        ldrb    w0, [x0]
        cmp     w0, 0x0
        bne		compare_remaining

		ldr     x0, [sp, 0x8]
        ldrb    w0, [x0]
        cmp     w0, 0x0
        bne     compare_remaining

		mov		w0, 0x0
		b		exit

	compare_remaining:
		ldr     x0, [sp]
        ldrb    w1, [x0]
		ldr     x0, [sp, 0x8]
        ldrb    w0, [x0]
        cmp     w1, w0
        bls     exit_failure

		mov		w0, 0x1
		b		exit

	exit_failure:
		mov		w0, 0x1

	exit:
		add		sp, sp, 0x10
		ret

