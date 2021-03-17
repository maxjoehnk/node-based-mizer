///
//  Generated code. Do not modify.
//  source: nodes.proto
//
// @dart = 2.7
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'nodes.pbenum.dart';

export 'nodes.pbenum.dart';

class AddNodeRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AddNodeRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'mizer'), createEmptyInstance: create)
    ..e<Node_NodeType>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'type', $pb.PbFieldType.OE, defaultOrMaker: Node_NodeType.Fader, valueOf: Node_NodeType.valueOf, enumValues: Node_NodeType.values)
    ..aOM<NodePosition>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'position', subBuilder: NodePosition.create)
    ..hasRequiredFields = false
  ;

  AddNodeRequest._() : super();
  factory AddNodeRequest({
    Node_NodeType type,
    NodePosition position,
  }) {
    final _result = create();
    if (type != null) {
      _result.type = type;
    }
    if (position != null) {
      _result.position = position;
    }
    return _result;
  }
  factory AddNodeRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AddNodeRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AddNodeRequest clone() => AddNodeRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AddNodeRequest copyWith(void Function(AddNodeRequest) updates) => super.copyWith((message) => updates(message as AddNodeRequest)); // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AddNodeRequest create() => AddNodeRequest._();
  AddNodeRequest createEmptyInstance() => create();
  static $pb.PbList<AddNodeRequest> createRepeated() => $pb.PbList<AddNodeRequest>();
  @$core.pragma('dart2js:noInline')
  static AddNodeRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AddNodeRequest>(create);
  static AddNodeRequest _defaultInstance;

  @$pb.TagNumber(1)
  Node_NodeType get type => $_getN(0);
  @$pb.TagNumber(1)
  set type(Node_NodeType v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasType() => $_has(0);
  @$pb.TagNumber(1)
  void clearType() => clearField(1);

  @$pb.TagNumber(2)
  NodePosition get position => $_getN(1);
  @$pb.TagNumber(2)
  set position(NodePosition v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasPosition() => $_has(1);
  @$pb.TagNumber(2)
  void clearPosition() => clearField(2);
  @$pb.TagNumber(2)
  NodePosition ensurePosition() => $_ensure(1);
}

class NodesRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NodesRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'mizer'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  NodesRequest._() : super();
  factory NodesRequest() => create();
  factory NodesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NodesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NodesRequest clone() => NodesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NodesRequest copyWith(void Function(NodesRequest) updates) => super.copyWith((message) => updates(message as NodesRequest)); // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NodesRequest create() => NodesRequest._();
  NodesRequest createEmptyInstance() => create();
  static $pb.PbList<NodesRequest> createRepeated() => $pb.PbList<NodesRequest>();
  @$core.pragma('dart2js:noInline')
  static NodesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NodesRequest>(create);
  static NodesRequest _defaultInstance;
}

