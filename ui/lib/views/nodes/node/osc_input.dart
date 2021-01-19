import 'package:flutter/material.dart';
import 'package:ui/protos/nodes.pb.dart';

class OscInputNode extends StatelessWidget {
  final Node node;

  OscInputNode(this.node);

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        TextField(
          decoration: InputDecoration(
              labelText: "Path"
          ),
        ),
        TextField(
          decoration: InputDecoration(
            labelText: "Port"
          ),
        )
      ],
    );
  }
}