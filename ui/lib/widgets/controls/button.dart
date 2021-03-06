import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';

class MizerButton extends StatefulWidget {
  final Function() onClick;
  final Widget child;
  final bool active;

  MizerButton({ @required this.child, this.onClick, this.active = false });

  @override
  _MizerButtonState createState() => _MizerButtonState();
}

class _MizerButtonState extends State<MizerButton> {
  bool hovering = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onHover: (e) => setState(() => hovering = true),
      onExit: (e) => setState(() => hovering = false),
      child: GestureDetector(
        onTap: widget.onClick,
        behavior: HitTestBehavior.opaque,
        child: Container(
          decoration: BoxDecoration(
            color: _active ? Colors.grey.shade700 : null,
            borderRadius: BorderRadius.circular(2),
          ),
          margin: const EdgeInsets.all(2),
          padding: const EdgeInsets.all(2),
          child: Center(child: widget.child),
        ),
      ),
    );
  }

  bool get _active {
    return widget.active || hovering;
  }
}
