import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/widgets.dart';

import 'decoration.dart';

class FaderInput extends StatefulWidget {
  final Function(double) onValue;
  final double value;
  final String label;
  final Gradient gradient;
  final Color color;

  FaderInput({this.onValue, this.value, this.label, this.color, this.gradient});

  @override
  _FaderInputState createState() => _FaderInputState(value ?? 0);
}

class _FaderInputState extends State<FaderInput> {
  double value = 0;

  _FaderInputState(this.value);

  @override
  Widget build(BuildContext context) {
    double y = 1 - (this.value * 2);
    var percentage = (value * 100).toStringAsFixed(1);
    return LayoutBuilder(
      builder: (context, constraints) =>
          Listener(
            onPointerSignal: (event) {
              if (event is PointerScrollEvent) {
                _onScroll(event.scrollDelta.direction);
              }
            },
            child: GestureDetector(
              onVerticalDragUpdate: (update) => _onInput(constraints, update.localPosition),
              onTapDown: (update) => _onInput(constraints, update.localPosition),
              child: Container(
                decoration: ControlDecoration(gradient: widget.gradient, color: widget.color),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    if (widget.label != null)
                      Container(
                          color: Colors.grey.shade800,
                          padding: const EdgeInsets.symmetric(vertical: 4, horizontal: 2),
                          child: Text(widget.label, textAlign: TextAlign.center)),
                    Expanded(
                      child: MouseRegion(
                        cursor: SystemMouseCursors.click,
                        child: Align(
                            alignment: AlignmentDirectional(0, y),
                            child: Container(
                                color: Colors.grey.shade800,
                                alignment: AlignmentDirectional.center,
                                constraints: BoxConstraints.expand(height: 32),
                                child: Text("$percentage%", textAlign: TextAlign.center))),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
    );
  }

  void _onInput(BoxConstraints constraints, Offset position) {
    double _value = (1.0 - position.dy / constraints.maxHeight).clamp(0.0, 1.0);
    setState(() {
      this.value = _value;
    });
    this.widget.onValue(_value);
  }

  void _onScroll(double direction) {
    double _value = value;
    if (direction < 0) {
      _value += 0.1;
    } else {
      _value -= 0.1;
    }
    _value = _value.clamp(0.0, 1.0);
    setState(() {
      this.value = _value;
    });
    this.widget.onValue(_value);
  }
}
