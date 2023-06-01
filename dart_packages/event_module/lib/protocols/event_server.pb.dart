///
//  Generated code. Do not modify.
//  source: event_server.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'package:cashmere_core/protocols/name.pb.dart' as $0;
import 'event.pb.dart' as $1;

class EventSystemConfigs extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'EventSystemConfigs',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..a<$fixnum.Int64>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'maxEventQueueLength',
        $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  EventSystemConfigs._() : super();
  factory EventSystemConfigs({
    $fixnum.Int64? maxEventQueueLength,
  }) {
    final _result = create();
    if (maxEventQueueLength != null) {
      _result.maxEventQueueLength = maxEventQueueLength;
    }
    return _result;
  }
  factory EventSystemConfigs.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory EventSystemConfigs.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  EventSystemConfigs clone() => EventSystemConfigs()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  EventSystemConfigs copyWith(void Function(EventSystemConfigs) updates) =>
      super.copyWith((message) => updates(message as EventSystemConfigs))
          as EventSystemConfigs; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static EventSystemConfigs create() => EventSystemConfigs._();
  EventSystemConfigs createEmptyInstance() => create();
  static $pb.PbList<EventSystemConfigs> createRepeated() =>
      $pb.PbList<EventSystemConfigs>();
  @$core.pragma('dart2js:noInline')
  static EventSystemConfigs getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<EventSystemConfigs>(create);
  static EventSystemConfigs? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get maxEventQueueLength => $_getI64(0);
  @$pb.TagNumber(1)
  set maxEventQueueLength($fixnum.Int64 v) {
    $_setInt64(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasMaxEventQueueLength() => $_has(0);
  @$pb.TagNumber(1)
  void clearMaxEventQueueLength() => clearField(1);
}

class GetEventSystemConfigsRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'GetEventSystemConfigsRequest',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  GetEventSystemConfigsRequest._() : super();
  factory GetEventSystemConfigsRequest() => create();
  factory GetEventSystemConfigsRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory GetEventSystemConfigsRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  GetEventSystemConfigsRequest clone() =>
      GetEventSystemConfigsRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  GetEventSystemConfigsRequest copyWith(
          void Function(GetEventSystemConfigsRequest) updates) =>
      super.copyWith(
              (message) => updates(message as GetEventSystemConfigsRequest))
          as GetEventSystemConfigsRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetEventSystemConfigsRequest create() =>
      GetEventSystemConfigsRequest._();
  GetEventSystemConfigsRequest createEmptyInstance() => create();
  static $pb.PbList<GetEventSystemConfigsRequest> createRepeated() =>
      $pb.PbList<GetEventSystemConfigsRequest>();
  @$core.pragma('dart2js:noInline')
  static GetEventSystemConfigsRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetEventSystemConfigsRequest>(create);
  static GetEventSystemConfigsRequest? _defaultInstance;
}

class GetEventSystemConfigsResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'GetEventSystemConfigsResponse',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOM<EventSystemConfigs>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'configs',
        subBuilder: EventSystemConfigs.create)
    ..hasRequiredFields = false;

  GetEventSystemConfigsResponse._() : super();
  factory GetEventSystemConfigsResponse({
    EventSystemConfigs? configs,
  }) {
    final _result = create();
    if (configs != null) {
      _result.configs = configs;
    }
    return _result;
  }
  factory GetEventSystemConfigsResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory GetEventSystemConfigsResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  GetEventSystemConfigsResponse clone() =>
      GetEventSystemConfigsResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  GetEventSystemConfigsResponse copyWith(
          void Function(GetEventSystemConfigsResponse) updates) =>
      super.copyWith(
              (message) => updates(message as GetEventSystemConfigsResponse))
          as GetEventSystemConfigsResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetEventSystemConfigsResponse create() =>
      GetEventSystemConfigsResponse._();
  GetEventSystemConfigsResponse createEmptyInstance() => create();
  static $pb.PbList<GetEventSystemConfigsResponse> createRepeated() =>
      $pb.PbList<GetEventSystemConfigsResponse>();
  @$core.pragma('dart2js:noInline')
  static GetEventSystemConfigsResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetEventSystemConfigsResponse>(create);
  static GetEventSystemConfigsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  EventSystemConfigs get configs => $_getN(0);
  @$pb.TagNumber(1)
  set configs(EventSystemConfigs v) {
    setField(1, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasConfigs() => $_has(0);
  @$pb.TagNumber(1)
  void clearConfigs() => clearField(1);
  @$pb.TagNumber(1)
  EventSystemConfigs ensureConfigs() => $_ensure(0);
}

class RegisterEventTypeRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'RegisterEventTypeRequest',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOM<$0.Name>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'name',
        subBuilder: $0.Name.create)
    ..aOB(
        2,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'hasEcho')
    ..aOS(
        3,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'description')
    ..hasRequiredFields = false;

  RegisterEventTypeRequest._() : super();
  factory RegisterEventTypeRequest({
    $0.Name? name,
    $core.bool? hasEcho,
    $core.String? description,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (hasEcho != null) {
      _result.hasEcho = hasEcho;
    }
    if (description != null) {
      _result.description = description;
    }
    return _result;
  }
  factory RegisterEventTypeRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory RegisterEventTypeRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  RegisterEventTypeRequest clone() =>
      RegisterEventTypeRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  RegisterEventTypeRequest copyWith(
          void Function(RegisterEventTypeRequest) updates) =>
      super.copyWith((message) => updates(message as RegisterEventTypeRequest))
          as RegisterEventTypeRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RegisterEventTypeRequest create() => RegisterEventTypeRequest._();
  RegisterEventTypeRequest createEmptyInstance() => create();
  static $pb.PbList<RegisterEventTypeRequest> createRepeated() =>
      $pb.PbList<RegisterEventTypeRequest>();
  @$core.pragma('dart2js:noInline')
  static RegisterEventTypeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RegisterEventTypeRequest>(create);
  static RegisterEventTypeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $0.Name get name => $_getN(0);
  @$pb.TagNumber(1)
  set name($0.Name v) {
    setField(1, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => clearField(1);
  @$pb.TagNumber(1)
  $0.Name ensureName() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.bool get hasEcho => $_getBF(1);
  @$pb.TagNumber(2)
  set hasEcho($core.bool v) {
    $_setBool(1, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasHasEcho() => $_has(1);
  @$pb.TagNumber(2)
  void clearHasEcho() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String v) {
    $_setString(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => clearField(3);
}

class RegisterEventTypeResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'RegisterEventTypeResponse',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'result')
    ..hasRequiredFields = false;

  RegisterEventTypeResponse._() : super();
  factory RegisterEventTypeResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory RegisterEventTypeResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory RegisterEventTypeResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  RegisterEventTypeResponse clone() =>
      RegisterEventTypeResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  RegisterEventTypeResponse copyWith(
          void Function(RegisterEventTypeResponse) updates) =>
      super.copyWith((message) => updates(message as RegisterEventTypeResponse))
          as RegisterEventTypeResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RegisterEventTypeResponse create() => RegisterEventTypeResponse._();
  RegisterEventTypeResponse createEmptyInstance() => create();
  static $pb.PbList<RegisterEventTypeResponse> createRepeated() =>
      $pb.PbList<RegisterEventTypeResponse>();
  @$core.pragma('dart2js:noInline')
  static RegisterEventTypeResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RegisterEventTypeResponse>(create);
  static RegisterEventTypeResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class DeregisterEventTypeRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'DeregisterEventTypeRequest',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'typeId')
    ..hasRequiredFields = false;

  DeregisterEventTypeRequest._() : super();
  factory DeregisterEventTypeRequest({
    $core.String? typeId,
  }) {
    final _result = create();
    if (typeId != null) {
      _result.typeId = typeId;
    }
    return _result;
  }
  factory DeregisterEventTypeRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory DeregisterEventTypeRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  DeregisterEventTypeRequest clone() =>
      DeregisterEventTypeRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  DeregisterEventTypeRequest copyWith(
          void Function(DeregisterEventTypeRequest) updates) =>
      super.copyWith(
              (message) => updates(message as DeregisterEventTypeRequest))
          as DeregisterEventTypeRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DeregisterEventTypeRequest create() => DeregisterEventTypeRequest._();
  DeregisterEventTypeRequest createEmptyInstance() => create();
  static $pb.PbList<DeregisterEventTypeRequest> createRepeated() =>
      $pb.PbList<DeregisterEventTypeRequest>();
  @$core.pragma('dart2js:noInline')
  static DeregisterEventTypeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeregisterEventTypeRequest>(create);
  static DeregisterEventTypeRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get typeId => $_getSZ(0);
  @$pb.TagNumber(1)
  set typeId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasTypeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTypeId() => clearField(1);
}

class DeregisterEventTypeResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'DeregisterEventTypeResponse',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'result')
    ..hasRequiredFields = false;

  DeregisterEventTypeResponse._() : super();
  factory DeregisterEventTypeResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory DeregisterEventTypeResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory DeregisterEventTypeResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  DeregisterEventTypeResponse clone() =>
      DeregisterEventTypeResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  DeregisterEventTypeResponse copyWith(
          void Function(DeregisterEventTypeResponse) updates) =>
      super.copyWith(
              (message) => updates(message as DeregisterEventTypeResponse))
          as DeregisterEventTypeResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DeregisterEventTypeResponse create() =>
      DeregisterEventTypeResponse._();
  DeregisterEventTypeResponse createEmptyInstance() => create();
  static $pb.PbList<DeregisterEventTypeResponse> createRepeated() =>
      $pb.PbList<DeregisterEventTypeResponse>();
  @$core.pragma('dart2js:noInline')
  static DeregisterEventTypeResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeregisterEventTypeResponse>(create);
  static DeregisterEventTypeResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class RegisterEventEmitterRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'RegisterEventEmitterRequest',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'eventType')
    ..aOM<$0.Name>(
        2,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'name',
        subBuilder: $0.Name.create)
    ..aOS(
        3,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'description')
    ..hasRequiredFields = false;

  RegisterEventEmitterRequest._() : super();
  factory RegisterEventEmitterRequest({
    $core.String? eventType,
    $0.Name? name,
    $core.String? description,
  }) {
    final _result = create();
    if (eventType != null) {
      _result.eventType = eventType;
    }
    if (name != null) {
      _result.name = name;
    }
    if (description != null) {
      _result.description = description;
    }
    return _result;
  }
  factory RegisterEventEmitterRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory RegisterEventEmitterRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  RegisterEventEmitterRequest clone() =>
      RegisterEventEmitterRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  RegisterEventEmitterRequest copyWith(
          void Function(RegisterEventEmitterRequest) updates) =>
      super.copyWith(
              (message) => updates(message as RegisterEventEmitterRequest))
          as RegisterEventEmitterRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RegisterEventEmitterRequest create() =>
      RegisterEventEmitterRequest._();
  RegisterEventEmitterRequest createEmptyInstance() => create();
  static $pb.PbList<RegisterEventEmitterRequest> createRepeated() =>
      $pb.PbList<RegisterEventEmitterRequest>();
  @$core.pragma('dart2js:noInline')
  static RegisterEventEmitterRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RegisterEventEmitterRequest>(create);
  static RegisterEventEmitterRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get eventType => $_getSZ(0);
  @$pb.TagNumber(1)
  set eventType($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasEventType() => $_has(0);
  @$pb.TagNumber(1)
  void clearEventType() => clearField(1);

  @$pb.TagNumber(2)
  $0.Name get name => $_getN(1);
  @$pb.TagNumber(2)
  set name($0.Name v) {
    setField(2, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);
  @$pb.TagNumber(2)
  $0.Name ensureName() => $_ensure(1);

  @$pb.TagNumber(3)
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String v) {
    $_setString(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => clearField(3);
}

class RegisterEventEmitterResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'RegisterEventEmitterResponse',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'result')
    ..hasRequiredFields = false;

  RegisterEventEmitterResponse._() : super();
  factory RegisterEventEmitterResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory RegisterEventEmitterResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory RegisterEventEmitterResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  RegisterEventEmitterResponse clone() =>
      RegisterEventEmitterResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  RegisterEventEmitterResponse copyWith(
          void Function(RegisterEventEmitterResponse) updates) =>
      super.copyWith(
              (message) => updates(message as RegisterEventEmitterResponse))
          as RegisterEventEmitterResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RegisterEventEmitterResponse create() =>
      RegisterEventEmitterResponse._();
  RegisterEventEmitterResponse createEmptyInstance() => create();
  static $pb.PbList<RegisterEventEmitterResponse> createRepeated() =>
      $pb.PbList<RegisterEventEmitterResponse>();
  @$core.pragma('dart2js:noInline')
  static RegisterEventEmitterResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RegisterEventEmitterResponse>(create);
  static RegisterEventEmitterResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class DeregisterEventEmitterRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'DeregisterEventEmitterRequest',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'emitterId')
    ..hasRequiredFields = false;

  DeregisterEventEmitterRequest._() : super();
  factory DeregisterEventEmitterRequest({
    $core.String? emitterId,
  }) {
    final _result = create();
    if (emitterId != null) {
      _result.emitterId = emitterId;
    }
    return _result;
  }
  factory DeregisterEventEmitterRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory DeregisterEventEmitterRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  DeregisterEventEmitterRequest clone() =>
      DeregisterEventEmitterRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  DeregisterEventEmitterRequest copyWith(
          void Function(DeregisterEventEmitterRequest) updates) =>
      super.copyWith(
              (message) => updates(message as DeregisterEventEmitterRequest))
          as DeregisterEventEmitterRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DeregisterEventEmitterRequest create() =>
      DeregisterEventEmitterRequest._();
  DeregisterEventEmitterRequest createEmptyInstance() => create();
  static $pb.PbList<DeregisterEventEmitterRequest> createRepeated() =>
      $pb.PbList<DeregisterEventEmitterRequest>();
  @$core.pragma('dart2js:noInline')
  static DeregisterEventEmitterRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeregisterEventEmitterRequest>(create);
  static DeregisterEventEmitterRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get emitterId => $_getSZ(0);
  @$pb.TagNumber(1)
  set emitterId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasEmitterId() => $_has(0);
  @$pb.TagNumber(1)
  void clearEmitterId() => clearField(1);
}

class DeregisterEventEmitterResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'DeregisterEventEmitterResponse',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'result')
    ..hasRequiredFields = false;

  DeregisterEventEmitterResponse._() : super();
  factory DeregisterEventEmitterResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory DeregisterEventEmitterResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory DeregisterEventEmitterResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  DeregisterEventEmitterResponse clone() =>
      DeregisterEventEmitterResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  DeregisterEventEmitterResponse copyWith(
          void Function(DeregisterEventEmitterResponse) updates) =>
      super.copyWith(
              (message) => updates(message as DeregisterEventEmitterResponse))
          as DeregisterEventEmitterResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DeregisterEventEmitterResponse create() =>
      DeregisterEventEmitterResponse._();
  DeregisterEventEmitterResponse createEmptyInstance() => create();
  static $pb.PbList<DeregisterEventEmitterResponse> createRepeated() =>
      $pb.PbList<DeregisterEventEmitterResponse>();
  @$core.pragma('dart2js:noInline')
  static DeregisterEventEmitterResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeregisterEventEmitterResponse>(create);
  static DeregisterEventEmitterResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class RegisterEventListenerRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'RegisterEventListenerRequest',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'eventType')
    ..aOM<$0.Name>(
        2,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'name',
        subBuilder: $0.Name.create)
    ..aOS(
        3,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'description')
    ..hasRequiredFields = false;

  RegisterEventListenerRequest._() : super();
  factory RegisterEventListenerRequest({
    $core.String? eventType,
    $0.Name? name,
    $core.String? description,
  }) {
    final _result = create();
    if (eventType != null) {
      _result.eventType = eventType;
    }
    if (name != null) {
      _result.name = name;
    }
    if (description != null) {
      _result.description = description;
    }
    return _result;
  }
  factory RegisterEventListenerRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory RegisterEventListenerRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  RegisterEventListenerRequest clone() =>
      RegisterEventListenerRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  RegisterEventListenerRequest copyWith(
          void Function(RegisterEventListenerRequest) updates) =>
      super.copyWith(
              (message) => updates(message as RegisterEventListenerRequest))
          as RegisterEventListenerRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RegisterEventListenerRequest create() =>
      RegisterEventListenerRequest._();
  RegisterEventListenerRequest createEmptyInstance() => create();
  static $pb.PbList<RegisterEventListenerRequest> createRepeated() =>
      $pb.PbList<RegisterEventListenerRequest>();
  @$core.pragma('dart2js:noInline')
  static RegisterEventListenerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RegisterEventListenerRequest>(create);
  static RegisterEventListenerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get eventType => $_getSZ(0);
  @$pb.TagNumber(1)
  set eventType($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasEventType() => $_has(0);
  @$pb.TagNumber(1)
  void clearEventType() => clearField(1);

  @$pb.TagNumber(2)
  $0.Name get name => $_getN(1);
  @$pb.TagNumber(2)
  set name($0.Name v) {
    setField(2, v);
  }

  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);
  @$pb.TagNumber(2)
  $0.Name ensureName() => $_ensure(1);

  @$pb.TagNumber(3)
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String v) {
    $_setString(2, v);
  }

  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => clearField(3);
}

class RegisterEventListenerResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'RegisterEventListenerResponse',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'result')
    ..hasRequiredFields = false;

  RegisterEventListenerResponse._() : super();
  factory RegisterEventListenerResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory RegisterEventListenerResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory RegisterEventListenerResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  RegisterEventListenerResponse clone() =>
      RegisterEventListenerResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  RegisterEventListenerResponse copyWith(
          void Function(RegisterEventListenerResponse) updates) =>
      super.copyWith(
              (message) => updates(message as RegisterEventListenerResponse))
          as RegisterEventListenerResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RegisterEventListenerResponse create() =>
      RegisterEventListenerResponse._();
  RegisterEventListenerResponse createEmptyInstance() => create();
  static $pb.PbList<RegisterEventListenerResponse> createRepeated() =>
      $pb.PbList<RegisterEventListenerResponse>();
  @$core.pragma('dart2js:noInline')
  static RegisterEventListenerResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<RegisterEventListenerResponse>(create);
  static RegisterEventListenerResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class DeregisterEventListenerRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'DeregisterEventListenerRequest',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'listenerId')
    ..hasRequiredFields = false;

  DeregisterEventListenerRequest._() : super();
  factory DeregisterEventListenerRequest({
    $core.String? listenerId,
  }) {
    final _result = create();
    if (listenerId != null) {
      _result.listenerId = listenerId;
    }
    return _result;
  }
  factory DeregisterEventListenerRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory DeregisterEventListenerRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  DeregisterEventListenerRequest clone() =>
      DeregisterEventListenerRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  DeregisterEventListenerRequest copyWith(
          void Function(DeregisterEventListenerRequest) updates) =>
      super.copyWith(
              (message) => updates(message as DeregisterEventListenerRequest))
          as DeregisterEventListenerRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DeregisterEventListenerRequest create() =>
      DeregisterEventListenerRequest._();
  DeregisterEventListenerRequest createEmptyInstance() => create();
  static $pb.PbList<DeregisterEventListenerRequest> createRepeated() =>
      $pb.PbList<DeregisterEventListenerRequest>();
  @$core.pragma('dart2js:noInline')
  static DeregisterEventListenerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeregisterEventListenerRequest>(create);
  static DeregisterEventListenerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get listenerId => $_getSZ(0);
  @$pb.TagNumber(1)
  set listenerId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasListenerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearListenerId() => clearField(1);
}

class DeregisterEventListenerResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'DeregisterEventListenerResponse',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'result')
    ..hasRequiredFields = false;

  DeregisterEventListenerResponse._() : super();
  factory DeregisterEventListenerResponse({
    $core.String? result,
  }) {
    final _result = create();
    if (result != null) {
      _result.result = result;
    }
    return _result;
  }
  factory DeregisterEventListenerResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory DeregisterEventListenerResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  DeregisterEventListenerResponse clone() =>
      DeregisterEventListenerResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  DeregisterEventListenerResponse copyWith(
          void Function(DeregisterEventListenerResponse) updates) =>
      super.copyWith(
              (message) => updates(message as DeregisterEventListenerResponse))
          as DeregisterEventListenerResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static DeregisterEventListenerResponse create() =>
      DeregisterEventListenerResponse._();
  DeregisterEventListenerResponse createEmptyInstance() => create();
  static $pb.PbList<DeregisterEventListenerResponse> createRepeated() =>
      $pb.PbList<DeregisterEventListenerResponse>();
  @$core.pragma('dart2js:noInline')
  static DeregisterEventListenerResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<DeregisterEventListenerResponse>(
          create);
  static DeregisterEventListenerResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get result => $_getSZ(0);
  @$pb.TagNumber(1)
  set result($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasResult() => $_has(0);
  @$pb.TagNumber(1)
  void clearResult() => clearField(1);
}

class ListEventTypeRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'ListEventTypeRequest',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  ListEventTypeRequest._() : super();
  factory ListEventTypeRequest() => create();
  factory ListEventTypeRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory ListEventTypeRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  ListEventTypeRequest clone() =>
      ListEventTypeRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  ListEventTypeRequest copyWith(void Function(ListEventTypeRequest) updates) =>
      super.copyWith((message) => updates(message as ListEventTypeRequest))
          as ListEventTypeRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListEventTypeRequest create() => ListEventTypeRequest._();
  ListEventTypeRequest createEmptyInstance() => create();
  static $pb.PbList<ListEventTypeRequest> createRepeated() =>
      $pb.PbList<ListEventTypeRequest>();
  @$core.pragma('dart2js:noInline')
  static ListEventTypeRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListEventTypeRequest>(create);
  static ListEventTypeRequest? _defaultInstance;
}

class ListEventTypeResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'ListEventTypeResponse',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..pc<$1.EventType>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'eventTypes',
        $pb.PbFieldType.PM,
        subBuilder: $1.EventType.create)
    ..hasRequiredFields = false;

  ListEventTypeResponse._() : super();
  factory ListEventTypeResponse({
    $core.Iterable<$1.EventType>? eventTypes,
  }) {
    final _result = create();
    if (eventTypes != null) {
      _result.eventTypes.addAll(eventTypes);
    }
    return _result;
  }
  factory ListEventTypeResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory ListEventTypeResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  ListEventTypeResponse clone() =>
      ListEventTypeResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  ListEventTypeResponse copyWith(
          void Function(ListEventTypeResponse) updates) =>
      super.copyWith((message) => updates(message as ListEventTypeResponse))
          as ListEventTypeResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListEventTypeResponse create() => ListEventTypeResponse._();
  ListEventTypeResponse createEmptyInstance() => create();
  static $pb.PbList<ListEventTypeResponse> createRepeated() =>
      $pb.PbList<ListEventTypeResponse>();
  @$core.pragma('dart2js:noInline')
  static ListEventTypeResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListEventTypeResponse>(create);
  static ListEventTypeResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$1.EventType> get eventTypes => $_getList(0);
}

class ListEmitterRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'ListEmitterRequest',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'typeId')
    ..hasRequiredFields = false;

  ListEmitterRequest._() : super();
  factory ListEmitterRequest({
    $core.String? typeId,
  }) {
    final _result = create();
    if (typeId != null) {
      _result.typeId = typeId;
    }
    return _result;
  }
  factory ListEmitterRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory ListEmitterRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  ListEmitterRequest clone() => ListEmitterRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  ListEmitterRequest copyWith(void Function(ListEmitterRequest) updates) =>
      super.copyWith((message) => updates(message as ListEmitterRequest))
          as ListEmitterRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListEmitterRequest create() => ListEmitterRequest._();
  ListEmitterRequest createEmptyInstance() => create();
  static $pb.PbList<ListEmitterRequest> createRepeated() =>
      $pb.PbList<ListEmitterRequest>();
  @$core.pragma('dart2js:noInline')
  static ListEmitterRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListEmitterRequest>(create);
  static ListEmitterRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get typeId => $_getSZ(0);
  @$pb.TagNumber(1)
  set typeId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasTypeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTypeId() => clearField(1);
}

class ListEmitterResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'ListEmitterResponse',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..pc<$1.EventEmitter>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'emitters',
        $pb.PbFieldType.PM,
        subBuilder: $1.EventEmitter.create)
    ..hasRequiredFields = false;

  ListEmitterResponse._() : super();
  factory ListEmitterResponse({
    $core.Iterable<$1.EventEmitter>? emitters,
  }) {
    final _result = create();
    if (emitters != null) {
      _result.emitters.addAll(emitters);
    }
    return _result;
  }
  factory ListEmitterResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory ListEmitterResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  ListEmitterResponse clone() => ListEmitterResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  ListEmitterResponse copyWith(void Function(ListEmitterResponse) updates) =>
      super.copyWith((message) => updates(message as ListEmitterResponse))
          as ListEmitterResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListEmitterResponse create() => ListEmitterResponse._();
  ListEmitterResponse createEmptyInstance() => create();
  static $pb.PbList<ListEmitterResponse> createRepeated() =>
      $pb.PbList<ListEmitterResponse>();
  @$core.pragma('dart2js:noInline')
  static ListEmitterResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListEmitterResponse>(create);
  static ListEmitterResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$1.EventEmitter> get emitters => $_getList(0);
}

class ListListenerRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'ListListenerRequest',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'typeId')
    ..hasRequiredFields = false;

  ListListenerRequest._() : super();
  factory ListListenerRequest({
    $core.String? typeId,
  }) {
    final _result = create();
    if (typeId != null) {
      _result.typeId = typeId;
    }
    return _result;
  }
  factory ListListenerRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory ListListenerRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  ListListenerRequest clone() => ListListenerRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  ListListenerRequest copyWith(void Function(ListListenerRequest) updates) =>
      super.copyWith((message) => updates(message as ListListenerRequest))
          as ListListenerRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListListenerRequest create() => ListListenerRequest._();
  ListListenerRequest createEmptyInstance() => create();
  static $pb.PbList<ListListenerRequest> createRepeated() =>
      $pb.PbList<ListListenerRequest>();
  @$core.pragma('dart2js:noInline')
  static ListListenerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListListenerRequest>(create);
  static ListListenerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get typeId => $_getSZ(0);
  @$pb.TagNumber(1)
  set typeId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasTypeId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTypeId() => clearField(1);
}

class ListListenerResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'ListListenerResponse',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..pc<$1.EventListener>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'listeners',
        $pb.PbFieldType.PM,
        subBuilder: $1.EventListener.create)
    ..hasRequiredFields = false;

  ListListenerResponse._() : super();
  factory ListListenerResponse({
    $core.Iterable<$1.EventListener>? listeners,
  }) {
    final _result = create();
    if (listeners != null) {
      _result.listeners.addAll(listeners);
    }
    return _result;
  }
  factory ListListenerResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory ListListenerResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  ListListenerResponse clone() =>
      ListListenerResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  ListListenerResponse copyWith(void Function(ListListenerResponse) updates) =>
      super.copyWith((message) => updates(message as ListListenerResponse))
          as ListListenerResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListListenerResponse create() => ListListenerResponse._();
  ListListenerResponse createEmptyInstance() => create();
  static $pb.PbList<ListListenerResponse> createRepeated() =>
      $pb.PbList<ListListenerResponse>();
  @$core.pragma('dart2js:noInline')
  static ListListenerResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ListListenerResponse>(create);
  static ListListenerResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$1.EventListener> get listeners => $_getList(0);
}

class GetEventEmitterRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'GetEventEmitterRequest',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'emitterId')
    ..hasRequiredFields = false;

  GetEventEmitterRequest._() : super();
  factory GetEventEmitterRequest({
    $core.String? emitterId,
  }) {
    final _result = create();
    if (emitterId != null) {
      _result.emitterId = emitterId;
    }
    return _result;
  }
  factory GetEventEmitterRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory GetEventEmitterRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  GetEventEmitterRequest clone() =>
      GetEventEmitterRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  GetEventEmitterRequest copyWith(
          void Function(GetEventEmitterRequest) updates) =>
      super.copyWith((message) => updates(message as GetEventEmitterRequest))
          as GetEventEmitterRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetEventEmitterRequest create() => GetEventEmitterRequest._();
  GetEventEmitterRequest createEmptyInstance() => create();
  static $pb.PbList<GetEventEmitterRequest> createRepeated() =>
      $pb.PbList<GetEventEmitterRequest>();
  @$core.pragma('dart2js:noInline')
  static GetEventEmitterRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetEventEmitterRequest>(create);
  static GetEventEmitterRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get emitterId => $_getSZ(0);
  @$pb.TagNumber(1)
  set emitterId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasEmitterId() => $_has(0);
  @$pb.TagNumber(1)
  void clearEmitterId() => clearField(1);
}

class GetEventEmitterResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'GetEventEmitterResponse',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOM<$1.EventEmitter>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'emitter',
        subBuilder: $1.EventEmitter.create)
    ..hasRequiredFields = false;

  GetEventEmitterResponse._() : super();
  factory GetEventEmitterResponse({
    $1.EventEmitter? emitter,
  }) {
    final _result = create();
    if (emitter != null) {
      _result.emitter = emitter;
    }
    return _result;
  }
  factory GetEventEmitterResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory GetEventEmitterResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  GetEventEmitterResponse clone() =>
      GetEventEmitterResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  GetEventEmitterResponse copyWith(
          void Function(GetEventEmitterResponse) updates) =>
      super.copyWith((message) => updates(message as GetEventEmitterResponse))
          as GetEventEmitterResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetEventEmitterResponse create() => GetEventEmitterResponse._();
  GetEventEmitterResponse createEmptyInstance() => create();
  static $pb.PbList<GetEventEmitterResponse> createRepeated() =>
      $pb.PbList<GetEventEmitterResponse>();
  @$core.pragma('dart2js:noInline')
  static GetEventEmitterResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetEventEmitterResponse>(create);
  static GetEventEmitterResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $1.EventEmitter get emitter => $_getN(0);
  @$pb.TagNumber(1)
  set emitter($1.EventEmitter v) {
    setField(1, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasEmitter() => $_has(0);
  @$pb.TagNumber(1)
  void clearEmitter() => clearField(1);
  @$pb.TagNumber(1)
  $1.EventEmitter ensureEmitter() => $_ensure(0);
}

class GetEventListenerRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'GetEventListenerRequest',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOS(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'listenerId')
    ..hasRequiredFields = false;

  GetEventListenerRequest._() : super();
  factory GetEventListenerRequest({
    $core.String? listenerId,
  }) {
    final _result = create();
    if (listenerId != null) {
      _result.listenerId = listenerId;
    }
    return _result;
  }
  factory GetEventListenerRequest.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory GetEventListenerRequest.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  GetEventListenerRequest clone() =>
      GetEventListenerRequest()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  GetEventListenerRequest copyWith(
          void Function(GetEventListenerRequest) updates) =>
      super.copyWith((message) => updates(message as GetEventListenerRequest))
          as GetEventListenerRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetEventListenerRequest create() => GetEventListenerRequest._();
  GetEventListenerRequest createEmptyInstance() => create();
  static $pb.PbList<GetEventListenerRequest> createRepeated() =>
      $pb.PbList<GetEventListenerRequest>();
  @$core.pragma('dart2js:noInline')
  static GetEventListenerRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetEventListenerRequest>(create);
  static GetEventListenerRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get listenerId => $_getSZ(0);
  @$pb.TagNumber(1)
  set listenerId($core.String v) {
    $_setString(0, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasListenerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearListenerId() => clearField(1);
}

class GetEventListenerResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      const $core.bool.fromEnvironment('protobuf.omit_message_names')
          ? ''
          : 'GetEventListenerResponse',
      package: const $pb.PackageName(
          const $core.bool.fromEnvironment('protobuf.omit_message_names')
              ? ''
              : 'event.cashmere'),
      createEmptyInstance: create)
    ..aOM<$1.EventListener>(
        1,
        const $core.bool.fromEnvironment('protobuf.omit_field_names')
            ? ''
            : 'listener',
        subBuilder: $1.EventListener.create)
    ..hasRequiredFields = false;

  GetEventListenerResponse._() : super();
  factory GetEventListenerResponse({
    $1.EventListener? listener,
  }) {
    final _result = create();
    if (listener != null) {
      _result.listener = listener;
    }
    return _result;
  }
  factory GetEventListenerResponse.fromBuffer($core.List<$core.int> i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(i, r);
  factory GetEventListenerResponse.fromJson($core.String i,
          [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(i, r);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
      'Will be removed in next major version')
  GetEventListenerResponse clone() =>
      GetEventListenerResponse()..mergeFromMessage(this);
  @$core.Deprecated('Using this can add significant overhead to your binary. '
      'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
      'Will be removed in next major version')
  GetEventListenerResponse copyWith(
          void Function(GetEventListenerResponse) updates) =>
      super.copyWith((message) => updates(message as GetEventListenerResponse))
          as GetEventListenerResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetEventListenerResponse create() => GetEventListenerResponse._();
  GetEventListenerResponse createEmptyInstance() => create();
  static $pb.PbList<GetEventListenerResponse> createRepeated() =>
      $pb.PbList<GetEventListenerResponse>();
  @$core.pragma('dart2js:noInline')
  static GetEventListenerResponse getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GetEventListenerResponse>(create);
  static GetEventListenerResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $1.EventListener get listener => $_getN(0);
  @$pb.TagNumber(1)
  set listener($1.EventListener v) {
    setField(1, v);
  }

  @$pb.TagNumber(1)
  $core.bool hasListener() => $_has(0);
  @$pb.TagNumber(1)
  void clearListener() => clearField(1);
  @$pb.TagNumber(1)
  $1.EventListener ensureListener() => $_ensure(0);
}
