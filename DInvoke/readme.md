# Tartocitron with DInvoke

## DInvoke?
DInvoke stands for Dynamic Invoke, which performs API resolving and calling at runtime. By doing so, API resolved this way are not mentioned in the executable IAT. However, pure DInvoke approach does not defeat userland API hooking performed by EDR.