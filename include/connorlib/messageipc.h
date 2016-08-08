// The MIT License (MIT) 
// Copyright (c) 2016 Connor Hilarides
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#pragma once

#include <connorlib/dll.h>
#include <connorlib/optional.h>
#include <stdint.h>

namespace MessageIpc
{
    namespace FFI
    {
        struct IpcClient;

        const int MIPC_SUCCESS = 0;
        const int MIPC_EMPTY = 1;
        const int MIPC_DISCONNECTED = 2;

        extern "C" IPC_DLL_IMPORT IpcClient *mipc_open_server(const char *name);
        extern "C" IPC_DLL_IMPORT IpcClient *mipc_open_client(const char *name, uint32_t pid);
        extern "C" IPC_DLL_IMPORT void mipc_close(IpcClient *client);

        extern "C" IPC_DLL_IMPORT int mipc_send(IpcClient *client, const uint8_t *data, uint32_t len);
        extern "C" IPC_DLL_IMPORT int mipc_recv(IpcClient *client, uint8_t **data, size_t *len);
        extern "C" IPC_DLL_IMPORT int mipc_try_recv(IpcClient *client, uint8_t **data, size_t *len);
        extern "C" IPC_DLL_IMPORT void mipc_recv_free(uint8_t *data, size_t len);
    }

    class IpcMessage
    {
    public:
        inline IpcMessage(uint8_t *data, size_t len)
            : data_(data), len_(len)
        {
        }

        inline ~IpcMessage()
        {
            if (data_)
            {
                FFI::mipc_recv_free(data_, len_);
            }
        }

        IpcMessage(const IpcMessage &) = delete;
        inline IpcMessage(IpcMessage &&move)
            : data_(move.data_), len_(move.len_)
        {
            move.data_ = nullptr;
        }

        IpcMessage &operator=(const IpcMessage &) = delete;
        inline IpcMessage &operator=(IpcMessage &&move)
        {
            data_ = move.data_;
            len_ = move.len_;
            move.data_ = nullptr;
        }

        inline const uint8_t *data() const
        {
            return data_;
        }

        inline size_t len() const
        {
            return len_;
        }

        inline size_t size() const
        {
            return len_;
        }

    private:
        uint8_t *data_;
        size_t len_;
    }

    class IpcClient
    {
    public:
        inline ~IpcClient()
        {
            if (client_)
            {
                FFI::mipc_close(client_);
            }
        }

        IpcClient(const IpcClient &) = delete;
        inline IpcClient(IpcClient &&move)
            : client_(move.client_)
        {
            move.client_ = nullptr;
        }

        IpcClient &operator=(const IpcClient &) = delete;
        inline IpcClient &operator=(IpcClient &&move)
        {
            client_ = move.client_;
            move.client_ = nullptr;
            return *this;
        }

        inline static std::optional<IpcClient> OpenServer(const char *name)
        {
            if (auto ptr = FFI::mipc_open_server(name))
                return IpcClient(ptr);
            return std::nullopt;
        }

        inline static std::optional<IpcClient> OpenClient(const char *name, uint32_t pid)
        {
            if (auto ptr = FFI::mipc_open_client(name, pid))
                return IpcClient(ptr);
            return std::nullopt;
        }

        inline std::optional<IpcMessage> Recv()
        {
            uint8_t *data;
            size_t len;
            if (FFI::mipc_recv(client_, &data, &len) == FFI::MIPC_SUCCESS)
            {
                return IpcMessage(data, len);
            }
            return std::nullopt;
        }

        inline std::optional<IpcMessage> TryRecv(bool &disconnected)
        {
            uint8_t *data;
            size_t len;

            disconnected = false;
            switch (FFI::mipc_try_recv(client_, &data, &len))
            {
                case MIPC_SUCCESS:
                    return IpcMessage(data, len);
                case MIPC_EMPTY:
                    return std::nullopt;
                case MIPC_DISCONNECTED:
                    disconnected = true;
                    return std::nullopt;
                default:
                    throw std::runtime_error("Unknown status code returned from mipc_try_recv");
            }
        }

    private:
        inline explicit IpcClient(FFI::IpcClient *client)
            : client_(client)
        {
        }

        FFI::IpcClient *client_;
    };
}
