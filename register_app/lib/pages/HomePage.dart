import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:register_app/stores/UserStore.dart';

class HomePage extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  BuildContext _ctx;

  void _handleUitschrijvenPressed() async {
    await showDialog(
        context: _ctx,
        builder: (BuildContext context) {
          return AlertDialog(
            title: Text("Uitschrijven?"),
            content: Text(
                "Bij het uitschrijven gaan alle geregistreerde afbeeldingen verloren en is het niet mogelijk om het evenement te bezoeken zonder nogmaals te registeren."),
            actions: <Widget>[
              FlatButton(
                child: Text("Annuleren"),
                onPressed: () {
                  Navigator.of(context).pop();
                },
              ),
              FlatButton(
                child: Text(
                  "Bevestigen",
                  style: TextStyle(color: Colors.red),
                ),
                onPressed: () async {
                  // TODO: Add uitschrijven actie
                  await Navigator.pushNamedAndRemoveUntil(
                      _ctx, '/', (_) => false);
                  userStore.setUser(null);
                },
              )
            ],
          );
        });
  }

  @override
  Widget build(BuildContext context) {
    _ctx = context;

    return Scaffold(
      appBar: AppBar(
        title: Text("Just Go"),
      ),
      body: SafeArea(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: <Widget>[
            Container(
              margin: EdgeInsets.symmetric(vertical: 30),
              child: Observer(
                builder: (_) => Text(
                      "Hi ${userStore.user.firstname}, je bent geregistreerd!",
                      textAlign: TextAlign.center,
                      style: TextStyle(fontSize: 18),
                    ),
              ),
            ),
            Expanded(
              child: Observer(
                builder: (_) => Text(
                  userStore.user.uuid,
                  textAlign: TextAlign.center,
                ),
              ),
            ),
            Container(
              margin: EdgeInsets.symmetric(horizontal: 50),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: <Widget>[
                  RaisedButton(
                    child: Text(
                      "Uitschrijven",
                      style: TextStyle(color: Colors.white),
                    ),
                    color: Colors.red,
                    onPressed: _handleUitschrijvenPressed,
                  ),
                ],
              ),
            )
          ],
        ),
      ),
    );
  }
}