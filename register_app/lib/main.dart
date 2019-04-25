import 'package:flutter/material.dart';
import 'package:register_app/pages/HomePage.dart';
import 'package:register_app/pages/LandingPage.dart';
import 'package:register_app/pages/RegisterPage.dart';

void main() => runApp(MyApp());

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'JustGo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      initialRoute: '/',
      onGenerateRoute: (settings) {
        switch (settings.name) {
          case '/':
            return MaterialPageRoute(builder: (_) => LandingPage());
            break;
          case '/home':
            return MaterialPageRoute(builder: (_) => HomePage());
            break;
          case '/register':
            final uuid = settings.arguments;
            return MaterialPageRoute(builder: (_) => RegisterPage(uuid: uuid));
            break;
          default:
            return null;
        }
      },
    );
  }
}