class WriteControl extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'WriteControl', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'mizer'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'path')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'port')
    ..a<$core.double>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'value', $pb.PbFieldType.OD)
    ..hasRequiredFields = false
  ;

  WriteControl._() : super();
  factory WriteControl({
    $core.String path,
    $core.String port,
    $core.double value,
  }) {
    final _result = create();
    if (path != null) {
      _result.path = path;
    }
    if (port != null) {
      _result.port = port;
    }
    if (value != null) {
      _result.value = value;
    }
    return _result;
  }
  factory WriteControl.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory WriteControl.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  WriteControl clone() => WriteControl()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  WriteControl copyWith(void Function(WriteControl) updates) => super.copyWith((message) => updates(message as WriteControl)); // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static WriteControl create() => WriteControl._();
  WriteControl createEmptyInstance() => create();
  static $pb.PbList<WriteControl> createRepeated() => $pb.PbList<WriteControl>();
  @$core.pragma('dart2js:noInline')
  static WriteControl getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<WriteControl>(create);
  static WriteControl _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get path => $_getSZ(0);
  @$pb.TagNumber(1)
  set path($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasPath() => $_has(0);
  @$pb.TagNumber(1)
  void clearPath() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get port => $_getSZ(1);
  @$pb.TagNumber(2)
  set port($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasPort() => $_has(1);
  @$pb.TagNumber(2)
  void clearPort() => clearField(2);

  @$pb.TagNumber(3)
  $core.double get value => $_getN(2);
  @$pb.TagNumber(3)
  set value($core.double v) { $_setDouble(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasValue() => $_has(2);
  @$pb.TagNumber(3)
  void clearValue() => clearField(3);
}

class WriteResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'WriteResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'mizer'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  WriteResponse._() : super();
  factory WriteResponse() => create();
  factory WriteResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory WriteResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  WriteResponse clone() => WriteResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  WriteResponse copyWith(void Function(WriteResponse) updates) => super.copyWith((message) => updates(message as WriteResponse)); // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static WriteResponse create() => WriteResponse._();
  WriteResponse createEmptyInstance() => create();
  static $pb.PbList<WriteResponse> createRepeated() => $pb.PbList<WriteResponse>();
  @$core.pragma('dart2js:noInline')
  static WriteResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<WriteResponse>(create);
  static WriteResponse _defaultInstance;
}

class Nodes extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Nodes', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'mizer'), createEmptyInstance: create)
    ..pc<Node>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nodes', $pb.PbFieldType.PM, subBuilder: Node.create)
    ..pc<NodeConnection>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'channels', $pb.PbFieldType.PM, subBuilder: NodeConnection.create)
    ..hasRequiredFields = false
  ;

  Nodes._() : super();
  factory Nodes({
    $core.Iterable<Node> nodes,
    $core.Iterable<NodeConnection> channels,
  }) {
    final _result = create();
    if (nodes != null) {
      _result.nodes.addAll(nodes);
    }
    if (channels != null) {
      _result.channels.addAll(channels);
    }
    return _result;
  }
  factory Nodes.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Nodes.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Nodes clone() => Nodes()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Nodes copyWith(void Function(Nodes) updates) => super.copyWith((message) => updates(message as Nodes)); // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Nodes create() => Nodes._();
  Nodes createEmptyInstance() => create();
  static $pb.PbList<Nodes> createRepeated() => $pb.PbList<Nodes>();
  @$core.pragma('dart2js:noInline')
  static Nodes getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Nodes>(create);
  static Nodes _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<Node> get nodes => $_getList(0);

  @$pb.TagNumber(2)
  $core.List<NodeConnection> get channels => $_getList(1);
}

