import 'package:mobx/mobx.dart';

part 'store.g.dart';

AppStore get appStore => AppStoreBase.instance;

class AppStore = AppStoreBase with _$AppStore;

abstract class AppStoreBase with Store {
    static AppStore instance = AppStore();
}
