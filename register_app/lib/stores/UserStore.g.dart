// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'UserStore.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars

mixin _$UserStore on UserStoreBase, Store {
  final _$userAtom = Atom(name: 'UserStoreBase.user');

  @override
  User get user {
    _$userAtom.reportObserved();
    return super.user;
  }

  @override
  set user(User value) {
    _$userAtom.context.checkIfStateModificationsAreAllowed(_$userAtom);
    super.user = value;
    _$userAtom.reportChanged();
  }

  final _$UserStoreBaseActionController =
      ActionController(name: 'UserStoreBase');

  @override
  void setUser(User user) {
    final _$actionInfo = _$UserStoreBaseActionController.startAction();
    try {
      return super.setUser(user);
    } finally {
      _$UserStoreBaseActionController.endAction(_$actionInfo);
    }
  }
}
