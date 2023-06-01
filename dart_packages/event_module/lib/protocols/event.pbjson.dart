///
//  Generated code. Do not modify.
//  source: event.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use eventTypeDescriptor instead')
const EventType$json = const {
  '1': 'EventType',
  '2': const [
    const {'1': 'type_id', '3': 1, '4': 1, '5': 9, '10': 'typeId'},
    const {'1': 'name', '3': 2, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    const {'1': 'description', '3': 4, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `EventType`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List eventTypeDescriptor = $convert.base64Decode('CglFdmVudFR5cGUSFwoHdHlwZV9pZBgBIAEoCVIGdHlwZUlkEiIKBG5hbWUYAiABKAsyDi5jYXNobWVyZS5OYW1lUgRuYW1lEiAKC2Rlc2NyaXB0aW9uGAQgASgJUgtkZXNjcmlwdGlvbg==');
@$core.Deprecated('Use eventDescriptor instead')
const Event$json = const {
  '1': 'Event',
  '2': const [
    const {'1': 'type_id', '3': 1, '4': 1, '5': 9, '10': 'typeId'},
    const {'1': 'emitter_id', '3': 2, '4': 1, '5': 9, '10': 'emitterId'},
    const {'1': 'emitter_instance_name', '3': 6, '4': 1, '5': 9, '10': 'emitterInstanceName'},
    const {'1': 'serial_number', '3': 3, '4': 1, '5': 4, '10': 'serialNumber'},
    const {'1': 'timestamp', '3': 4, '4': 1, '5': 4, '10': 'timestamp'},
    const {'1': 'context', '3': 5, '4': 1, '5': 12, '10': 'context'},
    const {'1': 'need_echo', '3': 7, '4': 1, '5': 8, '10': 'needEcho'},
  ],
};

/// Descriptor for `Event`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List eventDescriptor = $convert.base64Decode('CgVFdmVudBIXCgd0eXBlX2lkGAEgASgJUgZ0eXBlSWQSHQoKZW1pdHRlcl9pZBgCIAEoCVIJZW1pdHRlcklkEjIKFWVtaXR0ZXJfaW5zdGFuY2VfbmFtZRgGIAEoCVITZW1pdHRlckluc3RhbmNlTmFtZRIjCg1zZXJpYWxfbnVtYmVyGAMgASgEUgxzZXJpYWxOdW1iZXISHAoJdGltZXN0YW1wGAQgASgEUgl0aW1lc3RhbXASGAoHY29udGV4dBgFIAEoDFIHY29udGV4dBIbCgluZWVkX2VjaG8YByABKAhSCG5lZWRFY2hv');
@$core.Deprecated('Use eventEmitterDescriptor instead')
const EventEmitter$json = const {
  '1': 'EventEmitter',
  '2': const [
    const {'1': 'emitter_id', '3': 1, '4': 1, '5': 9, '10': 'emitterId'},
    const {'1': 'type_id', '3': 2, '4': 1, '5': 9, '10': 'typeId'},
    const {'1': 'name', '3': 3, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    const {'1': 'description', '3': 4, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `EventEmitter`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List eventEmitterDescriptor = $convert.base64Decode('CgxFdmVudEVtaXR0ZXISHQoKZW1pdHRlcl9pZBgBIAEoCVIJZW1pdHRlcklkEhcKB3R5cGVfaWQYAiABKAlSBnR5cGVJZBIiCgRuYW1lGAMgASgLMg4uY2FzaG1lcmUuTmFtZVIEbmFtZRIgCgtkZXNjcmlwdGlvbhgEIAEoCVILZGVzY3JpcHRpb24=');
@$core.Deprecated('Use eventListenerDescriptor instead')
const EventListener$json = const {
  '1': 'EventListener',
  '2': const [
    const {'1': 'listener_id', '3': 1, '4': 1, '5': 9, '10': 'listenerId'},
    const {'1': 'type_id', '3': 2, '4': 1, '5': 9, '10': 'typeId'},
    const {'1': 'name', '3': 3, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    const {'1': 'description', '3': 4, '4': 1, '5': 9, '10': 'description'},
    const {'1': 'is_online', '3': 5, '4': 1, '5': 8, '10': 'isOnline'},
  ],
};

/// Descriptor for `EventListener`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List eventListenerDescriptor = $convert.base64Decode('Cg1FdmVudExpc3RlbmVyEh8KC2xpc3RlbmVyX2lkGAEgASgJUgpsaXN0ZW5lcklkEhcKB3R5cGVfaWQYAiABKAlSBnR5cGVJZBIiCgRuYW1lGAMgASgLMg4uY2FzaG1lcmUuTmFtZVIEbmFtZRIgCgtkZXNjcmlwdGlvbhgEIAEoCVILZGVzY3JpcHRpb24SGwoJaXNfb25saW5lGAUgASgIUghpc09ubGluZQ==');
@$core.Deprecated('Use emitEventRequestDescriptor instead')
const EmitEventRequest$json = const {
  '1': 'EmitEventRequest',
  '2': const [
    const {'1': 'event', '3': 1, '4': 1, '5': 11, '6': '.event.cashmere.Event', '10': 'event'},
  ],
};

/// Descriptor for `EmitEventRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List emitEventRequestDescriptor = $convert.base64Decode('ChBFbWl0RXZlbnRSZXF1ZXN0EisKBWV2ZW50GAEgASgLMhUuZXZlbnQuY2FzaG1lcmUuRXZlbnRSBWV2ZW50');
@$core.Deprecated('Use emitEventResponseDescriptor instead')
const EmitEventResponse$json = const {
  '1': 'EmitEventResponse',
  '2': const [
    const {'1': 'event', '3': 1, '4': 1, '5': 11, '6': '.event.cashmere.Event', '10': 'event'},
  ],
};

/// Descriptor for `EmitEventResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List emitEventResponseDescriptor = $convert.base64Decode('ChFFbWl0RXZlbnRSZXNwb25zZRIrCgVldmVudBgBIAEoCzIVLmV2ZW50LmNhc2htZXJlLkV2ZW50UgVldmVudA==');
@$core.Deprecated('Use emitEventAndListenEchoRequestDescriptor instead')
const EmitEventAndListenEchoRequest$json = const {
  '1': 'EmitEventAndListenEchoRequest',
  '2': const [
    const {'1': 'event', '3': 1, '4': 1, '5': 11, '6': '.event.cashmere.Event', '10': 'event'},
  ],
};

/// Descriptor for `EmitEventAndListenEchoRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List emitEventAndListenEchoRequestDescriptor = $convert.base64Decode('Ch1FbWl0RXZlbnRBbmRMaXN0ZW5FY2hvUmVxdWVzdBIrCgVldmVudBgBIAEoCzIVLmV2ZW50LmNhc2htZXJlLkV2ZW50UgVldmVudA==');
@$core.Deprecated('Use emitEventAndListenEchoResponseDescriptor instead')
const EmitEventAndListenEchoResponse$json = const {
  '1': 'EmitEventAndListenEchoResponse',
  '2': const [
    const {'1': 'event', '3': 1, '4': 1, '5': 11, '6': '.event.cashmere.Event', '10': 'event'},
  ],
};

/// Descriptor for `EmitEventAndListenEchoResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List emitEventAndListenEchoResponseDescriptor = $convert.base64Decode('Ch5FbWl0RXZlbnRBbmRMaXN0ZW5FY2hvUmVzcG9uc2USKwoFZXZlbnQYASABKAsyFS5ldmVudC5jYXNobWVyZS5FdmVudFIFZXZlbnQ=');
@$core.Deprecated('Use listenEventTypeRequestDescriptor instead')
const ListenEventTypeRequest$json = const {
  '1': 'ListenEventTypeRequest',
  '2': const [
    const {'1': 'listener_id', '3': 2, '4': 1, '5': 9, '10': 'listenerId'},
    const {'1': 'type_id', '3': 1, '4': 1, '5': 9, '10': 'typeId'},
  ],
};

/// Descriptor for `ListenEventTypeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listenEventTypeRequestDescriptor = $convert.base64Decode('ChZMaXN0ZW5FdmVudFR5cGVSZXF1ZXN0Eh8KC2xpc3RlbmVyX2lkGAIgASgJUgpsaXN0ZW5lcklkEhcKB3R5cGVfaWQYASABKAlSBnR5cGVJZA==');
@$core.Deprecated('Use listenEventTypeResponseDescriptor instead')
const ListenEventTypeResponse$json = const {
  '1': 'ListenEventTypeResponse',
  '2': const [
    const {'1': 'event', '3': 1, '4': 1, '5': 11, '6': '.event.cashmere.Event', '10': 'event'},
  ],
};

/// Descriptor for `ListenEventTypeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listenEventTypeResponseDescriptor = $convert.base64Decode('ChdMaXN0ZW5FdmVudFR5cGVSZXNwb25zZRIrCgVldmVudBgBIAEoCzIVLmV2ZW50LmNhc2htZXJlLkV2ZW50UgVldmVudA==');
