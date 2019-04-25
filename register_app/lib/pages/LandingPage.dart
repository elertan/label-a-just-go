import 'package:barcode_scan/barcode_scan.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';
import 'package:mobx/mobx.dart';
import 'package:register_app/models/User.dart';
import 'package:register_app/stores/UserStore.dart';
import 'package:register_app/utils/validation.dart';

class LandingPage extends StatelessWidget {
  BuildContext _ctx;

  void _handleScanQRCodeButtonPressed() async {
    String barcode = null;
    try {
      barcode = await BarcodeScanner.scan();
    } on PlatformException catch (e) {
      if (e.code == BarcodeScanner.CameraAccessDenied) {
        await showDialog(
            context: _ctx,
            builder: (BuildContext context) {
              return AlertDialog(
                title: Text("Camera toegang niet toegestaan"),
                content: Text(
                    "Het is niet mogelijk om een QR Code te scannen, omdat er geen toegang tot de camera wordt verleent voor deze app."),
                actions: <Widget>[
                  FlatButton(
                    child: Text("Ok"),
                    onPressed: () {
                      Navigator.of(context).pop();
                    },
                  ),
                ],
              );
            });
      } else {
        print("Unknown error occured trying to access camera: $e");
      }
    }

    if (barcode == null) {
      return;
    }

    if (!isValidUuidv4(barcode)) {
      await showDialog(
          context: _ctx,
          builder: (BuildContext context) {
            return AlertDialog(
              title: Text("Onjuist formaat"),
              content:
                  Text("De gescande QR Code is niet in het juiste formaat."),
              actions: <Widget>[
                FlatButton(
                  child: Text("Ok"),
                  onPressed: () {
                    Navigator.of(context).pop();
                  },
                ),
              ],
            );
          });
      return;
    }
    Navigator.pushNamed(_ctx, '/register', arguments: barcode);

//    final user = User(uuid: barcode, firstname: 'Dennis', lastname: 'Kievits');
//
//    final dispose = reaction((_) => userStore.user, (_) async {
//      // Changes out the current router with a new root route
//      await Navigator.pushNamedAndRemoveUntil(_ctx, '/home', (_) => false);
//    });
//    userStore.setUser(user);
//    dispose();
  }

  @override
  Widget build(BuildContext context) {
    _ctx = context;

    return Scaffold(
      body: SafeArea(
        child: Column(
          children: <Widget>[
            Container(
              margin: EdgeInsets.only(top: 50),
              child: Center(
                child: Text(
                  "Just Go",
                  textAlign: TextAlign.center,
                  style: TextStyle(fontSize: 36),
                ),
              ),
            ),
            Expanded(
              child: Container(
                padding: EdgeInsets.symmetric(vertical: 50, horizontal: 50),
                child: Center(
                  child: Text(
                    "Naar evenementen gaan was nog nóóit zo makkelijk!",
                    textAlign: TextAlign.center,
                    style: TextStyle(
                      fontSize: 16,
                    ),
                  ),
                ),
              ),
            ),
            Container(
              margin: EdgeInsets.symmetric(horizontal: 50),
              child: Center(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: <Widget>[
                    RaisedButton(
                      padding: EdgeInsets.symmetric(vertical: 15),
                      child: Text(
                        "Scan QR Code",
                        style: TextStyle(fontSize: 16),
                      ),
                      onPressed: _handleScanQRCodeButtonPressed,
                    ),
                    Container(
                      margin: EdgeInsets.symmetric(vertical: 10),
                      child: Text("OF", textAlign: TextAlign.center),
                    ),
                    Text(
                      "Bezoek de uitnodigingslink via deze telefoon",
                      textAlign: TextAlign.center,
                    )
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
