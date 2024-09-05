    .extern  OSTaskSwHook
    .extern  OS_CPU_ExceptStkBase
    .extern  OS_KA_BASEPRI_Boundary
    .extern  OSTCBHighrdy
    
    .global  OSStartHighRdy                                     @ Functions declared in this file
    .global  OS_CPU_SR_Save
    .global  OS_CPU_SR_Restore  
    .global  OSIntCtxSw
    .global  OSCtxSw
    .global  PendSV_Handler
    .global  cli

@                     OS_ENTER_CRITICAL();             /* cpu_sr = OS_CPU_SR_Save(new_basepri);    */
@                     OS_EXIT_CRITICAL();              /* OS_CPU_RestoreSR(cpu_sr);                */

.equ NVIC_INT_CTRL,     0xE000ED04                              @ Interrupt control state register.
.equ NVIC_SYSPRI14,     0xE000ED22                              @ System priority register (priority 14).
.equ NVIC_PENDSV_PRI,   0xFF                                    @ PendSV priority value (lowest).
.equ NVIC_PENDSVSET,    0x10000000                              @ Value to trigger PendSV exception.

    .text
    .align 2
    .thumb
    .syntax unified


.thumb_func
OSCtxSw:
OSIntCtxSw:
    LDR     R0, =NVIC_INT_CTRL                                  @ Trigger the PendSV exception (causes context switch)
    LDR     R1, =NVIC_PENDSVSET
    STR     R1, [R0]
    BX      LR


.thumb_func
OS_CPU_SR_Save:
    CPSID   I                                   @ Cortex-M7 errata notice. See Note #2
    PUSH   {R1}
    MRS     R1, BASEPRI
    MSR     BASEPRI, R0
    DSB
    ISB
    MOV     R0, R1
    POP    {R1}
    CPSIE   I
    BX      LR

.thumb_func
OS_CPU_SR_Restore:
    CPSID   I                                   @ Cortex-M7 errata notice. See Note #2
    MSR     BASEPRI, R0
    DSB
    ISB
    CPSIE   I
    BX      LR

@ Note(s) : 1) PendSV is used to cause a context switch.  This is a recommended method for performing
@              context switches with Cortex-M.  This is because the Cortex-M auto-saves half of the
@              processor context on any exception, and restores same on return from exception.  So only
@              saving of R4-R11 & R14 is required and fixing up the stack pointers. Using the PendSV exception
@              this way means that context saving and restoring is identical whether it is initiated from
@              a thread or occurs due to an interrupt or exception.
@ 
@    a) The following have been saved on the process stack (by processor):
@                 xPSR, PC, LR, R12, R0-R3

.thumb_func
PendSV_Handler:
@ 中断屏蔽
    CPSID   I                                                   @ Cortex-M7 errata notice. See Note #5
    MOVW    R2, #:lower16:OS_KA_BASEPRI_Boundary                @ Set BASEPRI priority level required for exception preemption
    MOVT    R2, #:upper16:OS_KA_BASEPRI_Boundary
    LDR     R1, [R2]
    MSR     BASEPRI, R1
    DSB
    ISB
    CPSIE   I
@ 保存当前任务的上下文,这里PSP是任务的堆栈指针,只需要存储R4-R11,R14
    MRS     R0, PSP                                             @ PSP is process stack pointer
    STMFD   R0!, {R4-R11, R14}                                  @ Save remaining regs r4-11, R14 on process stack
@ 存储当前任务的堆栈指针到任务的TCB的Ptr位置
    @ MOVW    R5, #:lower16:OSTCBCur                              @ OSTCBCur->OSTCBStkPtr = SP;
    @ MOVT    R5, #:upper16:OSTCBCur
    @ 我认为可以用下面这句来代替上面两句
    LDR     R5, =OSTCBCur
    LDR     R1, [R5]
    STR     R0, [R1]                                            @ R0 is SP of process being switched out 这里很有趣，因为OSTCBStkPtr正好是OSTCBCur的第一个元素

                                                                @ At this point, entire context of process has been saved
    MOV     R4, LR                                              @ Save LR exc_return value
    BL      OSTaskSwHook                                        @ Call OSTaskSwHook() for FPU Push & Pop
@   这里就是使用:lower16把目标数据拆开成两个部分高低16位
    @ MOVW    R0, #:lower16:OSPrioCur                             @ OSPrioCur   = OSPrioHighRdy;
    @ MOVT    R0, #:upper16:OSPrioCur
    @ MOVW    R1, #:lower16:OSPrioHighRdy
    @ MOVT    R1, #:upper16:OSPrioHighRdy
    @ 转换当前的运行优先级,变成找出的OSPrioHighRdy
    LDR     R0, =OSPrioCur
    LDR     R1, =OSPrioHighRdy
    LDRB    R2, [R1]
    STRB    R2, [R0]

    @ MOVW    R1, #:lower16:OSTCBHighRdy                          @ OSTCBCur  = OSTCBHighRdy;
    @ MOVT    R1, #:upper16:OSTCBHighRdy
    LDR     R1, =OSTCBHighRdy
    LDR     R2, [R1]
    STR     R2, [R5]

    ORR     LR,  R4, #0x04                                      @ Ensure exception return uses process stack
    LDR     R0,  [R2]                                           @ R0 is new process SP; SP = OSTCBHighRdy->OSTCBStkPtr;
    LDMFD   R0!, {R4-R11, R14}                                  @ Restore r4-11, R14 from new process stack
    MSR     PSP, R0                                             @ Load PSP with new process SP

    MOV     R2, #0                                              @ Restore BASEPRI priority level to 0
    CPSID   I
    MSR     BASEPRI, R2
    DSB
    ISB
    CPSIE   I
    BX      LR                                                  @ Exception return will restore remaining context


