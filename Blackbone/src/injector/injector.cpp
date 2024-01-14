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
            std::string bytes;

            if (std::string(url).rfind("http", 0) == 0)
            {
                wrap::Response response = wrap::HttpsRequest(wrap::Url{url});

                bytes = std::string{response.raw.data(), response.raw.size()};
            }
            else
            {
                std::ifstream file(url, std::ios::in | std::ios::binary);

                if (!file)
                {
                    return false;
                }

                file.seekg(0, std::ios::end);

                bytes.resize(file.tellg());

                file.seekg(0, std::ios::beg);

                file.read(bytes.data(), bytes.size());

                file.close();
            }

            proc.pid();

            proc.mmap()
                .MapImage(bytes.size(), bytes.data());

            proc.Detach();

            return true;
        }
        default:
        {
            return false;
        }
        }
    }

    EXTERN_C __declspec(dllexport) DWORD get_module(const wchar_t *proc_name, const wchar_t *name)
    {
        blackbone::Process proc;

        switch (auto p = proc.Attach(proc_name); p)
        {
        case STATUS_SUCCESS:
        {
            return static_cast<DWORD>(proc.modules().GetModule(name).get()->baseAddress);
        }
        default:
        {
            return 0x0;
        }
        }
    }

    EXTERN_C __declspec(dllexport) DWORD get_pid(const wchar_t *name)
    {
        blackbone::Process proc;

        switch (auto p = proc.Attach(name); p)
        {
        case STATUS_SUCCESS:
        {
            return proc.pid();
        }
        default:
        {
            return 0;
        }
        }
    }
}