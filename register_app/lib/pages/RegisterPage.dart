import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:mobx/mobx.dart';
import 'package:register_app/main.dart';
import 'package:register_app/models/User.dart';
import 'package:register_app/stores/UserStore.dart';
import 'package:register_app/ui/FaceRecorder.dart';

class RegisterPage extends StatefulWidget {
  final String uuid;

  RegisterPage({@required this.uuid});

  @override
  State<StatefulWidget> createState() => _RegisterPageState(uuid: uuid);
}

class _RegisterPageState extends State<RegisterPage> {
  final String uuid;

  _RegisterPageState({@required this.uuid});

  BuildContext _ctx;
  bool _isLoading = true;

  @override
  void initState() {
    super.initState();

    _fetchData();
  }

  void _fetchData() async {
//    final response = await get('http://192.168.0.100/registration-details/$uuid');
//    if (response.)

    await Future.delayed(Duration(milliseconds: 1500));

    final user = User(uuid: uuid, firstname: 'Dennis', lastname: 'Kievits');

    final dispose = reaction((_) => userStore.user, (_) {
      setState(() {
        _isLoading = false;
      });
    });
    userStore.setUser(user);
    dispose();
  }

  void _handleRegistrerenPressed() async {
    await Navigator.pushNamedAndRemoveUntil(_ctx, '/home', (_) => false);
  }

  Widget _renderLoading() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: <Widget>[
        Expanded(
          child: Center(child: CircularProgressIndicator()),
        ),
        Container(
          margin: EdgeInsets.symmetric(vertical: 50),
          child: Text(
            "Gegevens ophalen...",
            textAlign: TextAlign.center,
            style: TextStyle(fontSize: 16),
          ),
        )
      ],
    );
  }

  Widget _renderContent() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: <Widget>[
        Container(
          color: Colors.grey,
          padding: EdgeInsets.symmetric(vertical: 20),
          child: Observer(
            builder: (_) => Text(
                  "${userStore.user.firstname} ${userStore.user.lastname}",
                  textAlign: TextAlign.center,
                  style: TextStyle(fontSize: 21, color: Colors.white),
                ),
          ),
        ),
        Container(
          margin: EdgeInsets.symmetric(horizontal: 30, vertical: 10),
          child: Text(
            "Voor het registratie proces hebben we een minimaal 10 foto's nodig van uw gezicht om u zo eenvoudig mogelijk toegang te kunnen bieden tot het evenement.",
            textAlign: TextAlign.justify,
            style: TextStyle(fontSize: 14),
          ),
        ),
        Expanded(
          child: Container(
            margin: EdgeInsets.symmetric(vertical: 10, horizontal: 10),
            child: FaceRecorder(
              cameras: cameras,
            ),
          ),
        ),
        Container(
          margin: EdgeInsets.symmetric(horizontal: 50),
          child: RaisedButton(
            padding: EdgeInsets.symmetric(vertical: 15),
            child: Text(
              "Registreren",
              style: TextStyle(fontSize: 16),
            ),
            onPressed: _handleRegistrerenPressed,
          ),
        ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    _ctx = context;

    return WillPopScope(
      onWillPop: () async => !_isLoading,
      child: Scaffold(
        appBar: AppBar(
          title: Text("Registreren"),
        ),
        body: SafeArea(
          child: _isLoading ? _renderLoading() : _renderContent(),
        ),
      ),
    );
  }
}
