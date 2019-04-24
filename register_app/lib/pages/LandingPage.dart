import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';

class LandingPage extends StatelessWidget {
  BuildContext _ctx;

  void _handleScanQRCodeButtonPressed() {
    // Changes out the current router with a new root route
    Navigator.pushNamedAndRemoveUntil(_ctx, '/home', (_) => false);
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
                      child: Text(
                        "OF",
                        textAlign: TextAlign.center
                      ),
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
