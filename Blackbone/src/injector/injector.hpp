#pragma comment(lib, "BlackBone.lib")
#include <blackbone/src/BlackBone/Process/Process.h>

#include <wnetwrap/wnetwrap.h>

namespace Injector
{
    EXTERN_C __declspec(dllexport) bool inject(const wchar_t *name, const char *url);
}