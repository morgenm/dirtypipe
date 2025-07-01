BITS 64
ORG 0x400000

ehdr:                                   ; ELF header
    db 0x7f, "ELF"                      ; Magic
    db 2, 1, 1, 0                       ; 64-bit, little endian, version
    times 8 db 0                       ; padding
    dw 2                                ; e_type = EXEC
    dw 0x3e                             ; e_machine = AMD64
    dd 1                                ; e_version
    dq _start                           ; e_entry
    dq phdr - $$                        ; e_phoff
    dq 0                                ; e_shoff
    dd 0                                ; e_flags
    dw ehdrsize                         ; e_ehsize
    dw phdrsize                         ; e_phentsize
    dw 1                                ; e_phnum
    dw 0, 0, 0                          ; e_shentsize, e_shnum, e_shstrndx

ehdrsize equ $ - ehdr

phdr:                                   ; Program header
    dd 1                                ; PT_LOAD
    dd 5                                ; PF_X | PF_R
    dq 0                                ; p_offset
    dq $$                               ; p_vaddr
    dq $$                               ; p_paddr
    dq filesize                         ; p_filesz
    dq filesize                         ; p_memsz
    dq 0x1000                           ; p_align

phdrsize equ $ - phdr

_start:
    call shellcode

shellcode:
    incbin "shellcode.bin"

filesize equ $ - $$
