///
//  Generated code. Do not modify.
//  source: event_server.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use eventSystemConfigsDescriptor instead')
const EventSystemConfigs$json = const {
  '1': 'EventSystemConfigs',
  '2': const [
    const {'1': 'max_event_queue_length', '3': 1, '4': 1, '5': 4, '10': 'maxEventQueueLength'},
  ],
};

/// Descriptor for `EventSystemConfigs`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List eventSystemConfigsDescriptor = $convert.base64Decode('ChJFdmVudFN5c3RlbUNvbmZpZ3MSMwoWbWF4X2V2ZW50X3F1ZXVlX2xlbmd0aBgBIAEoBFITbWF4RXZlbnRRdWV1ZUxlbmd0aA==');
@$core.Deprecated('Use getEventSystemConfigsRequestDescriptor instead')
const GetEventSystemConfigsRequest$json = const {
  '1': 'GetEventSystemConfigsRequest',
};

/// Descriptor for `GetEventSystemConfigsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEventSystemConfigsRequestDescriptor = $convert.base64Decode('ChxHZXRFdmVudFN5c3RlbUNvbmZpZ3NSZXF1ZXN0');
@$core.Deprecated('Use getEventSystemConfigsResponseDescriptor instead')
const GetEventSystemConfigsResponse$json = const {
  '1': 'GetEventSystemConfigsResponse',
  '2': const [
    const {'1': 'configs', '3': 1, '4': 1, '5': 11, '6': '.event.cashmere.EventSystemConfigs', '10': 'configs'},
  ],
};

