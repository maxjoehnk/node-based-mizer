import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:mizer/protos/nodes.pb.dart';
import 'package:mizer/views/nodes/properties/property_group.dart';

class VideoFileProperties extends StatelessWidget {
  final VideoFileNodeConfig config;

  VideoFileProperties(this.config);

  @override
  Widget build(BuildContext context) {
    return PropertyGroup(
      title: "Video File",
      children: [],
    );
  }
}