@ Note(s) : 1) This function triggers a PendSV exception (essentially, causes a context switch) to cause
@              the first task to start.
@
@           2) During task execution, PSP is used as the stack pointer.
@              When an exception occurs, the core will switch to MSP until the exception return.
@
@           3) OSStartHighRdy() MUST:
@              a) Setup PendSV exception priority to lowest;
@              b) Set initial PSP to 0, to tell context switcher this is first run;
@              c) Set the main stack to OS_CPU_ExceptStkBase
@              d) Set OSRunning to TRUE;
@              e) Get current high priority, OSPrioCur = OSPrioHighRdy;
@              f) Get current ready thread TCB, OSTCBCur = OSTCBHighRdy;
@              g) Get new process SP from TCB, SP = OSTCBHighRdy->OSTCBStkPtr;
@              h) Restore R0-R11 and R14 from new process stack;
@              i) Enable interrupts (tasks will run with interrupts enabled).

.thumb_func
OSStartHighRdy:
    CPSID   I                                                   @ Prevent interruption during context switch
    @ MOVW    R0, #:lower16:NVIC_SYSPRI14                         @ Set the PendSV exception priority
    @ MOVT    R0, #:upper16:NVIC_SYSPRI14
@ 我觉得可以换成下面这一句，待实验
    LDR     R0,=NVIC_SYSPRI14

    @ MOVW    R1, #:lower16:NVIC_PENDSV_PRI
    @ MOVT    R1, #:upper16:NVIC_PENDSV_PRI
    LDR     R1, =NVIC_PENDSV_PRI
    STRB    R1, [R0]

    MOVS    R0, #0                                              @ Set the PSP to 0 for initial context switch call
    MSR     PSP, R0
@ OS_CPU_ExceptStkBase是一个全局变量，用来存储异常堆栈的栈顶地址。栈向下增长
@ 这里应该利用不了伪指令，因为这个地址是一个全局变量，不是一个常量，不过还是试一试
    @ MOVW    R0, #:lower16:OS_CPU_ExceptStkBase                  @ Initialize the MSP to the OS_CPU_ExceptStkBase
    @ MOVT    R0, #:upper16:OS_CPU_ExceptStkBase
    LDR     R0, =OS_CPU_ExceptStkBase
    LDR     R1, [R0]
    MSR     MSP, R1

    BL      OSTaskSwHook                                        @ Call OSTaskSwHook() for FPU Push & Pop
@ 把OSRunning设置为true
    LDR     R0, =OSRunning                                      @ OSRunning = TRUE
    MOVS    R1, #1
    STRB    R1, [R0]

    @ MOVW    R0, #:lower16:OSPrioCur                             @ OSPrioCur   = OSPrioHighRdy;
    @ MOVT    R0, #:upper16:OSPrioCur
    LDR     R0, =OSPrioCur
    @ MOVW    R1, #:lower16:OSPrioHighRdy
    @ MOVT    R1, #:upper16:OSPrioHighRdy
    LDR     R1, =OSPrioHighRdy
    LDRB    R2, [R1]
    STRB    R2, [R0]

    @ MOVW    R0, #:lower16:OSTCBCur                              @ OSTCBCur  = OSTCBHighRdy;
    @ MOVT    R0, #:upper16:OSTCBCur
    LDR     R0, =OSTCBCur
    @ MOVW    R1, #:lower16:OSTCBHighRdy
    @ MOVT    R1, #:upper16:OSTCBHighRdy
    LDR     R1, =OSTCBHighRdy
    LDR     R2, [R1]
    STR     R2, [R0]

    LDR     R0, [R2]                                            @ R0 is new process SP; SP = OSTCBHighRdy->OSTCBStkPtr;
    MSR     PSP, R0                                             @ Load PSP with new process SP

    MRS     R0, CONTROL
    ORR     R0, R0, #2
    MSR     CONTROL, R0
    ISB                                                         @ Sync instruction stream

    LDMFD    SP!, {R4-R11, LR}                                  @ Restore r4-11, lr from new process stack
    LDMFD    SP!, {R0-R3}                                       @ Restore r0, r3
    LDMFD    SP!, {R12, LR}                                     @ Load R12 and LR
    LDMFD    SP!, {R1, R2}                                      @ Load PC and discard xPSR
    CPSIE    I
    BX       R1

cli:
    MRS R0, xPSR
    STMFD sp!,{R0}
    MOV PC,LR

.end