/// Descriptor for `GetEventSystemConfigsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEventSystemConfigsResponseDescriptor = $convert.base64Decode('Ch1HZXRFdmVudFN5c3RlbUNvbmZpZ3NSZXNwb25zZRI8Cgdjb25maWdzGAEgASgLMiIuZXZlbnQuY2FzaG1lcmUuRXZlbnRTeXN0ZW1Db25maWdzUgdjb25maWdz');
@$core.Deprecated('Use registerEventTypeRequestDescriptor instead')
const RegisterEventTypeRequest$json = const {
  '1': 'RegisterEventTypeRequest',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    const {'1': 'has_echo', '3': 2, '4': 1, '5': 8, '10': 'hasEcho'},
    const {'1': 'description', '3': 3, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `RegisterEventTypeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registerEventTypeRequestDescriptor = $convert.base64Decode('ChhSZWdpc3RlckV2ZW50VHlwZVJlcXVlc3QSIgoEbmFtZRgBIAEoCzIOLmNhc2htZXJlLk5hbWVSBG5hbWUSGQoIaGFzX2VjaG8YAiABKAhSB2hhc0VjaG8SIAoLZGVzY3JpcHRpb24YAyABKAlSC2Rlc2NyaXB0aW9u');
@$core.Deprecated('Use registerEventTypeResponseDescriptor instead')
const RegisterEventTypeResponse$json = const {
  '1': 'RegisterEventTypeResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RegisterEventTypeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registerEventTypeResponseDescriptor = $convert.base64Decode('ChlSZWdpc3RlckV2ZW50VHlwZVJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use deregisterEventTypeRequestDescriptor instead')
const DeregisterEventTypeRequest$json = const {
  '1': 'DeregisterEventTypeRequest',
  '2': const [
    const {'1': 'type_id', '3': 1, '4': 1, '5': 9, '10': 'typeId'},
  ],
};

/// Descriptor for `DeregisterEventTypeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deregisterEventTypeRequestDescriptor = $convert.base64Decode('ChpEZXJlZ2lzdGVyRXZlbnRUeXBlUmVxdWVzdBIXCgd0eXBlX2lkGAEgASgJUgZ0eXBlSWQ=');
@$core.Deprecated('Use deregisterEventTypeResponseDescriptor instead')
const DeregisterEventTypeResponse$json = const {
  '1': 'DeregisterEventTypeResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `DeregisterEventTypeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deregisterEventTypeResponseDescriptor = $convert.base64Decode('ChtEZXJlZ2lzdGVyRXZlbnRUeXBlUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use registerEventEmitterRequestDescriptor instead')
const RegisterEventEmitterRequest$json = const {
  '1': 'RegisterEventEmitterRequest',
  '2': const [
    const {'1': 'event_type', '3': 1, '4': 1, '5': 9, '10': 'eventType'},
    const {'1': 'name', '3': 2, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    const {'1': 'description', '3': 3, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `RegisterEventEmitterRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registerEventEmitterRequestDescriptor = $convert.base64Decode('ChtSZWdpc3RlckV2ZW50RW1pdHRlclJlcXVlc3QSHQoKZXZlbnRfdHlwZRgBIAEoCVIJZXZlbnRUeXBlEiIKBG5hbWUYAiABKAsyDi5jYXNobWVyZS5OYW1lUgRuYW1lEiAKC2Rlc2NyaXB0aW9uGAMgASgJUgtkZXNjcmlwdGlvbg==');
@$core.Deprecated('Use registerEventEmitterResponseDescriptor instead')
const RegisterEventEmitterResponse$json = const {
  '1': 'RegisterEventEmitterResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RegisterEventEmitterResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registerEventEmitterResponseDescriptor = $convert.base64Decode('ChxSZWdpc3RlckV2ZW50RW1pdHRlclJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use deregisterEventEmitterRequestDescriptor instead')
const DeregisterEventEmitterRequest$json = const {
  '1': 'DeregisterEventEmitterRequest',
  '2': const [
    const {'1': 'emitter_id', '3': 1, '4': 1, '5': 9, '10': 'emitterId'},
  ],
};

/// Descriptor for `DeregisterEventEmitterRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deregisterEventEmitterRequestDescriptor = $convert.base64Decode('Ch1EZXJlZ2lzdGVyRXZlbnRFbWl0dGVyUmVxdWVzdBIdCgplbWl0dGVyX2lkGAEgASgJUgllbWl0dGVySWQ=');
@$core.Deprecated('Use deregisterEventEmitterResponseDescriptor instead')
const DeregisterEventEmitterResponse$json = const {
  '1': 'DeregisterEventEmitterResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `DeregisterEventEmitterResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deregisterEventEmitterResponseDescriptor = $convert.base64Decode('Ch5EZXJlZ2lzdGVyRXZlbnRFbWl0dGVyUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use registerEventListenerRequestDescriptor instead')
const RegisterEventListenerRequest$json = const {
  '1': 'RegisterEventListenerRequest',
  '2': const [
    const {'1': 'event_type', '3': 1, '4': 1, '5': 9, '10': 'eventType'},
    const {'1': 'name', '3': 2, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'name'},
    const {'1': 'description', '3': 3, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `RegisterEventListenerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registerEventListenerRequestDescriptor = $convert.base64Decode('ChxSZWdpc3RlckV2ZW50TGlzdGVuZXJSZXF1ZXN0Eh0KCmV2ZW50X3R5cGUYASABKAlSCWV2ZW50VHlwZRIiCgRuYW1lGAIgASgLMg4uY2FzaG1lcmUuTmFtZVIEbmFtZRIgCgtkZXNjcmlwdGlvbhgDIAEoCVILZGVzY3JpcHRpb24=');
@$core.Deprecated('Use registerEventListenerResponseDescriptor instead')
const RegisterEventListenerResponse$json = const {
  '1': 'RegisterEventListenerResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RegisterEventListenerResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registerEventListenerResponseDescriptor = $convert.base64Decode('Ch1SZWdpc3RlckV2ZW50TGlzdGVuZXJSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
@$core.Deprecated('Use deregisterEventListenerRequestDescriptor instead')
const DeregisterEventListenerRequest$json = const {
  '1': 'DeregisterEventListenerRequest',
  '2': const [
    const {'1': 'listener_id', '3': 1, '4': 1, '5': 9, '10': 'listenerId'},
  ],
};

/// Descriptor for `DeregisterEventListenerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deregisterEventListenerRequestDescriptor = $convert.base64Decode('Ch5EZXJlZ2lzdGVyRXZlbnRMaXN0ZW5lclJlcXVlc3QSHwoLbGlzdGVuZXJfaWQYASABKAlSCmxpc3RlbmVySWQ=');
@$core.Deprecated('Use deregisterEventListenerResponseDescriptor instead')
const DeregisterEventListenerResponse$json = const {
  '1': 'DeregisterEventListenerResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `DeregisterEventListenerResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List deregisterEventListenerResponseDescriptor = $convert.base64Decode('Ch9EZXJlZ2lzdGVyRXZlbnRMaXN0ZW5lclJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use listEventTypeRequestDescriptor instead')
const ListEventTypeRequest$json = const {
  '1': 'ListEventTypeRequest',
};

/// Descriptor for `ListEventTypeRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listEventTypeRequestDescriptor = $convert.base64Decode('ChRMaXN0RXZlbnRUeXBlUmVxdWVzdA==');
@$core.Deprecated('Use listEventTypeResponseDescriptor instead')
const ListEventTypeResponse$json = const {
  '1': 'ListEventTypeResponse',
  '2': const [
    const {'1': 'event_types', '3': 1, '4': 3, '5': 11, '6': '.event.cashmere.EventType', '10': 'eventTypes'},
  ],
};

/// Descriptor for `ListEventTypeResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listEventTypeResponseDescriptor = $convert.base64Decode('ChVMaXN0RXZlbnRUeXBlUmVzcG9uc2USOgoLZXZlbnRfdHlwZXMYASADKAsyGS5ldmVudC5jYXNobWVyZS5FdmVudFR5cGVSCmV2ZW50VHlwZXM=');
@$core.Deprecated('Use listEmitterRequestDescriptor instead')
const ListEmitterRequest$json = const {
  '1': 'ListEmitterRequest',
  '2': const [
    const {'1': 'type_id', '3': 1, '4': 1, '5': 9, '10': 'typeId'},
  ],
};

/// Descriptor for `ListEmitterRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listEmitterRequestDescriptor = $convert.base64Decode('ChJMaXN0RW1pdHRlclJlcXVlc3QSFwoHdHlwZV9pZBgBIAEoCVIGdHlwZUlk');
@$core.Deprecated('Use listEmitterResponseDescriptor instead')
const ListEmitterResponse$json = const {
  '1': 'ListEmitterResponse',
  '2': const [
    const {'1': 'emitters', '3': 1, '4': 3, '5': 11, '6': '.event.cashmere.EventEmitter', '10': 'emitters'},
  ],
};

/// Descriptor for `ListEmitterResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listEmitterResponseDescriptor = $convert.base64Decode('ChNMaXN0RW1pdHRlclJlc3BvbnNlEjgKCGVtaXR0ZXJzGAEgAygLMhwuZXZlbnQuY2FzaG1lcmUuRXZlbnRFbWl0dGVyUghlbWl0dGVycw==');
@$core.Deprecated('Use listListenerRequestDescriptor instead')
const ListListenerRequest$json = const {
  '1': 'ListListenerRequest',
  '2': const [
    const {'1': 'type_id', '3': 1, '4': 1, '5': 9, '10': 'typeId'},
  ],
};

/// Descriptor for `ListListenerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listListenerRequestDescriptor = $convert.base64Decode('ChNMaXN0TGlzdGVuZXJSZXF1ZXN0EhcKB3R5cGVfaWQYASABKAlSBnR5cGVJZA==');
@$core.Deprecated('Use listListenerResponseDescriptor instead')
const ListListenerResponse$json = const {
  '1': 'ListListenerResponse',
  '2': const [
    const {'1': 'listeners', '3': 1, '4': 3, '5': 11, '6': '.event.cashmere.EventListener', '10': 'listeners'},
  ],
};

/// Descriptor for `ListListenerResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listListenerResponseDescriptor = $convert.base64Decode('ChRMaXN0TGlzdGVuZXJSZXNwb25zZRI7CglsaXN0ZW5lcnMYASADKAsyHS5ldmVudC5jYXNobWVyZS5FdmVudExpc3RlbmVyUglsaXN0ZW5lcnM=');
@$core.Deprecated('Use getEventEmitterRequestDescriptor instead')
const GetEventEmitterRequest$json = const {
  '1': 'GetEventEmitterRequest',
  '2': const [
    const {'1': 'emitter_id', '3': 1, '4': 1, '5': 9, '10': 'emitterId'},
  ],
};

/// Descriptor for `GetEventEmitterRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEventEmitterRequestDescriptor = $convert.base64Decode('ChZHZXRFdmVudEVtaXR0ZXJSZXF1ZXN0Eh0KCmVtaXR0ZXJfaWQYASABKAlSCWVtaXR0ZXJJZA==');
@$core.Deprecated('Use getEventEmitterResponseDescriptor instead')
const GetEventEmitterResponse$json = const {
  '1': 'GetEventEmitterResponse',
  '2': const [
    const {'1': 'emitter', '3': 1, '4': 1, '5': 11, '6': '.event.cashmere.EventEmitter', '10': 'emitter'},
  ],
};

/// Descriptor for `GetEventEmitterResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEventEmitterResponseDescriptor = $convert.base64Decode('ChdHZXRFdmVudEVtaXR0ZXJSZXNwb25zZRI2CgdlbWl0dGVyGAEgASgLMhwuZXZlbnQuY2FzaG1lcmUuRXZlbnRFbWl0dGVyUgdlbWl0dGVy');
@$core.Deprecated('Use getEventListenerRequestDescriptor instead')
const GetEventListenerRequest$json = const {
  '1': 'GetEventListenerRequest',
  '2': const [
    const {'1': 'listener_id', '3': 1, '4': 1, '5': 9, '10': 'listenerId'},
  ],
};

/// Descriptor for `GetEventListenerRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEventListenerRequestDescriptor = $convert.base64Decode('ChdHZXRFdmVudExpc3RlbmVyUmVxdWVzdBIfCgtsaXN0ZW5lcl9pZBgBIAEoCVIKbGlzdGVuZXJJZA==');
@$core.Deprecated('Use getEventListenerResponseDescriptor instead')
const GetEventListenerResponse$json = const {
  '1': 'GetEventListenerResponse',
  '2': const [
    const {'1': 'listener', '3': 1, '4': 1, '5': 11, '6': '.event.cashmere.EventListener', '10': 'listener'},
  ],
};

/// Descriptor for `GetEventListenerResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getEventListenerResponseDescriptor = $convert.base64Decode('ChhHZXRFdmVudExpc3RlbmVyUmVzcG9uc2USOQoIbGlzdGVuZXIYASABKAsyHS5ldmVudC5jYXNobWVyZS5FdmVudExpc3RlbmVyUghsaXN0ZW5lcg==');
