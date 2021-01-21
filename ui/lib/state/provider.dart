import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:grpc/grpc.dart';

import 'fixtures_bloc.dart';
import 'nodes_bloc.dart';
import 'session_bloc.dart';

class StateProvider extends StatelessWidget {
  final ClientChannel _channel;
  final Widget child;

  StateProvider(this._channel, {@required this.child});

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
      child: this.child,
      providers: [
        BlocProvider(create: (BuildContext context) => NodesBloc(_channel)),
        BlocProvider(create: (BuildContext context) => SessionBloc(_channel)),
        BlocProvider(create: (BuildContext context) => FixturesBloc(_channel)),
      ],
    );
  }
}