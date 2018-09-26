#ifndef HANDSHAKE_BUILDER_H
#define HANDSHAKE_BUILDER_H

/* Generated by flatcc 0.5.3-pre FlatBuffers schema compiler for C by dvide.com */

#ifndef HANDSHAKE_READER_H
#include "handshake_reader.h"
#endif
#ifndef FLATBUFFERS_COMMON_BUILDER_H
#include "flatbuffers_common_builder.h"
#endif
#include "flatcc/flatcc_prologue.h"
#ifndef flatbuffers_identifier
#define flatbuffers_identifier 0
#endif
#ifndef flatbuffers_extension
#define flatbuffers_extension ".bin"
#endif

#define __Handshake_Version_formal_args , uint8_t v0, uint8_t v1
#define __Handshake_Version_call_args , v0, v1
static inline Handshake_Version_t *Handshake_Version_assign(Handshake_Version_t *p, uint8_t v0, uint8_t v1)
{ p->major = v0; p->minor = v1;
  return p; }
static inline Handshake_Version_t *Handshake_Version_copy(Handshake_Version_t *p, const Handshake_Version_t *p2)
{ p->major = p2->major; p->minor = p2->minor;
  return p; }
static inline Handshake_Version_t *Handshake_Version_assign_to_pe(Handshake_Version_t *p, uint8_t v0, uint8_t v1)
{ p->major = v0; p->minor = v1;
  return p; }
static inline Handshake_Version_t *Handshake_Version_copy_to_pe(Handshake_Version_t *p, const Handshake_Version_t *p2)
{ p->major = p2->major; p->minor = p2->minor;
  return p; }
static inline Handshake_Version_t *Handshake_Version_assign_from_pe(Handshake_Version_t *p, uint8_t v0, uint8_t v1)
{ p->major = v0; p->minor = v1;
  return p; }
static inline Handshake_Version_t *Handshake_Version_copy_from_pe(Handshake_Version_t *p, const Handshake_Version_t *p2)
{ p->major = p2->major; p->minor = p2->minor;
  return p; }
__flatbuffers_build_struct(flatbuffers_, Handshake_Version, 2, 1, Handshake_Version_identifier, Handshake_Version_type_identifier)

static const flatbuffers_voffset_t __Handshake_PingMessage_required[] = { 0 };
typedef flatbuffers_ref_t Handshake_PingMessage_ref_t;
static Handshake_PingMessage_ref_t Handshake_PingMessage_clone(flatbuffers_builder_t *B, Handshake_PingMessage_table_t t);
__flatbuffers_build_table(flatbuffers_, Handshake_PingMessage, 3)

static const flatbuffers_voffset_t __Handshake_PongMessage_required[] = { 0 };
typedef flatbuffers_ref_t Handshake_PongMessage_ref_t;
static Handshake_PongMessage_ref_t Handshake_PongMessage_clone(flatbuffers_builder_t *B, Handshake_PongMessage_table_t t);
__flatbuffers_build_table(flatbuffers_, Handshake_PongMessage, 2)

#define __Handshake_PingMessage_formal_args , Handshake_Version_t *v0, uint64_t v1, flatbuffers_string_ref_t v2
#define __Handshake_PingMessage_call_args , v0, v1, v2
static inline Handshake_PingMessage_ref_t Handshake_PingMessage_create(flatbuffers_builder_t *B __Handshake_PingMessage_formal_args);
__flatbuffers_build_table_prolog(flatbuffers_, Handshake_PingMessage, Handshake_PingMessage_identifier, Handshake_PingMessage_type_identifier)

#define __Handshake_PongMessage_formal_args , Handshake_Version_t *v0, uint64_t v1
#define __Handshake_PongMessage_call_args , v0, v1
static inline Handshake_PongMessage_ref_t Handshake_PongMessage_create(flatbuffers_builder_t *B __Handshake_PongMessage_formal_args);
__flatbuffers_build_table_prolog(flatbuffers_, Handshake_PongMessage, Handshake_PongMessage_identifier, Handshake_PongMessage_type_identifier)

__flatbuffers_build_struct_field(0, flatbuffers_, Handshake_PingMessage_version, Handshake_Version, 2, 1, Handshake_PingMessage)
__flatbuffers_build_scalar_field(1, flatbuffers_, Handshake_PingMessage_timestamp, flatbuffers_uint64, uint64_t, 8, 8, UINT64_C(0), Handshake_PingMessage)
__flatbuffers_build_string_field(2, flatbuffers_, Handshake_PingMessage_message, Handshake_PingMessage)

static inline Handshake_PingMessage_ref_t Handshake_PingMessage_create(flatbuffers_builder_t *B __Handshake_PingMessage_formal_args)
{
    if (Handshake_PingMessage_start(B)
        || Handshake_PingMessage_timestamp_add(B, v1)
        || Handshake_PingMessage_message_add(B, v2)
        || Handshake_PingMessage_version_add(B, v0)) {
        return 0;
    }
    return Handshake_PingMessage_end(B);
}

static Handshake_PingMessage_ref_t Handshake_PingMessage_clone(flatbuffers_builder_t *B, Handshake_PingMessage_table_t t)
{
    __flatbuffers_memoize_begin(B, t);
    if (Handshake_PingMessage_start(B)
        || Handshake_PingMessage_timestamp_pick(B, t)
        || Handshake_PingMessage_message_pick(B, t)
        || Handshake_PingMessage_version_pick(B, t)) {
        return 0;
    }
    __flatbuffers_memoize_end(B, t, Handshake_PingMessage_end(B));
}

__flatbuffers_build_struct_field(0, flatbuffers_, Handshake_PongMessage_version, Handshake_Version, 2, 1, Handshake_PongMessage)
__flatbuffers_build_scalar_field(1, flatbuffers_, Handshake_PongMessage_timestamp, flatbuffers_uint64, uint64_t, 8, 8, UINT64_C(0), Handshake_PongMessage)

static inline Handshake_PongMessage_ref_t Handshake_PongMessage_create(flatbuffers_builder_t *B __Handshake_PongMessage_formal_args)
{
    if (Handshake_PongMessage_start(B)
        || Handshake_PongMessage_timestamp_add(B, v1)
        || Handshake_PongMessage_version_add(B, v0)) {
        return 0;
    }
    return Handshake_PongMessage_end(B);
}

static Handshake_PongMessage_ref_t Handshake_PongMessage_clone(flatbuffers_builder_t *B, Handshake_PongMessage_table_t t)
{
    __flatbuffers_memoize_begin(B, t);
    if (Handshake_PongMessage_start(B)
        || Handshake_PongMessage_timestamp_pick(B, t)
        || Handshake_PongMessage_version_pick(B, t)) {
        return 0;
    }
    __flatbuffers_memoize_end(B, t, Handshake_PongMessage_end(B));
}

#include "flatcc/flatcc_epilogue.h"
#endif /* HANDSHAKE_BUILDER_H */
