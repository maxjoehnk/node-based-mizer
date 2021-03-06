///
//  Generated code. Do not modify.
//  source: fixtures.proto
//
// @dart = 2.7
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:async' as $async;

import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'fixtures.pb.dart' as $0;
export 'fixtures.pb.dart';

class FixturesApiClient extends $grpc.Client {
  static final _$getFixtures =
      $grpc.ClientMethod<$0.GetFixturesRequest, $0.Fixtures>(
          '/mizer.FixturesApi/GetFixtures',
          ($0.GetFixturesRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $0.Fixtures.fromBuffer(value));
  static final _$getFixtureDefinitions = $grpc.ClientMethod<
          $0.GetFixtureDefinitionsRequest, $0.FixtureDefinitions>(
      '/mizer.FixturesApi/GetFixtureDefinitions',
      ($0.GetFixtureDefinitionsRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.FixtureDefinitions.fromBuffer(value));
  static final _$addFixtures =
      $grpc.ClientMethod<$0.AddFixturesRequest, $0.Fixtures>(
          '/mizer.FixturesApi/AddFixtures',
          ($0.AddFixturesRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $0.Fixtures.fromBuffer(value));
  static final _$writeFixtureChannel =
      $grpc.ClientMethod<$0.WriteFixtureChannelRequest, $0.Fixtures>(
          '/mizer.FixturesApi/WriteFixtureChannel',
          ($0.WriteFixtureChannelRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $0.Fixtures.fromBuffer(value));

  FixturesApiClient($grpc.ClientChannel channel,
      {$grpc.CallOptions options,
      $core.Iterable<$grpc.ClientInterceptor> interceptors})
      : super(channel, options: options, interceptors: interceptors);

  $grpc.ResponseFuture<$0.Fixtures> getFixtures($0.GetFixturesRequest request,
      {$grpc.CallOptions options}) {
    return $createUnaryCall(_$getFixtures, request, options: options);
  }

  $grpc.ResponseFuture<$0.FixtureDefinitions> getFixtureDefinitions(
      $0.GetFixtureDefinitionsRequest request,
      {$grpc.CallOptions options}) {
    return $createUnaryCall(_$getFixtureDefinitions, request, options: options);
  }

  $grpc.ResponseFuture<$0.Fixtures> addFixtures($0.AddFixturesRequest request,
      {$grpc.CallOptions options}) {
    return $createUnaryCall(_$addFixtures, request, options: options);
  }

  $grpc.ResponseFuture<$0.Fixtures> writeFixtureChannel(
      $0.WriteFixtureChannelRequest request,
      {$grpc.CallOptions options}) {
    return $createUnaryCall(_$writeFixtureChannel, request, options: options);
  }
}

abstract class FixturesApiServiceBase extends $grpc.Service {
  $core.String get $name => 'mizer.FixturesApi';

  FixturesApiServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.GetFixturesRequest, $0.Fixtures>(
        'GetFixtures',
        getFixtures_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetFixturesRequest.fromBuffer(value),
        ($0.Fixtures value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.GetFixtureDefinitionsRequest,
            $0.FixtureDefinitions>(
        'GetFixtureDefinitions',
        getFixtureDefinitions_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.GetFixtureDefinitionsRequest.fromBuffer(value),
        ($0.FixtureDefinitions value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.AddFixturesRequest, $0.Fixtures>(
        'AddFixtures',
        addFixtures_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.AddFixturesRequest.fromBuffer(value),
        ($0.Fixtures value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.WriteFixtureChannelRequest, $0.Fixtures>(
        'WriteFixtureChannel',
        writeFixtureChannel_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $0.WriteFixtureChannelRequest.fromBuffer(value),
        ($0.Fixtures value) => value.writeToBuffer()));
  }

  $async.Future<$0.Fixtures> getFixtures_Pre($grpc.ServiceCall call,
      $async.Future<$0.GetFixturesRequest> request) async {
    return getFixtures(call, await request);
  }

  $async.Future<$0.FixtureDefinitions> getFixtureDefinitions_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.GetFixtureDefinitionsRequest> request) async {
    return getFixtureDefinitions(call, await request);
  }

  $async.Future<$0.Fixtures> addFixtures_Pre($grpc.ServiceCall call,
      $async.Future<$0.AddFixturesRequest> request) async {
    return addFixtures(call, await request);
  }

  $async.Future<$0.Fixtures> writeFixtureChannel_Pre($grpc.ServiceCall call,
      $async.Future<$0.WriteFixtureChannelRequest> request) async {
    return writeFixtureChannel(call, await request);
  }

  $async.Future<$0.Fixtures> getFixtures(
      $grpc.ServiceCall call, $0.GetFixturesRequest request);
  $async.Future<$0.FixtureDefinitions> getFixtureDefinitions(
      $grpc.ServiceCall call, $0.GetFixtureDefinitionsRequest request);
  $async.Future<$0.Fixtures> addFixtures(
      $grpc.ServiceCall call, $0.AddFixturesRequest request);
  $async.Future<$0.Fixtures> writeFixtureChannel(
      $grpc.ServiceCall call, $0.WriteFixtureChannelRequest request);
}
