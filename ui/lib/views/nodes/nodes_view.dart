import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mizer/protos/nodes.pb.dart';
import 'package:mizer/state/nodes_bloc.dart';
import 'package:mizer/views/layout/layout_view.dart';
import 'package:mizer/views/nodes/widgets/editor_layers/add_node_layer.dart';
import 'package:provider/provider.dart';

import 'widgets/editor_layers/canvas_drop_layer.dart';
import 'widgets/editor_layers/graph_paint_layer.dart';
import 'models/node_editor_model.dart';
import 'widgets/editor_layers/nodes_layer.dart';
import 'widgets/editor_layers/background_layer.dart';
import 'widgets/properties/node_properties.dart';
import 'widgets/transport/transport_controls.dart';

class FetchNodesView extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    context.read<NodesBloc>().add(FetchNodes());
    return BlocBuilder<NodesBloc, Nodes>(builder: (context, nodes) {
      return SizeChangedLayoutNotifier(
        child: ChangeNotifierProvider<NodeEditorModel>.value(
            value: NodeEditorModel(nodes), // TODO: update existing NodeModels
            builder: (context, _) => SizedBox.expand(child: NodesView())),
      );
    });
  }
}

class NodesView extends StatefulWidget {
  @override
  _NodesViewState createState() => _NodesViewState();
}

class _NodesViewState extends State<NodesView> with WidgetsBindingObserver {
  Offset addMenuPosition;

  @override
  Widget build(BuildContext context) {
    WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
      afterLayout(context);
    });

    return Consumer<NodeEditorModel>(
      builder: (context, model, _) => Stack(
        children: [
          GestureDetector(
            onSecondaryTapUp: (event) {
              setState(() {
                addMenuPosition = event.localPosition;
              });
              log("right click");
            },
            onTap: () {
              setState(() {
                addMenuPosition = null;
              });
            },
            child: Stack(children: [
              SizedBox.expand(
                  child: InteractiveViewer(
                      transformationController: model.transformationController,
                      boundaryMargin: EdgeInsets.all(double.infinity),
                      minScale: 0.1,
                      maxScale: 10.0,
                      child: SizedBox.expand())),
              // Transform(
              //   transform: model.transformationController.value,
              //   child: IgnorePointer(child: CanvasBackgroundLayer(child: SizedBox.expand())),
              // ),
              Transform(
                  transform: model.transformationController.value,
                  child: IgnorePointer(
                      child: GraphPaintLayer(model: model))),
              CanvasDropLayer(),
              NodesTarget(),
              if (addMenuPosition != null)
                AddNodeMenu(addMenuPosition, onSelection: (nodeType) => _addNode(model, nodeType)),
            ]),
          ),
          Positioned(
              top: 16,
              right: 16,
              bottom: 16 + TRANSPORT_CONTROLS_HEIGHT,
              width: 256,
              child: NodePropertiesPane(node: model.selectedNode?.node)),
          Positioned(left: 0, right: 0, bottom: 0, child: TransportControls()),
        ],
      ),
    );
  }

  @override
  void initState() {
    WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
      WidgetsBinding.instance.addObserver(this);
      afterLayout(context);
    });
    super.initState();
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }

  void afterLayout(BuildContext context) {
    NodeEditorModel model = Provider.of<NodeEditorModel>(context, listen: false);
    model.updateNodes();
  }

  @override
  void didChangeMetrics() {
    super.didChangeMetrics();
    setState(() {});
  }

  void _addNode(NodeEditorModel model, Node_NodeType nodeType) {
    var nodes = context.read<NodesBloc>();
    var transformedPosition = model.transformationController.toScene(addMenuPosition);
    var position = transformedPosition / MULTIPLIER;
    nodes.add(AddNode(nodeType: nodeType, position: position));
    setState(() {
      addMenuPosition = null;
    });
  }
}
