#pragma comment(lib, "BlackBone.lib")
#include <blackbone/src/BlackBone/Process/Process.h>

#include <wnetwrap/wnetwrap.h>

namespace Injector
{
    EXTERN_C __declspec(dllexport) bool inject(const wchar_t *name, const char *url);
    EXTERN_C __declspec(dllexport) DWORD get_module(const wchar_t *proc_name, const wchar_t *name);
    EXTERN_C __declspec(dllexport) DWORD get_pid(const wchar_t *name);
}