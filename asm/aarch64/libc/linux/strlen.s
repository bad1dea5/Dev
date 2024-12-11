.global strlen

.text
	//
	// 	strlen
	//
	//	arguments
	//
	//		x0:		const char*		buffer
	//		x1:		unsigned long	maximum characters
	//		x2:		unsigned long*	length
	//
	//	return value
	//
	//		x0:		int				return value
	//
	//		 0:	success
	//		-1:	failure
	//
	strlen:
		sub		sp, sp, 0x30

		str		x0, [sp, 0x0]
		str		x1, [sp, 0x8]
		str		x2, [sp, 0x10]

		ldr		x0, [sp, 0x8]
		str		x0, [sp, 0x18]

		ldr		x0, [sp, 0x0]
		cmp		x0, 0x0
		beq		exit_failure

		ldr		x0, [sp, 0x8]
		cmp		x0, 0x0
		bne		loop_check

	exit_failure:
		mov		w0, -1
		b		exit

	loop:
		ldr		x0, [sp, 0x0]
		add		x0, x0, 0x1
		str		x0, [sp, 0x0]

		ldr     x0, [sp, 0x8]
        sub     x0, x0, 0x1
        str     x0, [sp, 0x8]

	loop_check:
		ldr		x0, [sp, 0x8]
		cmp		x0, 0x0
		beq		is_maximum_zero

		ldr		x0, [sp, 0x0]
		ldrb	w0, [x0]
		cmp		w0, 0x0
		bne		loop

	is_maximum_zero:
		ldr     x0, [sp, 0x8]
		cmp		x0, 0x0
		bne		calculate_length
		b		exit_failure

	calculate_length:
		ldr		x0, [sp, 0x10]
		cmp		x0, 0x0
		beq		exit_success

		ldr		x1, [sp, 0x18]
		ldr		x0, [sp, 0x8]
		sub		x1, x1, x0

		ldr		x0, [sp, 0x10]
		str		x1, [x0]

	exit_success:
		mov		w0, 0x0

	exit:
		add		sp, sp, 0x30
		ret

