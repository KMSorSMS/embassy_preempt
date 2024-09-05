#include "NVIC.h"


//设置NVIC分组
//主要寄存器： SCB_AIRCR[10:8] : 设置分组
//            SCB_AIRCR[31:16] : 写入时将0x5FA 写入VECTKEY否则将忽略写入
//                     [2] : SYSRESETREQ系统复位请求
//                     [1] : 保留供调试使用。该位读取为0。写入寄存器时，必须将0写入这个位，否则行为是不可预测的。
//                     [0] : 保留供调试使用。该位读取为0。写入寄存器时，必须将0写入这个位，否则行为是不可预测的。
/**
  * 
*/
void my_nvic_priorityGroupConfig(uint32_t NVIC_PriorityGroup){
    uint32_t value = 0;
    value = SCB->AIRCR;
    value &= ~(0xffffU << 16);//将密钥置零
    value |= (0x05FA << 16);//写入密钥
    value  &= ~(0x7<<8);//将分组的位置零
    value |= (~(NVIC_PriorityGroup)&0x7)<<8;//设置分组
    SCB->AIRCR = value;
}

uint32_t get_nvic_group(void){
  return (uint32_t)((SCB->AIRCR & (0x7 << 0x8)) >> 0x8);//取出中断的分组,经过取反的分组
}
/**
 *主要寄存器 ：NVIC_IPRx 只使用高4bits
 *            
 *            
*/
void my_nvic_set_priority(uint8_t NVIC_IRQChannel,uint8_t NVIC_IRQChannelPreemptionPriority,uint8_t NVIC_IRQChannelSubPriority){
    uint32_t NVIC_group = (get_nvic_group() & (uint32_t)0x7);
    uint32_t PreemptPriorityBits;
    uint32_t SubPriorityBits;
    uint32_t tempPriority = 0;

    PreemptPriorityBits  = ((0x7 - NVIC_group) > (uint32_t)0x4) ? (uint32_t)(0x4) : (uint32_t)(0x7 - NVIC_group); //计算抢占优先级的位数
    SubPriorityBits = ((NVIC_group + (uint32_t)0x4) < (uint32_t)0x7) ? (uint32_t)0x0 : (uint32_t)(0x4 - (0x7 - NVIC_group)); //计算响应优先级的位数
    tempPriority = (
      ((NVIC_IRQChannelPreemptionPriority & (uint32_t)((1UL << (PreemptPriorityBits)) - 1UL)) << SubPriorityBits) |//将中断优先级移动到适合的位置
      ((NVIC_IRQChannelSubPriority     & (uint32_t)((1UL << (SubPriorityBits    )) - 1UL)))
    );
    NVIC->IP[(uint32_t)NVIC_IRQChannel] = (uint8_t)((tempPriority << 0x4)& (uint32_t)0xff);//设置中断优先级
}
/**
 *  主要寄存器： NVIC_ISERx 根据中断号来使能中断
 *              NVIC_ISERx 共有8个这样的32位寄存器，每一位都支持一位的中断使能，最后一个寄存器的高16位没有用到
*/
void my_nvic_enable(uint8_t NVIC_IRQChannel){
    NVIC->ISER[ NVIC_IRQChannel >> 0x5] |= (uint32_t)( 0x01 << (NVIC_IRQChannel & (uint8_t)0x1f));
}
