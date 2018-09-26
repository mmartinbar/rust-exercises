#include "handshake_verifier.h"

int Ext_Handshake_PingMessage_verify_as_root(const void *buf, size_t bufsiz)
{
    return Handshake_PingMessage_verify_as_root(buf, bufsiz);
}

int Ext_Handshake_PingMessage_verify_as_typed_root(const void *buf, size_t bufsiz)
{
    return Handshake_PingMessage_verify_as_typed_root(buf, bufsiz);
}

int Ext_Handshake_PingMessage_verify_as_root_with_identifier(const void *buf, size_t bufsiz, const char *fid)
{
    return Handshake_PingMessage_verify_as_root_with_identifier(buf, bufsiz, fid);
}

int Ext_Handshake_PingMessage_verify_as_root_with_type_hash(const void *buf, size_t bufsiz, flatbuffers_thash_t thash)
{
    return Handshake_PingMessage_verify_as_root_with_type_hash(buf, bufsiz, thash);
}
