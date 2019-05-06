import 'dart:async';
import 'package:register_app/blocs/BaseBloc.dart';
import 'package:register_app/models/User.dart';

class UserBloc extends BaseBloc {
  final userController = StreamController<User>();

  Stream<User> get user => userController.stream;

  @override
  void dispose() {
    userController.close();
  }
}