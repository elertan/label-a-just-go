import 'package:mobx/mobx.dart';
import 'package:register_app/models/User.dart';

part 'UserStore.g.dart';

class UserStore = UserStoreBase with _$UserStore;

abstract class UserStoreBase implements Store {
  @observable
  User user;

  @action
  void setUser(User user) {
    this.user = user;
  }
}

final userStore = UserStore();