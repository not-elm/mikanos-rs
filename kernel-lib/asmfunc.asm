; asmfunc.asm
;
; System V AMD64 Calling Convention
; Registers: RDI, RSI, RDX, RCX, R8, R9

bits 64
section .text

global CallApp
CallApp:  ; void CallApp(int argc, char** argv, uint16_t cs, uint16_t ss, uint64_t rip, uint64_t rsp);
    push rbp
    mov rbp, rsp
    push rcx  ; SS
    push r9   ; RSP
    push rdx  ; CS
    push r8   ; RIP
    o64 retf