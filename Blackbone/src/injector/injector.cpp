#include "injector.hpp"

namespace Injector
{
    EXTERN_C __declspec(dllexport) bool inject(const wchar_t *name, const char *url)
    {
        blackbone::Process proc;

        switch (auto p = proc.Attach(name); p)
        {
        case STATUS_SUCCESS:
        {
            wrap::Response response = wrap::HttpsRequest(wrap::Url{url});

            proc.mmap().MapImage(response.raw.size(), response.raw.data());

            proc.Detach();

            return true;
        }
        default:
        {
            return false;
        }
        }
    }
}