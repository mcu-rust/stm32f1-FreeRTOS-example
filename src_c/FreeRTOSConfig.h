#ifndef FREERTOS_CONFIG_H
#define FREERTOS_CONFIG_H
#include <stdbool.h>
#include <stdint.h>

extern void assert_callback(uint32_t ulLine, const char *const pcFileName);
#define configASSERT(x) \
    if ((x) == 0)       \
    assert_callback(__LINE__, __FILE__)

#define vPortSVCHandler     SVCall
#define xPortPendSVHandler  PendSV
#define xPortSysTickHandler SysTick

/*-----------------------------------------------------------
 * Application specific definitions.
 *
 * These definitions should be adjusted for your particular hardware and
 * application requirements.
 *
 * THESE PARAMETERS ARE DESCRIBED WITHIN THE 'CONFIGURATION' SECTION OF THE
 * FreeRTOS API DOCUMENTATION AVAILABLE ON THE FreeRTOS.org WEB SITE.
 *
 * See http://www.freertos.org/a00110.html.
 *----------------------------------------------------------*/

#define configUSE_PREEMPTION           1
#define configUSE_IDLE_HOOK            0
#define configUSE_TICK_HOOK            0
#define configCPU_CLOCK_HZ             (72000000UL)
#define configTICK_RATE_HZ             ((TickType_t)1000)
#define configMAX_PRIORITIES           (5)
#define configMINIMAL_STACK_SIZE       ((uint16_t)40)
#define configTOTAL_HEAP_SIZE          ((size_t)(16 * 1024))
#define configMAX_TASK_NAME_LEN        (16)
#define configUSE_TRACE_FACILITY       1
#define configUSE_16_BIT_TICKS         0
#define configIDLE_SHOULD_YIELD        1
#define configUSE_MUTEXES              1
#define configQUEUE_REGISTRY_SIZE      5
#define configCHECK_FOR_STACK_OVERFLOW 0
#define configUSE_RECURSIVE_MUTEXES    0
#define configUSE_MALLOC_FAILED_HOOK   0
#define configUSE_APPLICATION_TASK_TAG 0
#define configUSE_COUNTING_SEMAPHORES  1

// http://www.freertos.org/Configuring-a-real-time-RTOS-application-to-use-software-timers.html
#define configUSE_TIMERS             1
#define configTIMER_TASK_PRIORITY    1
#define configTIMER_QUEUE_LENGTH     10
#define configTIMER_TASK_STACK_DEPTH 200

// optional processing for sleep tickless mode (short tickless periods)
#define configPRE_STOP_PROCESSING(x)
#define configPOST_STOP_PROCESSING(x)

// optional processing for stop (deep sleep) tickless mode (long tickless periods > 5 sec)
// note: clock is potentially slowed down if enough ticks could be suppressed!
// so perform sysclock check before invoke timing relevant function as uart tx!
#define configPRE_SLEEP_PROCESSING(x)
#define configPOST_SLEEP_PROCESSING(x)

/* Co-routine definitions. */
#define configUSE_CO_ROUTINES           0
#define configMAX_CO_ROUTINE_PRIORITIES (2)

/* Set the following definitions to 1 to include the API function, or zero
to exclude the API function. */

#define INCLUDE_vTaskPrioritySet            1
#define INCLUDE_uxTaskPriorityGet           1
#define INCLUDE_vTaskCleanUpResources       0
#define INCLUDE_vTaskSuspend                0
#define INCLUDE_uxTaskGetStackHighWaterMark 1
#define INCLUDE_eTaskGetState               1

/* Use the system definition, if there is one */
// #ifdef __NVIC_PRIO_BITS
//     #define configPRIO_BITS       __NVIC_PRIO_BITS
// #else
//     #define configPRIO_BITS       4        /* 15 priority levels */
// #endif
#define configPRIO_BITS 4 /* 15 priority levels*/

#define configLIBRARY_LOWEST_INTERRUPT_PRIORITY      15
#define configLIBRARY_MAX_SYSCALL_INTERRUPT_PRIORITY 5

/* The lowest priority. */
#define configKERNEL_INTERRUPT_PRIORITY (configLIBRARY_LOWEST_INTERRUPT_PRIORITY << (8 - configPRIO_BITS))
/* Priority 5, or 95 as only the top four bits are implemented. */
/* !!!! configMAX_SYSCALL_INTERRUPT_PRIORITY must not be set to zero !!!!
See http://www.FreeRTOS.org/RTOS-Cortex-M3-M4.html. */
#define configMAX_SYSCALL_INTERRUPT_PRIORITY (configLIBRARY_MAX_SYSCALL_INTERRUPT_PRIORITY << (8 - configPRIO_BITS))

#endif /* FREERTOS_CONFIG_H */
