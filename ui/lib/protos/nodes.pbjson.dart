///
//  Generated code. Do not modify.
//  source: nodes.proto
//
// @dart = 2.7
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

const ChannelProtocol$json = const {
  '1': 'ChannelProtocol',
  '2': const [
    const {'1': 'Single', '2': 0},
    const {'1': 'Multi', '2': 1},
    const {'1': 'Texture', '2': 2},
    const {'1': 'Vector', '2': 3},
    const {'1': 'Laser', '2': 4},
    const {'1': 'Poly', '2': 5},
    const {'1': 'Data', '2': 6},
    const {'1': 'Material', '2': 7},
    const {'1': 'Gst', '2': 8},
  ],
};

const AddNodeRequest$json = const {
  '1': 'AddNodeRequest',
  '2': const [
    const {'1': 'type', '3': 1, '4': 1, '5': 14, '6': '.mizer.Node.NodeType', '10': 'type'},
    const {'1': 'position', '3': 2, '4': 1, '5': 11, '6': '.mizer.NodePosition', '10': 'position'},
  ],
};

const NodesRequest$json = const {
  '1': 'NodesRequest',
};

const WriteControl$json = const {
  '1': 'WriteControl',
  '2': const [
    const {'1': 'path', '3': 1, '4': 1, '5': 9, '10': 'path'},
    const {'1': 'port', '3': 2, '4': 1, '5': 9, '10': 'port'},
    const {'1': 'value', '3': 3, '4': 1, '5': 1, '10': 'value'},
  ],
};

const WriteResponse$json = const {
  '1': 'WriteResponse',
};

const Nodes$json = const {
  '1': 'Nodes',
  '2': const [
    const {'1': 'nodes', '3': 1, '4': 3, '5': 11, '6': '.mizer.Node', '10': 'nodes'},
    const {'1': 'channels', '3': 2, '4': 3, '5': 11, '6': '.mizer.NodeConnection', '10': 'channels'},
  ],
};

const NodeConnection$json = const {
  '1': 'NodeConnection',
  '2': const [
    const {'1': 'targetNode', '3': 1, '4': 1, '5': 9, '10': 'targetNode'},
    const {'1': 'targetPort', '3': 2, '4': 1, '5': 11, '6': '.mizer.Port', '10': 'targetPort'},
    const {'1': 'sourceNode', '3': 3, '4': 1, '5': 9, '10': 'sourceNode'},
    const {'1': 'sourcePort', '3': 4, '4': 1, '5': 11, '6': '.mizer.Port', '10': 'sourcePort'},
    const {'1': 'protocol', '3': 5, '4': 1, '5': 14, '6': '.mizer.ChannelProtocol', '10': 'protocol'},
  ],
};

const Node$json = const {
  '1': 'Node',
  '2': const [
    const {'1': 'type', '3': 1, '4': 1, '5': 14, '6': '.mizer.Node.NodeType', '10': 'type'},
    const {'1': 'path', '3': 2, '4': 1, '5': 9, '10': 'path'},
    const {'1': 'inputs', '3': 3, '4': 3, '5': 11, '6': '.mizer.Port', '10': 'inputs'},
    const {'1': 'outputs', '3': 4, '4': 3, '5': 11, '6': '.mizer.Port', '10': 'outputs'},
    const {'1': 'designer', '3': 5, '4': 1, '5': 11, '6': '.mizer.NodeDesigner', '10': 'designer'},
  ],
  '4': const [Node_NodeType$json],
};

const Node_NodeType$json = const {
  '1': 'NodeType',
  '2': const [
    const {'1': 'Fader', '2': 0},
    const {'1': 'Button', '2': 20},
    const {'1': 'DmxOutput', '2': 1},
    const {'1': 'Oscillator', '2': 2},
    const {'1': 'Clock', '2': 3},
    const {'1': 'OscInput', '2': 4},
    const {'1': 'VideoFile', '2': 5},
    const {'1': 'VideoOutput', '2': 6},
    const {'1': 'VideoEffect', '2': 7},
    const {'1': 'VideoColorBalance', '2': 8},
    const {'1': 'VideoTransform', '2': 9},
    const {'1': 'Script', '2': 10},
    const {'1': 'PixelToDmx', '2': 11},
    const {'1': 'PixelPattern', '2': 12},
    const {'1': 'OpcOutput', '2': 13},
    const {'1': 'Fixture', '2': 14},
    const {'1': 'Sequence', '2': 15},
    const {'1': 'MidiInput', '2': 16},
    const {'1': 'MidiOutput', '2': 17},
    const {'1': 'Laser', '2': 18},
    const {'1': 'IldaFile', '2': 19},
  ],
};

const NodePosition$json = const {
  '1': 'NodePosition',
  '2': const [
    const {'1': 'x', '3': 1, '4': 1, '5': 1, '10': 'x'},
    const {'1': 'y', '3': 2, '4': 1, '5': 1, '10': 'y'},
  ],
};

const NodeDesigner$json = const {
  '1': 'NodeDesigner',
  '2': const [
    const {'1': 'position', '3': 1, '4': 1, '5': 11, '6': '.mizer.NodePosition', '10': 'position'},
    const {'1': 'scale', '3': 2, '4': 1, '5': 1, '10': 'scale'},
  ],
};

const Port$json = const {
  '1': 'Port',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'protocol', '3': 2, '4': 1, '5': 14, '6': '.mizer.ChannelProtocol', '10': 'protocol'},
  ],
};

