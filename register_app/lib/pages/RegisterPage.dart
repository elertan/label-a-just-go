import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';

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
    await Future.delayed(Duration(milliseconds: 1500));
    setState(() {
      _isLoading = false;
    });
  }

  Widget _renderLoading() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: <Widget>[
        Expanded(
          child: Center(
            child: CircularProgressIndicator()
          ),
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
    return Text("Content");
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
