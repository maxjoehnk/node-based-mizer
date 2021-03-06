import 'package:flutter/widgets.dart';

class OpenViewIntent extends Intent {
  final View view;

  const OpenViewIntent(this.view);
}

enum View {
  Layout,
  Plan2D,
  PreViz,
  Nodes,
  Sequences,
  Fixtures,
  Media,
  Connections,
  Session,
}

class OpenProjectIntent extends Intent {
}
