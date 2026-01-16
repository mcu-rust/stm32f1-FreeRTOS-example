#ifndef FREERTOS_CONFIG_H
#define FREERTOS_CONFIG_H

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

#define configUSE_PREEMPTION      1
#define configCPU_CLOCK_HZ        (72000000UL)
#define configMAX_PRIORITIES      (5)
#define configMINIMAL_STACK_SIZE  ((uint16_t)40)
#define configTOTAL_HEAP_SIZE     ((size_t)(16 * 1024))
#define configMAX_TASK_NAME_LEN   8
#define configIDLE_SHOULD_YIELD   1
#define configQUEUE_REGISTRY_SIZE 5

// http://www.freertos.org/Configuring-a-real-time-RTOS-application-to-use-software-timers.html
#define configUSE_TIMERS             1
#define configTIMER_TASK_PRIORITY    1
#define configTIMER_QUEUE_LENGTH     10
#define configTIMER_TASK_STACK_DEPTH 200

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