class NodeConnection extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NodeConnection', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'mizer'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'targetNode', protoName: 'targetNode')
    ..aOM<Port>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'targetPort', protoName: 'targetPort', subBuilder: Port.create)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sourceNode', protoName: 'sourceNode')
    ..aOM<Port>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sourcePort', protoName: 'sourcePort', subBuilder: Port.create)
    ..e<ChannelProtocol>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'protocol', $pb.PbFieldType.OE, defaultOrMaker: ChannelProtocol.Single, valueOf: ChannelProtocol.valueOf, enumValues: ChannelProtocol.values)
    ..hasRequiredFields = false
  ;

  NodeConnection._() : super();
  factory NodeConnection({
    $core.String targetNode,
    Port targetPort,
    $core.String sourceNode,
    Port sourcePort,
    ChannelProtocol protocol,
  }) {
    final _result = create();
    if (targetNode != null) {
      _result.targetNode = targetNode;
    }
    if (targetPort != null) {
      _result.targetPort = targetPort;
    }
    if (sourceNode != null) {
      _result.sourceNode = sourceNode;
    }
    if (sourcePort != null) {
      _result.sourcePort = sourcePort;
    }
    if (protocol != null) {
      _result.protocol = protocol;
    }
    return _result;
  }
  factory NodeConnection.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NodeConnection.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NodeConnection clone() => NodeConnection()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NodeConnection copyWith(void Function(NodeConnection) updates) => super.copyWith((message) => updates(message as NodeConnection)); // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NodeConnection create() => NodeConnection._();
  NodeConnection createEmptyInstance() => create();
  static $pb.PbList<NodeConnection> createRepeated() => $pb.PbList<NodeConnection>();
  @$core.pragma('dart2js:noInline')
  static NodeConnection getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NodeConnection>(create);
  static NodeConnection _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get targetNode => $_getSZ(0);
  @$pb.TagNumber(1)
  set targetNode($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTargetNode() => $_has(0);
  @$pb.TagNumber(1)
  void clearTargetNode() => clearField(1);

  @$pb.TagNumber(2)
  Port get targetPort => $_getN(1);
  @$pb.TagNumber(2)
  set targetPort(Port v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasTargetPort() => $_has(1);
  @$pb.TagNumber(2)
  void clearTargetPort() => clearField(2);
  @$pb.TagNumber(2)
  Port ensureTargetPort() => $_ensure(1);

  @$pb.TagNumber(3)
  $core.String get sourceNode => $_getSZ(2);
  @$pb.TagNumber(3)
  set sourceNode($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasSourceNode() => $_has(2);
  @$pb.TagNumber(3)
  void clearSourceNode() => clearField(3);

  @$pb.TagNumber(4)
  Port get sourcePort => $_getN(3);
  @$pb.TagNumber(4)
  set sourcePort(Port v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasSourcePort() => $_has(3);
  @$pb.TagNumber(4)
  void clearSourcePort() => clearField(4);
  @$pb.TagNumber(4)
  Port ensureSourcePort() => $_ensure(3);

  @$pb.TagNumber(5)
  ChannelProtocol get protocol => $_getN(4);
  @$pb.TagNumber(5)
  set protocol(ChannelProtocol v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasProtocol() => $_has(4);
  @$pb.TagNumber(5)
  void clearProtocol() => clearField(5);
}

class Node extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Node', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'mizer'), createEmptyInstance: create)
    ..e<Node_NodeType>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'type', $pb.PbFieldType.OE, defaultOrMaker: Node_NodeType.Fader, valueOf: Node_NodeType.valueOf, enumValues: Node_NodeType.values)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'path')
    ..pc<Port>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'inputs', $pb.PbFieldType.PM, subBuilder: Port.create)
    ..pc<Port>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'outputs', $pb.PbFieldType.PM, subBuilder: Port.create)
    ..aOM<NodeDesigner>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'designer', subBuilder: NodeDesigner.create)
    ..hasRequiredFields = false
  ;

  Node._() : super();
  factory Node({
    Node_NodeType type,
    $core.String path,
    $core.Iterable<Port> inputs,
    $core.Iterable<Port> outputs,
    NodeDesigner designer,
  }) {
    final _result = create();
    if (type != null) {
      _result.type = type;
    }
    if (path != null) {
      _result.path = path;
    }
    if (inputs != null) {
      _result.inputs.addAll(inputs);
    }
    if (outputs != null) {
      _result.outputs.addAll(outputs);
    }
    if (designer != null) {
      _result.designer = designer;
    }
    return _result;
  }
  factory Node.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Node.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Node clone() => Node()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Node copyWith(void Function(Node) updates) => super.copyWith((message) => updates(message as Node)); // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Node create() => Node._();
  Node createEmptyInstance() => create();
  static $pb.PbList<Node> createRepeated() => $pb.PbList<Node>();
  @$core.pragma('dart2js:noInline')
  static Node getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Node>(create);
  static Node _defaultInstance;

  @$pb.TagNumber(1)
  Node_NodeType get type => $_getN(0);
  @$pb.TagNumber(1)
  set type(Node_NodeType v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasType() => $_has(0);
  @$pb.TagNumber(1)
  void clearType() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get path => $_getSZ(1);
  @$pb.TagNumber(2)
  set path($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasPath() => $_has(1);
  @$pb.TagNumber(2)
  void clearPath() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<Port> get inputs => $_getList(2);

  @$pb.TagNumber(4)
  $core.List<Port> get outputs => $_getList(3);

  @$pb.TagNumber(5)
  NodeDesigner get designer => $_getN(4);
  @$pb.TagNumber(5)
  set designer(NodeDesigner v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasDesigner() => $_has(4);
  @$pb.TagNumber(5)
  void clearDesigner() => clearField(5);
  @$pb.TagNumber(5)
  NodeDesigner ensureDesigner() => $_ensure(4);
}

class NodePosition extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NodePosition', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'mizer'), createEmptyInstance: create)
    ..a<$core.double>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'x', $pb.PbFieldType.OD)
    ..a<$core.double>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'y', $pb.PbFieldType.OD)
    ..hasRequiredFields = false
  ;

  NodePosition._() : super();
  factory NodePosition({
    $core.double x,
    $core.double y,
  }) {
    final _result = create();
    if (x != null) {
      _result.x = x;
    }
    if (y != null) {
      _result.y = y;
    }
    return _result;
  }
  factory NodePosition.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NodePosition.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NodePosition clone() => NodePosition()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NodePosition copyWith(void Function(NodePosition) updates) => super.copyWith((message) => updates(message as NodePosition)); // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NodePosition create() => NodePosition._();
  NodePosition createEmptyInstance() => create();
  static $pb.PbList<NodePosition> createRepeated() => $pb.PbList<NodePosition>();
  @$core.pragma('dart2js:noInline')
  static NodePosition getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NodePosition>(create);
  static NodePosition _defaultInstance;

  @$pb.TagNumber(1)
  $core.double get x => $_getN(0);
  @$pb.TagNumber(1)
  set x($core.double v) { $_setDouble(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasX() => $_has(0);
  @$pb.TagNumber(1)
  void clearX() => clearField(1);

  @$pb.TagNumber(2)
  $core.double get y => $_getN(1);
  @$pb.TagNumber(2)
  set y($core.double v) { $_setDouble(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasY() => $_has(1);
  @$pb.TagNumber(2)
  void clearY() => clearField(2);
}

class NodeDesigner extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'NodeDesigner', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'mizer'), createEmptyInstance: create)
    ..aOM<NodePosition>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'position', subBuilder: NodePosition.create)
    ..a<$core.double>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'scale', $pb.PbFieldType.OD)
    ..hasRequiredFields = false
  ;

  NodeDesigner._() : super();
  factory NodeDesigner({
    NodePosition position,
    $core.double scale,
  }) {
    final _result = create();
    if (position != null) {
      _result.position = position;
    }
    if (scale != null) {
      _result.scale = scale;
    }
    return _result;
  }
  factory NodeDesigner.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NodeDesigner.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NodeDesigner clone() => NodeDesigner()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NodeDesigner copyWith(void Function(NodeDesigner) updates) => super.copyWith((message) => updates(message as NodeDesigner)); // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static NodeDesigner create() => NodeDesigner._();
  NodeDesigner createEmptyInstance() => create();
  static $pb.PbList<NodeDesigner> createRepeated() => $pb.PbList<NodeDesigner>();
  @$core.pragma('dart2js:noInline')
  static NodeDesigner getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NodeDesigner>(create);
  static NodeDesigner _defaultInstance;

  @$pb.TagNumber(1)
  NodePosition get position => $_getN(0);
  @$pb.TagNumber(1)
  set position(NodePosition v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasPosition() => $_has(0);
  @$pb.TagNumber(1)
  void clearPosition() => clearField(1);
  @$pb.TagNumber(1)
  NodePosition ensurePosition() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.double get scale => $_getN(1);
  @$pb.TagNumber(2)
  set scale($core.double v) { $_setDouble(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasScale() => $_has(1);
  @$pb.TagNumber(2)
  void clearScale() => clearField(2);
}

class Port extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Port', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'mizer'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..e<ChannelProtocol>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'protocol', $pb.PbFieldType.OE, defaultOrMaker: ChannelProtocol.Single, valueOf: ChannelProtocol.valueOf, enumValues: ChannelProtocol.values)
    ..hasRequiredFields = false
  ;

  Port._() : super();
  factory Port({
    $core.String name,
    ChannelProtocol protocol,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (protocol != null) {
      _result.protocol = protocol;
    }
    return _result;
  }
  factory Port.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Port.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Port clone() => Port()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Port copyWith(void Function(Port) updates) => super.copyWith((message) => updates(message as Port)); // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Port create() => Port._();
  Port createEmptyInstance() => create();
  static $pb.PbList<Port> createRepeated() => $pb.PbList<Port>();
  @$core.pragma('dart2js:noInline')
  static Port getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Port>(create);
  static Port _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => clearField(1);

  @$pb.TagNumber(2)
  ChannelProtocol get protocol => $_getN(1);
  @$pb.TagNumber(2)
  set protocol(ChannelProtocol v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasProtocol() => $_has(1);
  @$pb.TagNumber(2)
  void clearProtocol() => clearField(2);
}

