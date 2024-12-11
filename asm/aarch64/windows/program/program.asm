	AREA .text, CODE, READONLY
	EXPORT WinMainCRTStartup
	
WinMainCRTStartup PROC
		mov		x0, 0x0
		mov		x1, 0x0
		svc		#44
	ENDP
	END
