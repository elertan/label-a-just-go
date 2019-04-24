import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';

class HomePage extends StatelessWidget {
  BuildContext _ctx;

  void _handleUitschrijvenPressed() {
    showDialog(
      context: _ctx,
      builder: (BuildContext context) {
        return AlertDialog(
          title: Text("Uitschrijven?"),
          content: Text("Bij het uitschrijven gaan alle geregistreerde afbeeldingen verloren en is het niet mogelijk om het evenement te bezoeken zonder nogmaals te registeren."),
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
              onPressed: () {
                // TODO: Add uitschrijven actie

                Navigator.of(context).pop();
              },
            )
          ],
        );
      }
    );
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
              child: Text(
                "Hi Dennis, je bent geregistreerd!",
                textAlign: TextAlign.center,
                style: TextStyle(
                  fontSize: 18
                ),
              ),
            ),
            Expanded(
              child: Text("Content"),
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